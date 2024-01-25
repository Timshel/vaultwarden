# Fork from [dani-garcia/vaultwarden](https://github.com/dani-garcia/vaultwarden)

## Experimental version

In general you don't want to run this version.

## Docker

Docker images available at:

 - Docker hub [hub.docker.com/r/timshel/oidcwarden](https://hub.docker.com/r/timshel/oidcwarden/tags)
 - Github container registry [ghcr.io/timshel/oidcwarden](https://github.com/Timshel/vaultwarden/pkgs/container/oidcwarden)


### Stop storing Master Password hash

This allow to stop storing the Master password in the server database.
This is a work in progress and released for testing.
Once activated newly created account will no longer store a master password hash, making reverting to a standard VaultWarden instance troublesome.

#### To activate

 - `SSO_EXPERIMENTAL_NO_MASTER_PWD`: Control the activation of the feature. Default `true`.

Additionnaly a new web build is available which stop sending the hash cf `experimental` in [Timshel/oidc_web_builds](https://github.com/Timshel/oidc_web_builds/releases)
Activated by default but controlled by: `-e SSO_FRONTEND='experimental'` (cf [start.sh](docker/start.sh)).

#### To revert

You'll first need to run the server without the `experimental` front-end: `-e SSO_FRONTEND='override'`.
\
You can then go to `Account settings \ Security \ Keys` and trigger the `Change KDF`.
\
This endpoint is not modified and will save the new master password hash, every user will need to do this to restore a Master password in db.
