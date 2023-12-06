use resalt_config::strip_quotes;
use resalt_models::*;
use serde_json::Value;
use version_compare::Cmp;

// TODO: convert to enum
pub fn sort_jobs(jobs: &mut [Job], sort: &str) {
    jobs.sort_by(|a, b| match sort {
        "id.asc" => a.id.cmp(&b.id),
        "id.desc" => b.id.cmp(&a.id),
        "timestamp.asc" => a.timestamp.cmp(&b.timestamp),
        "timestamp.desc" => b.timestamp.cmp(&a.timestamp),
        "jid.asc" => a.jid.cmp(&b.jid),
        "jid.desc" => b.jid.cmp(&a.jid),
        "user.asc" => a.user.cmp(&b.user),
        "user.desc" => b.user.cmp(&a.user),
        _ => std::cmp::Ordering::Equal,
    })
}

fn value_to_simple_str(value: &Value) -> String {
    match value {
        Value::String(s) => strip_quotes(&s.to_string()),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => a
            .iter()
            .map(|v| strip_quotes(&v.to_string()))
            .collect::<Vec<String>>()
            .join(", "),
        Value::Object(_) => String::from("<OBJECT>"),
        Value::Null => String::from("null"),
    }
}

fn filter_i32_logic(minion_value: i32, filter_value: &str, operand: &FilterOperand) -> bool {
    match filter_value.parse::<i32>() {
        Ok(filter_value) => match operand {
            FilterOperand::Equals => minion_value == filter_value,
            FilterOperand::NotEquals => minion_value != filter_value,
            FilterOperand::GreaterThanOrEqual => minion_value >= filter_value,
            FilterOperand::LessThanOrEqual => minion_value <= filter_value,
            FilterOperand::Contains
            | FilterOperand::NotContains
            | FilterOperand::StartsWith
            | FilterOperand::EndsWith => false,
        },
        Err(_) => false,
    }
}

fn filter_timestamp_logic(
    minion_timestamp: ResaltTime,
    filter_timestamp: ResaltTime,
    operand: &FilterOperand,
) -> bool {
    let minion_str = minion_timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    let filter_str = filter_timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    match operand {
        FilterOperand::Contains => minion_str.contains(&filter_str),
        FilterOperand::NotContains => !minion_str.contains(&filter_str),
        FilterOperand::StartsWith => minion_str.starts_with(&filter_str),
        FilterOperand::EndsWith => minion_str.ends_with(&filter_str),
        FilterOperand::Equals => minion_timestamp == filter_timestamp,
        FilterOperand::NotEquals => minion_timestamp != filter_timestamp,
        FilterOperand::GreaterThanOrEqual => minion_timestamp >= filter_timestamp,
        FilterOperand::LessThanOrEqual => minion_timestamp <= filter_timestamp,
    }
}

