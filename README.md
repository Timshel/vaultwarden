# Fork from [dani-garcia/vaultwarden](https://github.com/dani-garcia/vaultwarden)

Goal is to help testing code for the SSO [PR](https://github.com/dani-garcia/vaultwarden/pull/3899).
Based on [Timshel/sso-support](https://github.com/Timshel/vaultwarden/tree/sso-support)

:warning: Branch will be rebased and forced-pushed from time to time. :warning:

## Additionnal features

This branch now contain additionnal features not added to the SSO [PR](https://github.com/dani-garcia/vaultwarden/pull/3899) since it would slow even more it's review.

### Role mapping

Allow to map roles from the Access token to users to grant access to `VaultWarden` `admin` console.
Support two roles: `admin` or `user`.

This feature is controlled by the following conf:

- `SSO_ROLES_ENABLED`: control if the mapping is done, default is `false`
- `SSO_ROLES_DEFAULT_TO_USER`: do not block login in case of missing or invalid roles, default is `true`.
- `SSO_ROLES_TOKEN_PATH=/resource_access/${SSO_CLIENT_ID}/roles`: path to read roles in the Access token

## Docker

Change the docker files to package both front-end from [Timshel/oidc_web_builds](https://github.com/Timshel/oidc_web_builds/releases).
\
By default it will use the release which only make the `sso` button visible.

If you want to use the version which additionally change the default redirection to `/sso` and fix organization invitation to persist.
You need to pass an env variable: `-e SSO_FRONTEND='override'` (cf [start.sh](docker/start.sh)).

Docker images available at:

 - Docker hub [hub.docker.com/r/oidcwarden/vaultwarden-oidc](https://hub.docker.com/r/oidcwarden/vaultwarden-oidc/tags)
 - Github container registry [ghcr.io/timshel/vaultwarden](https://github.com/Timshel/vaultwarden/pkgs/container/vaultwarden)

### Front-end version

By default front-end version is fixed to prevent regression (check [CHANGELOG.md](CHANGELOG.md)).
\
When building the docker image it can be overrided by passing the `OIDC_WEB_RELEASE` arg.
\
Ex to build with latest: `--build-arg OIDC_WEB_RELEASE="https://github.com/Timshel/oidc_web_builds/releases/latest/download"`

## To test VaultWarden with Keycloak

[Readme](test/oidc/README.md)

## DB Migration

ATM The migrations add an independant table `sso_nonce` and a column `invited_by_email` to `users_organizations`.

### Revert to default VW

Reverting to the default VW DB state can easily be done manually (Make a backup :) :

```psql
>BEGIN;
BEGIN
>DELETE FROM __diesel_schema_migrations WHERE version in ('20230910133000', '20230914133000');
DELETE 2
>DROP TABLE sso_nonce;
DROP TABLE
>ALTER TABLE users_organizations DROP COLUMN invited_by_email;
ALTER TABLE
> COMMIT / ROLLBACK;
```

### FROM old PR Version

:warning: Changed the past migration creating the `sso_nonce` table in a recent [commit](https://github.com/Timshel/vaultwarden/commit/afa26f3cf5a39ff0bc4c3cbe563cfcfaf91b40a0).:warning: <br>
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

Then the new migration will play without issue.
