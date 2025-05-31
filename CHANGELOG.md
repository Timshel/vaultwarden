# Changelog

# 1.34.1-1

- Rebased on `1.34.1` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2025.5.1-2`
- :warning: Rework of the organization sync, see [README.md#organization_sync](README.md#organization_sync) :warning:
- Rewrote organization sync
  - Add support for organization role mapping
  - Add support for oganization groups sync, initially the feature will be avaible only if `ORG_GROUPS_ENABLED` and `SSO_ORGANIZATIONS_GROUPS_ENABLED` are activated.
    `SSO_ORGANIZATIONS_GROUPS_ENABLED` will be removed in subsequent release (feature will be active if `ORG_GROUPS_ENABLED` is enabled).
- :warning: multiple deprecations
  - `SSO_ORGANIZATIONS_INVITE`: Will be removed with the next release. replaced with `SSO_ORGANIZATIONS_ENABLED`.
  - `SSO_ORGANIZATIONS_ID_MAPPING` Will be removed with the next release. For now if present is still used, only Organization and User role mapping is done.
- :warning: new database modification (add a column with a default value, old version will run on the modified db).
- Add sso identifier in admin user panel
- Fix enforcing of organization master password policies.

# 1.33.2-4

- Augment default `refresh_token` validity to 90days on mobile (match [Bitwarden](https://bitwarden.com/blog/configuring-bitwarden-clients-for-offline-access/#staying-logged-in-to-browser-extension-desktop-and-mobile-clients), apply to non SSO login or if `SSO_AUTH_ONLY_NOT_SESSION` is activated).
- Remove a duplicate token expiration check, and log the `refresh_token` on decode failure (`LOG_LEVEL=info,vaultwarden::auth=debug`).

# 1.33.2-3

- Member role fix on invite/update https://github.com/dani-garcia/vaultwarden/pull/5673

# 1.33.2-2

- Ignore unsupported roles

# 1.33.2-1

- Rebased on `1.33.2` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2025.1.1-5`
- :warning: Upgrade openidconnect to 4.0.0 :warning:
- :warning: Org enrollment is now more aligned with Bitwarden, but user can get enrolled even without using the invitation link :warning:

# 1.33.1-2

- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2025.1.1-2`
  \
  On redirection keep only the base path (cleaner url without left-over `code`).

# 1.33.1-1

- Rebased on `1.33.1` from `dani-garcia/vaultwarden`

# 1.33.0-5

- `SSO_ORGANIZATIONS_ID_MAPPING` organization can now be mapped using `uuid` or `name`.

# 1.33.0-4

- If `SSO_ORGANIZATIONS_ID_MAPPING` is defined then revocation will apply only to the listed organizations.
  \
  Can be used to restrict on which organizations the revocation logic apply.

# 1.33.0-3

- Added `SSO_ORGANIZATIONS_REVOCATION` to control membership revocation activation, disabled by default.

# 1.33.0-2

- Add revocation support
  \
  :warning: if `SSO_ORGANIZATIONS_INVITE` is activated and the provider do not return a matching group for an organization then the user membership will be revoked.
  \
  More details in [README.md#Revocation](https://github.com/Timshel/vaultwarden/blob/main/README.md#revocation)


# 1.33.0-1

- Rebased on `1.33.0` from `dani-garcia/vaultwarden`
  :warning: This includes a DB migration; but the added column comes with a default value so a rollback is still possible.:warning:
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2025.1.1-1`
  Add dynamic CSS support
- Base64 encode state before sending it to providers
- Prevent disabled User from logging using SSO
- Disable signups if SSO_ONLY is activated

# 1.32.7-1

- Rebased on `1.32.7` from `dani-garcia/vaultwarden`

# 1.32.5-1

- Rebased on `1.32.5` from `dani-garcia/vaultwarden`

# 1.32.4-1

- Rebased on `1.32.4` from `dani-garcia/vaultwarden`

# 1.32.3-1

- Rebased on `1.32.3` from `dani-garcia/vaultwarden`

