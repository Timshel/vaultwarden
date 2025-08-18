# OpenID Connect test setup

This `docker-compose` template allow to run locally a `VaultWarden` and [`Authentik`](https://goauthentik.io/) instance to test OIDC.

## Usage

This rely on `docker` and the `compose` [plugin](https://docs.docker.com/compose/install/).
First create a copy of `.env.template` as `.env` (This is done to prevent commiting your custom settings, Ex `SMTP_`).

Then start the stack (the `profile` is required to run the `VaultWarden`).

```bash
> docker compose --profile Vaultwarden up VaultWarden
```

Then you can access :

 - `VaultWarden` on http://localhost:8000 with the default user `test@yopmail.com/test`.
 - `Authentik` on http://127.0.0.1:9000/ with the default user `akadmin/admin`

## Switching VaultWarden front-end

You can switch between both [version](https://github.com/Timshel/oidc_web_builds) of the front-end using the env variable `SSO_FRONTEND` with `button` or `override` (default is `button`).

## Running only Authentik

Since the `VaultWarden` service is defined with a `profile` you can just use the default `docker compose` command :

```bash
> docker compose up
```
## To force rebuilding the VaultWarden image

Use `DOCKER_BUILDKIT=1 docker compose --profile VaultWarden up --build VaultWarden`.

If after building the `Authentik` configuration is not run, just interrupt and run without `--build`

## Cleanup

Use `docker compose --profile VaultWarden down`.

## Issues

At the moment the access token lifetime is set to `5min` which will collide with the expiration detection of `VaultWarden` which is set to `5min` too.
This might result in spamming of the refresh token endpoint and race condition might trigger a logout.
