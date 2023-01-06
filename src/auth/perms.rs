use crate::components::*;
use actix_web::web;
use log::*;
use resalt_models::User;
use resalt_storage::StorageImpl;
use serde_json::Value;

pub const P_ADMIN_SUPERADMIN: &str = "admin.superadmin";
pub const P_ADMIN_GROUP: &str = "admin.group";

pub const P_RUN_LIVE: &str = "run.live";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_LIST: &str = "run.approval.list";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_SUBMIT: &str = "run.approval.submit";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_JUDGE: &str = "run.approval.judge";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_EXECUTE: &str = "run.approval.execute";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_RECOUNT: &str = "run.approval.recount"; // Can reset all approvals/rejects
#[allow(dead_code)]
pub const P_RUN_APPROVAL_CLOSE: &str = "run.approval.close";
#[allow(dead_code)]
pub const P_RUN_APPROVAL_DELETE: &str = "run.approval.delete";
#[allow(dead_code)]
pub const P_RUN_TEMPLATE_LIST: &str = "run.template.list";
#[allow(dead_code)]
pub const P_RUN_TEMPLATE_LOCAL: &str = "run.template.local"; // Can create local templates
#[allow(dead_code)]
pub const P_RUN_TEMPLATE_GLOBAL: &str = "run.template.global"; // Can create global templates

pub const P_MINION_LIST: &str = "minion.list";
pub const P_MINION_CONFORMITY: &str = "minion.conformity";
pub const P_MINION_PILLARS: &str = "minion.pillars";
pub const P_MINION_PACKAGES: &str = "minion.packages";
#[allow(dead_code)]
pub const P_MINION_PRESETS_LIST: &str = "minion.presets.list";
#[allow(dead_code)]
pub const P_MINION_PRESETS_MANAGE: &str = "minion.presets.manage";
pub const P_MINION_GRAINEXPLORER: &str = "minion.grainexplorer";

pub const P_JOB_LIST: &str = "job.list";

pub const P_EVENT_LIST: &str = "event.list";

pub const P_SALTKEY_LIST: &str = "saltkey.list";
pub const P_SALTKEY_ACCEPT: &str = "saltkey.accept";
pub const P_SALTKEY_REJECT: &str = "saltkey.reject";
pub const P_SALTKEY_DELETE: &str = "saltkey.delete";

pub const P_USER_ADMIN: &str = "user";
pub const P_USER_LIST: &str = "user.list";
#[allow(dead_code)]
pub const P_USER_EMAIL: &str = "user.email";
pub const P_USER_PASSWORD: &str = "user.password";

pub fn has_permission(
    data: &web::Data<Box<dyn StorageImpl>>,
    user_id: &str,
    permission: &str,
) -> Result<bool, ApiError> {
    let user = match data.get_user_by_id(user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Ok(false),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let perms = match serde_json::from_str(&user.perms) {
        Ok(perms) => perms,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    Ok(evalute_resalt_permission(&perms, permission))
}

pub fn evalute_resalt_permission(permissions: &Value, permission: &str) -> bool {
    let permissions = match permissions.as_array() {
        Some(permissions) => permissions,
        None => return false,
    };

    // Assume there can be multiple @resalt sections, from ugly merge.
    let resalt_permissions: Vec<&Value> = permissions
        .iter()
        .filter_map(|p| p.get("@resalt"))
        .filter(|p| p.is_array())
        // merge array of arrays
        .flat_map(|p| p.as_array().unwrap())
        .collect();

    // If the permission we are looking for is admin.group.create,
    // test both:
    // - admin.group.create
    // - admin.group
    // - admin
    //
    // Additionally, always return true if they have admin.superadmin.
    let mut test_perms = vec![permission.to_string()];
    for (i, c) in permission.char_indices() {
        if c == '.' {
            test_perms.push(permission[..i].to_string());
        }
    }
    test_perms.push(P_ADMIN_SUPERADMIN.to_string());

    log::debug!("resalt_permissions: {:?}", resalt_permissions);
    log::debug!("permission: {:?}", permission);

    for user_permission in resalt_permissions {
        let user_permission = match user_permission.as_str() {
            Some(user_permission) => user_permission,
            None => continue,
        };
        for test_perm in &test_perms {
            log::debug!(
                "test perm: {:?} {:?} {:?}",
                test_perm,
                user_permission,
                test_perm.eq(user_permission)
            );
            if test_perm.eq(user_permission) {
                return true;
            }
        }
    }
    false
}

pub fn update_user_permissions_from_groups(
    data: &web::Data<Box<dyn StorageImpl>>,
    user: &User,
) -> Result<(), ApiError> {
    let groups = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(groups) => groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(ApiError::DatabaseError);
        }
    };
    let mut perms: Vec<Value> = Vec::new();
    for group in groups {
        // Parse group.perms as json array
        let serdegroup: serde_json::Value = match serde_json::from_str(&group.perms) {
            Ok(serdegroup) => serdegroup,
            Err(e) => {
                error!("{:?}", e);
                return Err(ApiError::DatabaseError);
            }
        };
        let group_perms = match serdegroup.as_array() {
            Some(group_perms) => group_perms,
            None => continue,
        };
        for group_perm in group_perms {
            perms.push(group_perm.clone());
        }
    }
    let perms = Value::Array(perms);
    let perms = serde_json::to_string(&perms).unwrap();
    let mut user: User = user.clone();
    user.perms = perms;
    match data.update_user(&user) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{:?}", e);
            Err(ApiError::DatabaseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::auth::evalute_resalt_permission;

    #[test]
    fn test_evalute_resalt_permission() {
        let perms = from_str(
            r#"[
                "test.ping",
                {
                  "*": [
                    {
                      "my_mod.*": {
                        "args": [
                          "a.*",
                          "b.*"
                        ],
                        "kwargs": {
                          "kwa": "kwa.*",
                          "kwb": "kwb"
                        }
                      }
                    }
                  ]
                },
                {
                  "@runner": [
                    {
                      "runner_mod.*": {
                        "args": [
                          "a.*",
                          "b.*"
                        ],
                        "kwargs": {
                          "kwa": "kwa.*",
                          "kwb": "kwb"
                        }
                      }
                    }
                  ]
                },
                {
                  "@wheel": [
                    "key.list_all"
                  ]
                },
                {
                  "G@os:RedHat": [
                    "kmod.*"
                  ]
                },
                {
                  "minion2": [
                    "network.*",
                    "state.*"
                  ]
                },
                {
                  "minion\\*": [
                    "network.*"
                  ]
                },
                {
                  "@resalt": [
                    "admin.user.changepassword",
                    "admin.user.delete"
                  ]
                }
              ]"#,
        )
        .unwrap();
        assert!(!evalute_resalt_permission(&perms, "test.ping"));
        assert!(evalute_resalt_permission(
            &perms,
            "admin.user.changepassword"
        ));
        assert!(evalute_resalt_permission(&perms, "admin.user.delete"));
        assert!(!evalute_resalt_permission(&perms, "admin.unicorn"));
    }
}
