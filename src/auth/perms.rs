use serde_json::Value;

pub fn evalute_resalt_permission(perms: &Value, node: &str) -> bool {
    let perms = match perms.as_array() {
        Some(perms) => perms,
        None => return false,
    };
    let resalt_section = match perms.iter().filter_map(|p| p.get("@resalt")).next() {
        Some(resalt_section) => resalt_section,
        None => return false,
    };
    let resalt_perms = match resalt_section.as_array() {
        Some(resalt_perms) => resalt_perms,
        None => return false,
    };
    for resalt_perm in resalt_perms {
        let resalt_perm = match resalt_perm.as_str() {
            Some(resalt_perm) => resalt_perm,
            None => continue,
        };
        if node.eq(resalt_perm) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::prelude::evalute_resalt_permission;

    #[test]
    fn test_resalt_perms() {
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
