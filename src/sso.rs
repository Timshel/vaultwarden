use chrono::Utc;
use std::sync::RwLock;
use std::time::Duration;
use url::Url;

use jsonwebtoken::{DecodingKey, Validation};
use mini_moka::sync::Cache;
use once_cell::sync::Lazy;
use openidconnect::core::{CoreClient, CoreProviderMetadata, CoreResponseType, CoreUserInfoClaims};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessToken, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IdToken, Nonce,
    OAuth2TokenResponse, RefreshToken, Scope,
};

use crate::{
    api::ApiResult,
    auth,
    auth::{AuthMethodScope, TokenWrapper, DEFAULT_REFRESH_VALIDITY},
    db::{
        models::{Device, SsoNonce, User},
        DbConn,
    },
    CONFIG,
};

pub static COOKIE_NAME_REDIRECT: &str = "sso_redirect_url";

static AC_CACHE: Lazy<Cache<String, AuthenticatedUser>> =
    Lazy::new(|| Cache::builder().max_capacity(1000).time_to_live(Duration::from_secs(10 * 60)).build());

static CLIENT_CACHE: RwLock<Option<CoreClient>> = RwLock::new(None);

static SSO_JWT_VALIDATION: Lazy<Decoding> = Lazy::new(prepare_decoding);

static DEFAULT_BW_EXPIRATION: Lazy<chrono::Duration> = Lazy::new(|| chrono::Duration::minutes(5));

// Will Panic if SSO is activated and a key file is present but we can't decode its content
pub fn pre_load_sso_jwt_validation() {
    Lazy::force(&SSO_JWT_VALIDATION);
}

#[rocket::async_trait]
trait CoreClientExt {
    async fn _get_client() -> ApiResult<CoreClient>;
    async fn cached() -> ApiResult<CoreClient>;
    async fn user_info_async(&self, access_token: AccessToken) -> ApiResult<CoreUserInfoClaims>;
}

#[rocket::async_trait]
impl CoreClientExt for CoreClient {
    // Call the OpenId discovery endpoint to retrieve configuration
    async fn _get_client() -> ApiResult<CoreClient> {
        let client_id = ClientId::new(CONFIG.sso_client_id());
        let client_secret = ClientSecret::new(CONFIG.sso_client_secret());

        let issuer_url = CONFIG.sso_issuer_url()?;

        let provider_metadata = match CoreProviderMetadata::discover_async(issuer_url, async_http_client).await {
            Err(err) => err!(format!("Failed to discover OpenID provider: {err}")),
            Ok(metadata) => metadata,
        };

        Ok(CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
            .set_redirect_uri(CONFIG.sso_redirect_url()?))
    }

    // Simple cache to prevent recalling the discovery endpoint each time
    async fn cached() -> ApiResult<CoreClient> {
        let cc_client = CLIENT_CACHE.read().ok().and_then(|rw_lock| rw_lock.clone());
        match cc_client {
            Some(client) => Ok(client),
            None => Self::_get_client().await.map(|client| {
                let mut cached_client = CLIENT_CACHE.write().unwrap();
                *cached_client = Some(client.clone());
                client
            }),
        }
    }

    async fn user_info_async(&self, access_token: AccessToken) -> ApiResult<CoreUserInfoClaims> {
        let endpoint = match self.user_info(access_token, None) {
            Err(err) => err!(format!("No user_info endpoint: {err}")),
            Ok(endpoint) => endpoint,
        };

        match endpoint.request_async(async_http_client).await {
            Err(err) => err!(format!("Request to user_info endpoint failed: {err}")),
            Ok(user_info) => Ok(user_info),
        }
    }
}