fn filter_minion(minion: &Minion, filters: &[Filter]) -> bool {
    for filter in filters {
        let operand = filter.operand.clone();
        match filter.field_type {
            FilterFieldType::None => {}
            FilterFieldType::Object => match filter.field.as_str() {
                "id" => {
                    if !operand.filter_str_logic(&minion.id, &filter.value) {
                        return false;
                    }
                }
                "os_type" => {
                    let value = minion.os_type.as_deref().unwrap_or("");
                    if !operand.filter_str_logic(value, &filter.value) {
                        return false;
                    }
                }
                "last_seen" => {
                    if !filter_timestamp_logic(
                        minion.last_seen,
                        ResaltTime::parse_from_str(&filter.value, "%Y-%m-%d %H:%M:%S")
                            .unwrap_or_default(),
                        &filter.operand,
                    ) {
                        return false;
                    }
                }
                "conformity_success" => {
                    let value: i32 = match minion.conformity_success {
                        Some(value) => value,
                        None => return false,
                    };
                    if !filter_i32_logic(value, &filter.value, &filter.operand) {
                        return false;
                    }
                }
                "conformity_incorrect" => {
                    let value: i32 = match minion.conformity_incorrect {
                        Some(value) => value,
                        None => return false,
                    };
                    if !filter_i32_logic(value, &filter.value, &filter.operand) {
                        return false;
                    }
                }
                "conformity_error" => {
                    let value: i32 = match minion.conformity_error {
                        Some(value) => value,
                        None => return false,
                    };
                    if !filter_i32_logic(value, &filter.value, &filter.operand) {
                        return false;
                    }
                }
                _ => {
                    log::warn!("Filtering on unknown field: {}", filter.field);
                    return false;
                }
            },
            FilterFieldType::Grain => {
                let grains = minion.grains.clone().unwrap_or_default();
                let grains: Value = serde_json::from_str(&grains).unwrap_or_default();
                let json_path = filter.field.clone();
                let selected = match jsonpath_lib::select(&grains, &json_path) {
                    Ok(selected) => selected,
                    Err(_) => {
                        log::warn!("Filtering on grain with invalid JSONPath: {}", json_path);
                        return false;
                    }
                };

                // Convert the selected JSON value to a string. "selected" is always a JSON array.
                // If it is empty, return an empty string.
                // If it contains just one object, return that, without quotes.
                // If it contains multiple objects, join them with ", " and without each string having quotes.
                let selected_str = match selected.len() {
                    0 => {
                        if filter.operand == FilterOperand::NotContains && filter.value.is_empty() {
                            return false;
                        }
                        String::new()
                    }
                    1 => value_to_simple_str(selected[0]),
                    _ => selected
                        .iter()
                        .map(|s| value_to_simple_str(s))
                        .collect::<Vec<String>>()
                        .join(", "),
                };

                // log::debug!("Selected stringified: {}", selected_str);

                match filter.operand {
                    FilterOperand::Contains => {
                        if !selected_str.contains(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::NotContains => {
                        if selected_str.contains(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::Equals => {
                        if selected_str != filter.value {
                            return false;
                        }
                    }
                    FilterOperand::NotEquals => {
                        if selected_str == filter.value {
                            return false;
                        }
                    }
                    FilterOperand::StartsWith => {
                        if !selected_str.starts_with(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::EndsWith => {
                        if !selected_str.ends_with(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::GreaterThanOrEqual => {
                        let selected_float = match selected_str.parse::<f64>() {
                            Ok(selected_float) => selected_float,
                            Err(_) => {
                                return false;
                            }
                        };
                        let filter_float = match filter.value.parse::<f64>() {
                            Ok(filter_float) => filter_float,
                            Err(_) => {
                                return false;
                            }
                        };
                        if selected_float < filter_float {
                            return false;
                        }
                    }
                    FilterOperand::LessThanOrEqual => {
                        let selected_float = match selected_str.parse::<f64>() {
                            Ok(selected_float) => selected_float,
                            Err(_) => {
                                return false;
                            }
                        };
                        let filter_float = match filter.value.parse::<f64>() {
                            Ok(filter_float) => filter_float,
                            Err(_) => {
                                return false;
                            }
                        };
                        if selected_float > filter_float {
                            return false;
                        }
                    }
                };
            }
            FilterFieldType::Package => {
                let packages = minion.pkgs.clone().unwrap_or_default();
                let packages: Value = serde_json::from_str(&packages).unwrap_or_default();
                let version = match &packages[&filter.field] {
                    Value::String(s) => Some(s),
                    _ => None,
                };

                match filter.operand {
                    FilterOperand::Contains => {
                        if filter.value.is_empty() {
                            if version.is_none() {
                                return false;
                            }
                        } else if version.is_none() || !version.unwrap().contains(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::NotContains => {
                        if filter.value.is_empty() {
                            if version.is_some() {
                                return false;
                            }
                        } else if version.is_some() && version.unwrap().contains(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::Equals => {
                        if version.is_none() || version.unwrap() != &filter.value {
                            return false;
                        }
                    }
                    FilterOperand::NotEquals => {
                        if version.is_none() || version.unwrap() == &filter.value {
                            return false;
                        }
                    }
                    FilterOperand::StartsWith => {
                        if version.is_none() || !version.unwrap().starts_with(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::EndsWith => {
                        if version.is_none() || !version.unwrap().ends_with(&filter.value) {
                            return false;
                        }
                    }
                    FilterOperand::GreaterThanOrEqual => {
                        if version.is_none() {
                            return false;
                        }
                        match version_compare::compare_to(version.unwrap(), &filter.value, Cmp::Ge)
                        {
                            Ok(result) => {
                                if !result {
                                    return false;
                                }
                            }
                            Err(_) => return false,
                        }
                    }
                    FilterOperand::LessThanOrEqual => {
                        if version.is_none() {
                            return false;
                        }
                        match version_compare::compare_to(version.unwrap(), &filter.value, Cmp::Le)
                        {
                            Ok(result) => {
                                if !result {
                                    return false;
                                }
                            }
                            Err(_) => return false,
                        }
                    }
                };
            }
        }
    }
    true
}

pub fn filter_minions(minions: &mut Vec<Minion>, filters: &[Filter]) {
    // Filter each minion on filter_minion
    minions.retain(|minion| filter_minion(minion, filters));
}
