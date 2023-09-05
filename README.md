# Fork from [dani-garcia/vaultwarden](https://github.com/dani-garcia/vaultwarden)

Goal is to help testing code for the SSO [PR](https://github.com/dani-garcia/vaultwarden/pull/3154).
Based on [timshel/sso-support](https://github.com/Timshel/vaultwarden/tree/sso-support)

## Docker

Override the [Dockerfile](Dockerfile) to use front-end builds from : https://github.com/Timshel/oidc_web_builds/releases
By default it will use the release which make the `sso` button visible and fix organization invitation to persist across the sso redirection.

If you want to use the version which additionally change the default redirection to `/sso`.
``` bash
> sed -i 's/oidc_button_web_vault/oidc_overide_web_vault/' Dockerfile
```

## To test VaultWarden with Keycloak

[Readme](test/oidc/README.md)

## DB Migration

:warning: Changed the past migration in a recent [commit](https://github.com/Timshel/vaultwarden/commit/afa26f3cf5a39ff0bc4c3cbe563cfcfaf91b40a0).:warning: <br>
If you already deployed the previous version you'll need to do some manual cleanup :

```psql
>BEGIN;
BEGIN
>DELETE FROM __diesel_schema_migrations WHERE version = '20230201133000';
DELETE 1
>DROP TABLE sso_nonce;
DROP TABLE
> COMMIT / ROLLBACK;
```
