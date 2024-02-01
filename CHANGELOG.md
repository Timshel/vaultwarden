# Changelog

## 1.30.2-6

 - Add `SSO_AUDIENCE_TRUSTED` config to allow to trust additionnal audience.

## 1.30.2-5

 - Fix mysql migration `2024-02-14-170000_add_state_to_sso_nonce`

## 1.30.2-4

 - Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.1.2-6`
 - Use `openidconnect` to validate Id Token claims
 - Remove `SSO_KEY_FILEPATH` should not be useful now
 - Add `SSO_DEBUG_TOKENS` to log Id/Access/Refresh token to debug
 - Hardcoded redircetion url
 - Switch to reading the roles and groups Claims from the Id Token

## 1.30.2-3

 - Add `SSO_AUTHORIZE_EXTRA_PARAMS` to add extra parameter to the authorize redirection (needed to obtain a `refresh_token` with Google Auth).

## 1.30.2-2

 - Fix non jwt `acess_token` check when there is no `refresh_token`
 - Add `SSO_AUTH_ONLY_NOT_SESSION` to use SSO only for auth not the session lifecycle.

## 1.30.2-1

 - Update [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.1.2-4` which move the org invite patch to the `button` release (which is expected to be merged in VW).
 - Remove the `sso_acceptall_invites` setting
 - Allow to override log level for specific target

## 1.30.1-11

 - Encode redirect url parameters and add `debug` logging.

## 1.30.1-10

 - Keep old prevalidate endpoint for Mobile apps

## 1.30.1-9

 - Add non jwt access_token support

## 1.30.1-8

 - Prevalidate endpoint change in Bitwarden WebVault [web-v2024.1.2](https://github.com/bitwarden/clients/tree/web-v2024.1.2/apps/web)
 - Add support for `experimental` front-end which stop sending the Master password hash to the server
 - Fix the in docker images

## 1.30.1-7

 - Switch user invitation status to `Confirmed` on when user login not before (cf https://github.com/Timshel/vaultwarden/issues/17)
 - Return a 404 when user has no `public_key`, will prevent confirming the user in case previous fix is insufficient.

## 1.30.1-6

 - Ensure the token endpoint always return a `refresh_token` (cf https://github.com/Timshel/vaultwarden/issues/16)