// The `nonce` allow to protect against replay attacks
pub async fn authorize_url(mut conn: DbConn, state: String) -> ApiResult<Url> {
    let scopes = CONFIG.sso_scopes_vec().into_iter().map(Scope::new);

    let (auth_url, _csrf_state, nonce) = CoreClient::cached()
        .await?
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            || CsrfToken::new(state),
            Nonce::new_random,
        )
        .add_scopes(scopes)
        .url();

    let sso_nonce = SsoNonce::new(nonce.secret().to_string());
    sso_nonce.save(&mut conn).await?;

    Ok(auth_url)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct IdTokenPayload {
    exp: i64,
    email: Option<String>,
    nonce: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BasicTokenPayload {
    iat: Option<i64>,
    nbf: Option<i64>,
    exp: i64,
}

impl BasicTokenPayload {
    fn nbf(&self) -> i64 {
        self.nbf.or(self.iat).unwrap_or_else(|| Utc::now().naive_utc().timestamp())
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub nonce: String,
    pub refresh_token: Option<String>,
    pub access_token: String,
    pub email: String,
    pub user_name: Option<String>,
}

struct Decoding {
    key: DecodingKey,
    id_validation: Validation,
    access_validation: Validation,
    debug_key: DecodingKey,
    debug_validation: Validation,
}

impl Decoding {
    pub fn new(key: DecodingKey, validation: Validation) -> Self {
        let mut access_validation = validation.clone();
        access_validation.validate_aud = false;

        let mut debug_validation = insecure_validation();
        debug_validation.validate_aud = false;

        Decoding {
            key,
            id_validation: validation,
            access_validation,
            debug_key: DecodingKey::from_secret(&[]),
            debug_validation,
        }
    }

    pub fn decode_id_token<
        AC: openidconnect::AdditionalClaims,
        GC: openidconnect::GenderClaim,
        JE: openidconnect::JweContentEncryptionAlgorithm<JT>,
        JS: openidconnect::JwsSigningAlgorithm<JT>,
        JT: openidconnect::JsonWebKeyType,
    >(
        &self,
        oic_id_token: Option<&IdToken<AC, GC, JE, JS, JT>>,
    ) -> ApiResult<IdTokenPayload> {
        let id_token_str = match oic_id_token {
            None => err!("Token response did not contain an id_token"),
            Some(token) => token.to_string(),
        };

        match jsonwebtoken::decode::<IdTokenPayload>(id_token_str.as_str(), &self.key, &self.id_validation) {
            Ok(payload) => Ok(payload.claims),
            Err(err) => {
                self.log_decode_debug("identity_token", id_token_str.as_str());
                err!(format!("Could not decode id token: {err}"))
            }
        }
    }

    pub fn decode_basic_token(&self, token_name: &str, token: &str) -> ApiResult<BasicTokenPayload> {
        match jsonwebtoken::decode::<BasicTokenPayload>(token, &self.key, &self.access_validation) {
            Ok(payload) => Ok(payload.claims),
            Err(err) => {
                self.log_decode_debug(token_name, token);
                err_silent!(format!("Could not decode {token_name}: {err}"))
            }
        }
    }

    pub fn log_decode_debug(&self, token_name: &str, token: &str) {
        let _ = jsonwebtoken::decode::<serde_json::Value>(token, &self.debug_key, &self.debug_validation)
            .map(|payload| debug!("Token {token_name}: {}", payload.claims));
    }
}

fn insecure_validation() -> Validation {
    let mut validation = jsonwebtoken::Validation::default();
    validation.set_audience(&[CONFIG.sso_client_id()]);
    validation.insecure_disable_signature_validation();

    validation
}

// DecodingKey and Validation used to read the SSO JWT token response
// If there is no key fallback to reading without validation
fn prepare_decoding() -> Decoding {
    let maybe_key = CONFIG.sso_enabled().then_some(()).and_then(|_| match std::fs::read(CONFIG.sso_key_filepath()) {
        Ok(key) => Some(DecodingKey::from_rsa_pem(&key).unwrap_or_else(|e| {
            panic!(
                "Failed to decode optional SSO public RSA Key, format should exactly match:\n\
                -----BEGIN PUBLIC KEY-----\n\
                ...\n\
                -----END PUBLIC KEY-----\n\
                Error: {e}"
            );
        })),
        Err(err) => {
            println!("[INFO] Can't read optional SSO public key at {} : {err}", CONFIG.sso_key_filepath());
            None
        }
    });

    match maybe_key {
        Some(key) => {
            let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
            validation.leeway = 30; // 30 seconds
            validation.validate_exp = true;
            validation.validate_nbf = true;
            validation.set_audience(&[CONFIG.sso_client_id()]);
            validation.set_issuer(&[CONFIG.sso_authority()]);

            Decoding::new(key, validation)
        }
        None => Decoding::new(DecodingKey::from_secret(&[]), insecure_validation()),
    }
}

#[derive(Clone, Debug)]
pub struct UserInformation {
    pub email: String,
    pub user_name: Option<String>,
}

// During the 2FA flow we will
//  - retrieve the user information and then only discover he needs 2FA.
//  - second time we will rely on the `AC_CACHE` since the `code` has already been exchanged.
// The `nonce` will ensure that the user is authorized only once.
// We return only the `UserInformation` to force calling `redeem` to obtain the `refresh_token`.
pub async fn exchange_code(code: &String) -> ApiResult<UserInformation> {
    if let Some(authenticated_user) = AC_CACHE.get(code) {
        return Ok(UserInformation {
            email: authenticated_user.email,
            user_name: authenticated_user.user_name,
        });
    }

    let oidc_code = AuthorizationCode::new(code.clone());
    let client = CoreClient::cached().await?;

    match client.exchange_code(oidc_code).request_async(async_http_client).await {
        Ok(token_response) => {
            let user_info = client.user_info_async(token_response.access_token().to_owned()).await?;

            let id_token = SSO_JWT_VALIDATION.decode_id_token(token_response.extra_fields().id_token())?;

            let email = match id_token.email {
                Some(email) => email,
                None => match user_info.email() {
                    None => err!("Neither id token nor userinfo contained an email"),
                    Some(email) => email.to_owned().to_string(),
                },
            };

            let user_name = user_info.preferred_username().map(|un| un.to_string());

            let refresh_token = token_response.refresh_token().map(|t| t.secret().to_string());
            if refresh_token.is_none() && CONFIG.sso_scopes_vec().contains(&"offline_access".to_string()) {
                error!("Scope offline_access is present but response contain no refresh_token");
            }

            let authenticated_user = AuthenticatedUser {
                nonce: id_token.nonce,
                refresh_token,
                access_token: token_response.access_token().secret().to_string(),
                email: email.clone(),
                user_name: user_name.clone(),
            };

            AC_CACHE.insert(code.clone(), authenticated_user.clone());

            Ok(UserInformation {
                email,
                user_name,
            })
        }
        Err(err) => err!(format!("Failed to contact token endpoint: {err}")),
    }
}

// User has passed 2FA flow we can delete `nonce` and clear the cache.
pub async fn redeem(code: &String, conn: &mut DbConn) -> ApiResult<AuthenticatedUser> {
    if let Some(au) = AC_CACHE.get(code) {
        AC_CACHE.invalidate(code);

        if let Some(sso_nonce) = SsoNonce::find(&au.nonce, conn).await {
            match sso_nonce.delete(conn).await {
                Err(msg) => err!(format!("Failed to delete nonce: {msg}")),
                Ok(_) => Ok(au),
            }
        } else {
            err!("Failed to retrive nonce from db")
        }
    } else {
        err!("Failed to retrieve user info from sso cache")
    }
}

// We always return a refresh_token (with no refresh_token some secrets are not displayed in the web front).
// If there is no SSO refresh_token, we keep the access_token to be able to call user_info to check for validity
pub fn create_auth_tokens(
    device: &Device,
    user: &User,
    refresh_token: Option<String>,
    access_token: &str,
) -> ApiResult<auth::AuthTokens> {
    let access_payload = SSO_JWT_VALIDATION.decode_basic_token("access_token", access_token)?;
    debug!("Access_payload: {:?}", access_payload);

    let (nbf, exp, token) = if let Some(rt) = refresh_token.as_ref() {
        let (nbf, exp) = match SSO_JWT_VALIDATION.decode_basic_token("refresh_token", rt) {
            Err(_) => {
                let time_now = Utc::now().naive_utc();
                let exp = (time_now + *DEFAULT_REFRESH_VALIDITY).timestamp();
                debug!("Non jwt refresh_token (expiration set to {})", exp);
                (time_now.timestamp(), exp)
            }
            Ok(refresh_payload) => {
                debug!("Refresh_payload: {:?}", refresh_payload);
                (refresh_payload.nbf(), refresh_payload.exp)
            }
        };
        (nbf, exp, TokenWrapper::Refresh(rt.to_string()))
    } else {
        debug!("No refresh_token present");
        (access_payload.nbf(), access_payload.exp, TokenWrapper::Access(access_token.to_string()))
    };

    let refresh_claims = auth::RefreshJwtClaims {
        nbf,
        exp,
        iss: auth::JWT_LOGIN_ISSUER.to_string(),
        sub: auth::AuthMethod::Sso,
        device_token: device.refresh_token.clone(),
        token: Some(token),
    };

    let access_claims = auth::LoginJwtClaims::new(
        device,
        user,
        access_payload.nbf(),
        access_payload.exp,
        auth::AuthMethod::Sso.scope_vec(),
    );

    Ok(auth::AuthTokens {
        refresh_claims,
        access_claims,
    })
}

// This endpoint is called in two case
//  - the session is close to expiration we will try to extend it
//  - the user is going to make an action and we check that the session is still valid
pub async fn exchange_refresh_token(
    device: &Device,
    user: &User,
    refresh_claims: &auth::RefreshJwtClaims,
) -> ApiResult<auth::AuthTokens> {
    match &refresh_claims.token {
        Some(TokenWrapper::Refresh(refresh_token)) => {
            let rt = RefreshToken::new(refresh_token.to_string());

            let client = CoreClient::cached().await?;

            let token_response = match client.exchange_refresh_token(&rt).request_async(async_http_client).await {
                Err(err) => err!(format!("Request to exchange_refresh_token endpoint failed: {:?}", err)),
                Ok(token_response) => token_response,
            };

            // Use new refresh_token if returned
            let rolled_refresh_token = token_response
                .refresh_token()
                .map(|token| token.secret().to_string())
                .unwrap_or(refresh_token.to_string());

            create_auth_tokens(device, user, Some(rolled_refresh_token), token_response.access_token().secret())
        }
        Some(TokenWrapper::Access(access_token)) => {
            let exp_limit = (Utc::now().naive_utc() + *DEFAULT_BW_EXPIRATION).timestamp();

            match SSO_JWT_VALIDATION.decode_basic_token("access_token", access_token) {
                Err(err) => err!(format!("Impossible to read access_token: {err}")),
                Ok(claims) if claims.exp < exp_limit => {
                    err_silent!("Access token is close to expiration but we have no refresh token")
                }
                Ok(_) => {
                    let at = AccessToken::new(access_token.to_string());
                    let client = CoreClient::cached().await?;
                    match client.user_info_async(at).await {
                        Err(err) => err_silent!(format!(
                            "Failed to retrieve user info, token has probably been invalidated: {err}"
                        )),
                        Ok(_) => create_auth_tokens(device, user, None, access_token),
                    }
                }
            }
        }
        None => err!("No token present while in SSO"),
    }
}