# 1.32.2-1

- Rebased on `1.32.2` from `dani-garcia/vaultwarden`

# 1.32.1-2

- Fix result ordering when searching sso_user on login

# 1.32.1-1

- Rebased on `1.32.1` from `dani-garcia/vaultwarden`
- Org invitation now redirect to SSO login if `SSO_ONLY=true` is set.
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.6.2-4`
- Add `ORGANIZATION_INVITE_AUTO_ACCEPT`
- :warning: Breaking change :warning:
  - `SSO_PKCE` is now on by default, if you are running Zitadel you'll probably need to set it to `false` since it's incompatible with `CLIENT_SECRET`
  - On first SSO login if the provider does not return the email verification status log in will be blocked.
    Check the [documentation](https://github.com/Timshel/vaultwarden/blob/main/SSO.md#on-sso_allow_unknown_email_verification) for more details.

# 1.32.0-2

- Based on `1.32.0` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.6.2-2`
  Org invitation was lost when creating the master password post SSO loogin.

# 1.32.0-1

- Rebased on `1.32.0` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.6.2-1`
- Removed `LOG_LEVEL_OVERRIDE` since `LOG_LEVEL=info,vaultwarden::sso=debug` is now available

# 1.32.0-1

- Rebased on `1.32.0` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.6.2-1`
- Removed `LOG_LEVEL_OVERRIDE` since `LOG_LEVEL=info,vaultwarden::sso=debug` is now available

## 1.31.0-1

- Rebased on `1.31.0` from `dani-garcia/vaultwarden`
- Upgrade [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.5.1-3`
- Use `WEB_VAULT_FOLDER` to switch front-end without modifying the FS

## 1.30.5-9

- Fix organization invitation when SMTP is disabled.
- Add `SSO_ORGANIZATIONS_ALL_COLLECTIONS` config to allow to grant or not access to all collections (default `true`)

## 1.30.5-8

- Rebased on top dani-garcia/vaultwarden latest `main`.
- Update [oidc_web_builds](https://github.com/Timshel/oidc_web_builds) version to `v2024.3.1-1` which introduce new layout.
- Stop rolling the device token (too many issues with refresh token calls in parallel).

## 1.30.5-7

- Fix mysql sso_users.identifier key creation error.

## 1.30.5-6

- Fix lower case issue which generated invalid "your email has changed" (thx @tribut).

## 1.30.5-5

- Add `SSO_ORGANIZATIONS_ID_MAPPING` to map a Provider group `id` to a Vaultwarden organization `uuid`.

## 1.30.5-4

- Rebased on latest from [dani-garcia:main](https://github.com/dani-garcia/vaultwarden/tree/main)
- Move docker release to [timshel](https://hub.docker.com/repository/docker/timshel/vaultwarden/general)
- Split the `experimental` version to a separate [repository](https://hub.docker.com/repository/docker/timshel/oidcwarden/general).

## 1.30.5-3

- Fix `ForeignKeyViolation` when trying to delete sso user.

## 1.30.5-2

- Store SSO identifier to prevent account takeover

## 1.30.5-1

- Rebased on latest from `dani-garcia/vaultwarden`

## 1.30.3-2

- Add `SSO_CLIENT_CACHE_EXPIRATION` config, to optionally cache the calls to the OpenID discovery endpoint.
- Add a `scope` and `iss` in the oidc redirection to try to fix the IOS login failure.

## 1.30.3-1

- Add `SSO_PKCE` config, disabled for now will probably be activated by defaut in next release.

## 1.30.2-7

- Reduce default `refresh_validity` to 7 days (reset with each `access_token` refresh, so act as an idle timer).
   Apply to non sso login and SSO which return a non JWT token with no expiration information.
- Roll the already present `Device.refresh_token` which will invalidate past `refresh_token` (SSO and non SSO login).
- Remove the `openidconnect` cache since it's not [recommended](https://github.com/ramosbugs/openidconnect-rs/issues/25).

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
