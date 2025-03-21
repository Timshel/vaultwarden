use crate::{
    api::{core::log_event, core::organizations::CollectionData, core::two_factor, ApiResult, EmptyResult},
    auth::ClientIp,
    db::models::*,
    db::DbConn,
    mail, CONFIG,
};

#[allow(clippy::too_many_arguments)]
pub async fn invite(
    act_user_id: &UserId,
    device: &Device,
    ip: &ClientIp,
    org: &Organization,
    user: &User,
    membership_type: MembershipType,
    groups: &Vec<GroupId>,
    access_all: bool,
    collections: &Vec<CollectionData>,
    invited_by_email: String,
    auto: bool,
    conn: &mut DbConn,
) -> ApiResult<()> {
    let mut membership_status = MembershipStatus::Invited;

    // automatically accept existing users if mail is disabled or config if set
    if (!user.password_hash.is_empty() && !CONFIG.mail_enabled())
        || (CONFIG.sso_enabled() && CONFIG.organization_invite_auto_accept())
    {
        membership_status = MembershipStatus::Accepted;
    }

    let mut new_member = Membership::new(user.uuid.clone(), org.uuid.clone(), Some(invited_by_email.clone()));
    new_member.access_all = access_all;
    new_member.atype = membership_type as i32;
    new_member.status = membership_status as i32;

    // If no accessAll, add the collections received
    if !access_all {
        for col in collections {
            match Collection::find_by_uuid_and_org(&col.id, &org.uuid, conn).await {
                None => err!("Collection not found in Organization"),
                Some(collection) => {
                    CollectionUser::save(
                        &user.uuid,
                        &collection.uuid,
                        col.read_only,
                        col.hide_passwords,
                        col.manage,
                        conn,
                    )
                    .await?;
                }
            }
        }
    }

    new_member.save(conn).await?;

    for group in groups {
        let mut group_entry = GroupUser::new(group.clone(), new_member.uuid.clone());
        group_entry.save(conn).await?;
    }

    log_event(
        EventType::OrganizationUserInvited as i32,
        &new_member.uuid,
        &org.uuid,
        act_user_id,
        device.atype,
        &ip.ip,
        conn,
    )
    .await;

    if CONFIG.mail_enabled() {
        match membership_status {
            MembershipStatus::Invited => {
                if let Err(e) = mail::send_invite(
                    user,
                    org.uuid.clone(),
                    new_member.uuid.clone(),
                    &org.name,
                    new_member.invited_by_email.clone(),
                )
                .await
                {
                    new_member.delete(conn).await?;
                    err!(format!("Error sending invite: {e:?} "));
                }
            }
            MembershipStatus::Accepted => {
                mail::send_enrolled(&user.email, &org.name).await?;
                if auto {
                    mail::send_invite_accepted(&user.email, &invited_by_email, &org.name).await?;
                }
            }
            MembershipStatus::Revoked | MembershipStatus::Confirmed => (),
        }
    }

    Ok(())
}

pub async fn revoke_member(
    act_user_id: &UserId,
    device: &Device,
    ip: &ClientIp,
    mut member: Membership,
    conn: &mut DbConn,
) -> EmptyResult {
    if member.atype == MembershipType::Owner
        && Membership::count_confirmed_by_org_and_type(&member.org_uuid, MembershipType::Owner, conn).await <= 1
    {
        err!(format!("Organization must have at least one confirmed owner, cannot revoke membership ({})", member.uuid))
    }

    member.revoke();
    member.save(conn).await?;

    log_event(
        EventType::OrganizationUserRevoked as i32,
        &member.uuid,
        &member.org_uuid,
        act_user_id,
        device.atype,
        &ip.ip,
        conn,
    )
    .await;

    Ok(())
}

pub async fn restore_member(
    act_user_id: &UserId,
    device: &Device,
    ip: &ClientIp,
    mut member: Membership,
    conn: &mut DbConn,
) -> EmptyResult {
    // This check is also done at accept_invite, _confirm_invite, _activate_member, edit_member, admin::update_membership_type
    // It returns different error messages per function.
    if member.atype < MembershipType::Admin {
        match OrgPolicy::is_user_allowed(&member.user_uuid, &member.org_uuid, false, conn).await {
            Ok(_) => {}
            Err(OrgPolicyErr::TwoFactorMissing) => {
                if CONFIG.email_2fa_auto_fallback() {
                    two_factor::email::find_and_activate_email_2fa(&member.user_uuid, conn).await?;
                } else {
                    err!(format!(
                        "Cannot restore this user because they have not setup 2FA (membership {})",
                        member.uuid
                    ));
                }
            }
            Err(OrgPolicyErr::SingleOrgEnforced) => {
                err!(format!("Cannot restore this user because they are a member of an organization which forbids it (membership {})", member.uuid));
            }
        }
    }

    member.restore();
    member.save(conn).await?;

    log_event(
        EventType::OrganizationUserRestored as i32,
        &member.uuid,
        &member.org_uuid,
        act_user_id,
        device.atype,
        &ip.ip,
        conn,
    )
    .await;

    Ok(())
}
