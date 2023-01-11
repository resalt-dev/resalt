use std::collections::HashMap;

use crate::components::*;
use actix_web::web;
use log::*;
use regex::Regex;
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

pub fn has_resalt_permission(
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
    Ok(evaluate_resalt_permission(&perms, permission))
}

pub fn evaluate_resalt_permission(permissions: &Value, permission: &str) -> bool {
    let args = Vec::new();
    let kwargs = HashMap::new();
    let normal = evaluate_permission(permissions, "@resalt", permission, &args, &kwargs);
    if !normal {
        evaluate_permission(permissions, "@resalt", P_ADMIN_SUPERADMIN, &args, &kwargs)
    } else {
        normal
    }
}

fn salt_wrapped_regex(regex: &str) -> String {
    format!("^{}$", regex.replace("([a-zA-Z0-9])\\*", "$1.*"))
}

fn evaluate_function(
    fun_section: &Value,
    fun: &str,
    args: &Vec<String>,
    kwargs: &HashMap<String, String>,
) -> bool {
    if let Some(fun_section) = fun_section.as_str() {
        let regex = salt_wrapped_regex(fun_section);
        let re = Regex::new(&regex).unwrap();
        return re.is_match(fun);
    }
    if let Some(fun_section) = fun_section.as_array() {
        if fun_section.len() == 0 {
            return true;
        }
        for arg in fun_section {
            let regex = salt_wrapped_regex(arg.as_str().unwrap());
            let re = Regex::new(&regex).unwrap();
            if re.is_match(&args.join(" ")) {
                return true;
            }
        }
        return false;
    }
    if let Some(fun_section) = fun_section.as_object() {
        if fun_section.contains_key("args") {
            let args_section = fun_section["args"].as_array().unwrap();
            if args_section.len() > 0 {
                for arg in args_section {
                    let regex = salt_wrapped_regex(arg.as_str().unwrap());
                    let re = Regex::new(&regex).unwrap();
                    if re.is_match(&args.join(" ")) {
                        return true;
                    }
                }
                return false;
            }
        }
        if fun_section.contains_key("kwargs") {
            let kwargs_section = fun_section["kwargs"].as_object().unwrap();
            if kwargs_section.len() > 0 {
                for key in kwargs_section.keys() {
                    let regex = salt_wrapped_regex(kwargs_section[key].as_str().unwrap());
                    let re = Regex::new(&regex).unwrap();
                    if re.is_match(&kwargs[key]) {
                        return true;
                    }
                }
                return false;
            }
        }
        return true;
    }
    false
}

fn evaluate_target(
    target_section: &Value,
    target: &str,
    fun: &str,
    args: &Vec<String>,
    kwargs: &HashMap<String, String>,
) -> bool {
    if let Some(target_section) = target_section.as_str() {
        let regex = salt_wrapped_regex(target_section);
        let re = Regex::new(&regex).unwrap();
        return re.is_match(fun);
    }
    let keys = target_section
        .as_object()
        .unwrap()
        .keys()
        .collect::<Vec<_>>();
    if keys.len() != 1 {
        return false;
    }
    for key in keys {
        let regex = salt_wrapped_regex(key);
        let re = Regex::new(&regex).unwrap();
        if re.is_match(target) {
            let fun_sections = target_section[key].as_array().unwrap();
            for fun_section in fun_sections {
                if evaluate_function(fun_section, fun, args, kwargs) {
                    return true;
                }
            }
            return false;
        }
    }
    false
}

pub fn evaluate_permission(
    permissions: &Value,
    target: &str,
    fun: &str,
    args: &Vec<String>,
    kwargs: &HashMap<String, String>,
) -> bool {
    let perms = match permissions.as_array() {
        Some(perms) => perms.to_vec(),
        None => Vec::new(),
    };
    for permission in perms {
        if evaluate_target(&permission, target, fun, &args, &kwargs) {
            return true;
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
    use std::collections::HashMap;

    use serde_json::from_str;

    use crate::auth::evaluate_permission;
    use crate::auth::evaluate_resalt_permission;

    #[test]
    fn test_evalute_resalt_permission() {
        let perms = from_str(
            r#"[
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
        assert!(!evaluate_resalt_permission(&perms, "test.ping"));
        assert!(evaluate_resalt_permission(
            &perms,
            "admin.user.changepassword"
        ));
        assert!(evaluate_resalt_permission(&perms, "admin.user.delete"));
        assert!(!evaluate_resalt_permission(&perms, "admin.unicorn"));
    }

    #[test]
    fn test_evalute_permission() {
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
        assert!(evaluate_permission(
            &perms,
            "minion1",
            "test.ping",
            &vec![],
            &HashMap::new()
        ));
        assert!(evaluate_permission(
            &perms,
            "minion1",
            "my_mod.my_fun",
            &vec!["a".to_string(), "b".to_string()],
            &HashMap::new()
        ));
        assert!(evaluate_permission(
            &perms,
            "minion1",
            "my_mod.my_fun",
            &vec!["a".to_string(), "b".to_string()],
            &vec![("kwa".to_string(), "kwa".to_string())]
                .into_iter()
                .collect()
        ));
        assert!(evaluate_permission(
            &perms,
            "minion1",
            "my_mod.my_fun",
            &vec!["a".to_string(), "b".to_string()],
            &vec![
                ("kwa".to_string(), "kwa".to_string()),
                ("kwb".to_string(), "kwb".to_string())
            ]
            .into_iter()
            .collect()
        ));
        assert!(!evaluate_permission(
            &perms,
            "minion1",
            "my_mod.my_fun",
            &vec!["a".to_string(), "b".to_string()],
            &vec![
                ("kwa".to_string(), "kwa".to_string()),
                ("kwb".to_string(), "kwc".to_string())
            ]
            .into_iter()
            .collect()
        ));
        assert!(!evaluate_permission(
            &perms,
            "minion1",
            "my_mod.my_fun",
            &vec!["a".to_string(), "b".to_string()],
            &vec![
                ("kwa".to_string(), "kwc".to_string()),
                ("kwb".to_string(), "kwb".to_string())
            ]
            .into_iter()
            .collect()
        ));
    }
}
