use axum::http::StatusCode;
use log::*;
use regex::Regex;
use resalt_models::AuthStatus;
use serde_json::Value;
use std::collections::HashMap;

pub const P_ADMIN_SUPERADMIN: &str = "admin.superadmin";
pub const P_ADMIN_GROUP: &str = "admin.group";
pub const P_ADMIN_USER: &str = "admin.user";

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
pub const P_MINION_REFRESH: &str = "minion.refresh";
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

pub const P_USER_LIST: &str = "user.list";
#[allow(dead_code)]
pub const P_USER_EMAIL: &str = "user.email";
pub const P_USER_PASSWORD: &str = "user.password";

pub fn has_resalt_permission(auth: &AuthStatus, permission: &str) -> Result<bool, StatusCode> {
    let perms = match serde_json::from_str(&auth.perms) {
        Ok(perms) => perms,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(evaluate_resalt_permission(&perms, permission))
}

pub(crate) fn evaluate_resalt_permission(permissions: &Value, permission: &str) -> bool {
    let args = Vec::new();
    let kwargs = HashMap::new();
    let normal = evaluate_permission(permissions, "@resalt", permission, &args, &kwargs);
    if !normal {
        evaluate_permission(permissions, "@resalt", P_ADMIN_SUPERADMIN, &args, &kwargs)
    } else {
        normal
    }
}

#[inline]
#[must_use]
fn salt_wrapped_regex(regex: &str) -> String {
    format!("^{}$", regex.replace("([a-zA-Z0-9])\\*", "$1.*"))
}

fn evaluate_function(
    fun_section_perm: &Value,
    fun: &str,
    args: &Vec<String>,
    kwargs: &HashMap<String, String>,
) -> bool {
    if let Some(fun_section) = fun_section_perm.as_str() {
        let regex = salt_wrapped_regex(fun_section);
        let re = Regex::new(&regex).unwrap();
        return re.is_match(fun);
    }
    let keys = fun_section_perm
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
        if re.is_match(fun) {
            let value = &fun_section_perm[key];
            if let Some(_value) = value.as_str() {
                return true;
            }
            if let Some(value) = value.as_array() {
                if value.is_empty() && !args.is_empty() {
                    return false;
                }
                // Test each arg in the permission argainst "args"
                let mut result = true;
                for (i, value) in value.iter().enumerate() {
                    let regex = salt_wrapped_regex(value.as_str().unwrap());
                    let re = Regex::new(&regex).unwrap();
                    if !re.is_match(&args[i]) {
                        result = false;
                        break;
                    }
                }
                return result;
            }
            if let Some(value) = value.as_object() {
                if let Some(value) = value.get("args") {
                    if let Some(value) = value.as_array() {
                        if value.is_empty() && !args.is_empty() {
                            return false;
                        }
                        // Test each arg in the permission argainst "args"
                        for (i, value) in value.iter().enumerate() {
                            let regex = salt_wrapped_regex(value.as_str().unwrap());
                            let re = Regex::new(&regex).unwrap();
                            if !re.is_match(&args[i]) {
                                return false;
                            }
                        }
                    }
                }
                if let Some(value) = value.get("kwargs") {
                    if let Some(value) = value.as_object() {
                        let keys = value.keys().collect::<Vec<_>>();
                        if keys.is_empty() && !kwargs.is_empty() {
                            return false;
                        }
                        // Test each arg in the permission argainst "kwargs"
                        for key in keys {
                            let regex = salt_wrapped_regex(value[key].as_str().unwrap());
                            let re = Regex::new(&regex).unwrap();
                            if kwargs.contains_key(key) && !re.is_match(&kwargs[key]) {
                                return false;
                            }
                        }
                    }
                    return true;
                }
            }
        }
    }
    false
}

fn evaluate_target(
    target_section_perm: &Value,
    target: &str,
    fun: &str,
    args: &Vec<String>,
    kwargs: &HashMap<String, String>,
) -> bool {
    if let Some(target_section) = target_section_perm.as_str() {
        let regex = salt_wrapped_regex(target_section);
        let re = Regex::new(&regex).unwrap();
        return re.is_match(fun);
    }
    let keys = target_section_perm
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
            let fun_sections = target_section_perm[key].as_array().unwrap();
            for fun_section_perm in fun_sections {
                if evaluate_function(fun_section_perm, fun, args, kwargs) {
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
        if evaluate_target(&permission, target, fun, args, kwargs) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::from_str;

    use crate::permission::{evaluate_permission, evaluate_resalt_permission};

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
