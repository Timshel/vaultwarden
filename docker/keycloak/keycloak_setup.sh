#!/bin/bash

export PATH=$PATH:/opt/keycloak/bin

CANARY=/tmp/keycloak_setup_done

if [ -f $CANARY ]
then
    echo "Setup should already be done. Will not run."
    exit 0
fi

while true; do
    sleep 5
    kcadm.sh config credentials --server "http://${KC_HTTP_HOST}:${KC_HTTP_PORT}" --realm master --user "$KEYCLOAK_ADMIN" --password "$KEYCLOAK_ADMIN_PASSWORD" --client admin-cli
    EC=$?
    if [ $EC -eq 0 ]; then
        break
    fi
    echo "Will retry in 5 seconds"
done

kcadm.sh create realms -s realm="$TEST_REALM" -s enabled=true -s "accessTokenLifespan=600"

## Delete default roles mapping
DEFAULT_ROLE_SCOPE_ID=$(kcadm.sh get -r "$TEST_REALM" client-scopes | jq -r '.[] | select(.name == "roles") | .id')
kcadm.sh delete -r "$TEST_REALM" "client-scopes/$DEFAULT_ROLE_SCOPE_ID"

## Create role mapping client scope
TEST_CLIENT_ROLES_SCOPE_ID=$(kcadm.sh create -r "$TEST_REALM" client-scopes -s name=roles -s protocol=openid-connect -i)
kcadm.sh create -r "$TEST_REALM" "client-scopes/$TEST_CLIENT_ROLES_SCOPE_ID/protocol-mappers/models" \
    -s name=Roles \
    -s protocol=openid-connect \
    -s protocolMapper=oidc-usermodel-client-role-mapper \
    -s consentRequired=false \
    -s 'config."multivalued"=true' \
    -s 'config."claim.name"=resource_access.${client_id}.roles' \
    -s 'config."full.path"=false' \
    -s 'config."id.token.claim"=true' \
    -s 'config."access.token.claim"=false' \
    -s 'config."userinfo.token.claim"=true'

## Create group mapping client scope
TEST_GROUPS_CLIENT_SCOPE_ID=$(kcadm.sh create -r "$TEST_REALM" client-scopes -s name=groups -s protocol=openid-connect -i)
kcadm.sh create -r "$TEST_REALM" "client-scopes/$TEST_GROUPS_CLIENT_SCOPE_ID/protocol-mappers/models" \
    -s name=Groups \
    -s protocol=openid-connect \
    -s protocolMapper=oidc-group-membership-mapper \
    -s consentRequired=false \
    -s 'config."claim.name"=groups' \
    -s 'config."full.path"=false' \
    -s 'config."id.token.claim"=true' \
    -s 'config."access.token.claim"=true' \
    -s 'config."userinfo.token.claim"=true'

TEST_GROUP_ID=$(kcadm.sh create -r "$TEST_REALM" groups -s name=Test -i)

TEST_CLIENT_ID=$(kcadm.sh create -r "$TEST_REALM" clients -s "name=VaultWarden" -s "clientId=$SSO_CLIENT_ID" -s "secret=$SSO_CLIENT_SECRET" -s "redirectUris=[\"$DOMAIN/*\"]" -i)

## ADD Role mapping scope
kcadm.sh update -r "$TEST_REALM" "clients/$TEST_CLIENT_ID" --body "{\"optionalClientScopes\": [\"$TEST_CLIENT_ROLES_SCOPE_ID\"]}"
kcadm.sh update -r "$TEST_REALM" "clients/$TEST_CLIENT_ID/optional-client-scopes/$TEST_CLIENT_ROLES_SCOPE_ID"

## ADD Group mapping scope
kcadm.sh update -r "$TEST_REALM" "clients/$TEST_CLIENT_ID" --body "{\"optionalClientScopes\": [\"$TEST_GROUPS_CLIENT_SCOPE_ID\"]}"
kcadm.sh update -r "$TEST_REALM" "clients/$TEST_CLIENT_ID/optional-client-scopes/$TEST_GROUPS_CLIENT_SCOPE_ID"

## CREATE TEST ROLES
kcadm.sh create -r "$TEST_REALM" "clients/$TEST_CLIENT_ID/roles" -s name=admin -s 'description=Admin role'
kcadm.sh create -r "$TEST_REALM" "clients/$TEST_CLIENT_ID/roles" -s name=user -s 'description=Admin role'

# To list roles : kcadm.sh get-roles -r "$TEST_REALM" --cid "$TEST_CLIENT_ID"

TEST_USER_ID=$(kcadm.sh create users -r "$TEST_REALM" -s "username=$TEST_USER" -s "email=$TEST_USER_MAIL"  -s emailVerified=true -s enabled=true -i)
kcadm.sh update -r "$TEST_REALM" "users/$TEST_USER_ID/reset-password" -s type=password -s "value=$TEST_USER_PASSWORD" -n
kcadm.sh update -r "$TEST_REALM" "users/$TEST_USER_ID/groups/$TEST_GROUP_ID"
kcadm.sh add-roles -r "$TEST_REALM" --uusername "$TEST_USER" --cid "$TEST_CLIENT_ID" --rolename admin


TEST_USER_2_ID=$(kcadm.sh create users -r "$TEST_REALM" -s "username=$TEST_USER_2" -s "email=$TEST_USER_2_MAIL"  -s emailVerified=true -s enabled=true -i)
kcadm.sh update -r "$TEST_REALM" "users/$TEST_USER_2_ID/reset-password" -s type=password -s "value=$TEST_USER_2_PASSWORD" -n
kcadm.sh update -r "$TEST_REALM" "users/$TEST_USER_2_ID/groups/$TEST_GROUP_ID"
kcadm.sh add-roles -r "$TEST_REALM" --uusername "$TEST_USER_2" --cid "$TEST_CLIENT_ID" --rolename user

touch $CANARY

# TO DEBUG uncomment the following line to keep the setup container running
# sleep 3600
# THEN in another terminal:
# docker exec -it keycloakSetup-dev /bin/bash
# export PATH=$PATH:/opt/keycloak/bin
# kcadm.sh config credentials --server "http://${KC_HTTP_HOST}:${KC_HTTP_PORT}" --realm master --user "$KEYCLOAK_ADMIN" --password "$KEYCLOAK_ADMIN_PASSWORD" --client admin-cli
# ENJOY
# Doc: https://wjw465150.gitbooks.io/keycloak-documentation/content/server_admin/topics/admin-cli.html
