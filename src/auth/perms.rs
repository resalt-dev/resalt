use crate::{
    models::User,
    prelude::{api_error_database, Storage},
};
use actix_web::web;
use log::*;
use serde_json::Value;

pub const P_ADMIN_SUPERADMIN: &str = "admin.superadmin";
pub const P_ADMIN_GROUP: &str = "admin.group";
pub const P_ADMIN_USER: &str = "admin.user";
pub const P_USER_PASSWORD: &str = "user.password";

pub fn has_permission(
    data: &web::Data<Storage>,
    user_id: &str,
    permission: &str,
) -> Result<bool, actix_web::Error> {
    let user = match data.get_user_by_id(user_id) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Ok(false),
        },
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    let perms = match serde_json::from_str(&user.perms) {
        Ok(perms) => perms,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
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
        .filter(|p| p.get("@resalt").is_some())
        .map(|p| p.get("@resalt").unwrap())
        .filter(|p| p.is_array())
        // merge array of arrays
        .map(|p| p.as_array().unwrap())
        .flatten()
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
            if test_perm.eq(user_permission) {
                return true;
            }
        }
    }
    return false;
}

pub fn update_user_permissions_from_groups(
    data: &web::Data<Storage>,
    user: &User,
) -> Result<(), actix_web::Error> {
    let groups = match data.list_permission_groups_by_user_id(&user.id) {
        Ok(groups) => groups,
        Err(e) => {
            error!("{:?}", e);
            return Err(api_error_database());
        }
    };
    let mut perms: Vec<Value> = Vec::new();
    for group in groups {
        // Parse group.perms as json array
        let serdegroup: serde_json::Value = match serde_json::from_str(&group.perms) {
            Ok(serdegroup) => serdegroup,
            Err(e) => {
                error!("{:?}", e);
                return Err(api_error_database());
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
            Err(api_error_database())
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::prelude::evalute_resalt_permission;

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
