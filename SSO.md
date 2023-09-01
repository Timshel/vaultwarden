# SSO using OpenId Connect

To use an external source of authentication your SSO will need to support OpenID Connect :

 - And OpenID Connect Discovery endpoint should be available
 - Client authentication will be done using Id and Secret.

A master password will still required and not controlled by the SSO (depending of your point of view this might be a feature ;).
This introduce another way to control who can use the vault without having to use invitation or using an LDAP.

## Configuration

The following configurations are available

 - `SSO_ENABLED` : Activate the SSO
 - `SSO_ONLY` : disable email+Master password authentication
 - `SSO_AUTHORITY` : the OpendID Connect Discovery endpoint of your SSO
 	- Should not include the `/.well-known/openid-configuration` part and no trailing `/`
 	- $SSO_AUTHORITY/.well-known/openid-configuration should return the a json document: https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfigurationResponse
 - `SSO_SCOPES` : Optional, allow to override scopes if needed (default `"email profile"`)
 - `SSO_CLIENT_ID` : Client Id
 - `SSO_CLIENT_SECRET` : Client Secret
 - `SSO_AUTH_FAILURE_SILENT`: Silently redirect to the home instead of displaying a JSON error.
 - `SSO_KEY_FILEPATH` : Optional public key to validate the JWT token (without it signature check will not be done).
 - `SSO_MASTER_PASSWORD_POLICY`: Optional Master password policy

The callback url is : `https://your.domain/identity/connect/oidc-signin`

## Configuration example using GitLab

Create an application in your Gitlab Settings with

- `redirectURI`: https://your.domain/identity/connect/oidc-signin
- `Confidential`: `true`
- `scopes`: `openid`, `profile`, `email`

Then configure your server with `SSO_AUTHORITY=https://gitlab.com`, `SSO_CLIENT_ID` and `SSO_CLIENT_SECRET`.

## Configuration hints using Google

Google [Documentation](https://developers.google.com/identity/openid-connect/openid-connect).
Then configure your server with `SSO_AUTHORITY=https://accounts.google.com`, `SSO_CLIENT_ID` and `SSO_CLIENT_SECRET`.

## Microsoft Entra ID

Only the v2 endpooint is compliant with the OpenID spec.
The endpoint should be in the format: https://login.microsoftonline.com/${tenantguid}/v2.0

You should able to find it on https://entra.microsoft.com/ following `Identity | Applications | App registrations | Endpoints`.

Additionnaly you'll need to override the default scopes to add `offline_access` otherwise no refresh_token is returned ([cf](https://github.com/MicrosoftDocs/azure-docs/issues/17134)).

Configuration should look like this:

	- `SSO_AUTHORITY=https://login.microsoftonline.com/${tenantguid}/v2.0`,
	- `SSO_SCOPES="email profile offline_access"`
	- `SSO_CLIENT_ID=...`
	- `SSO_CLIENT_SECRET=...`.

Other endoints are not OpenID compliant, cf:

 - https://github.com/MicrosoftDocs/azure-docs/issues/38427
 - https://github.com/ramosbugs/openidconnect-rs/issues/122

