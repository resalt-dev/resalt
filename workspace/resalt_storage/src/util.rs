use resalt_config::strip_quotes;
use resalt_models::*;
use serde_json::Value;
use version_compare::Cmp;

fn value_to_simple_str(value: &Value) -> String {
    match value {
        Value::String(s) => strip_quotes!(s.to_string()),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => a
            .iter()
            .map(|v| strip_quotes!(v.to_string()))
            .collect::<Vec<String>>()
            .join(", "),
        Value::Object(_) => String::from("<OBJECT>"),
        Value::Null => String::from("null"),
    }
}

pub fn filter_minions_on_grains(minions: &mut Vec<Minion>, filters: &Vec<Filter>) {
    // Map grain values to json paths
    // If filter.field does not start with "$.", prepend it.
    let json_paths: Vec<String> = filters
        .iter()
        .map(|f| {
            f.field
                .starts_with("$.")
                .then(|| f.field.clone())
                .unwrap_or(format!("$.{}", f.field))
        })
        .collect();

    // Filter
    minions.retain(|minion| {
        // Parse Grains from JSON
        let grains = minion.grains.clone().unwrap_or_default();
        let grains: Value = serde_json::from_str(&grains).unwrap_or_default();
        // Iterate filters with index
        for (i, filter) in filters.iter().enumerate() {
            // Skip filters that are not of type Grain, when filtering grains
            if filter.field_type != FilterFieldType::Grain {
                continue;
            }

            let json_path = &json_paths[i];
            let selected = match jsonpath_lib::select(&grains, json_path) {
                Ok(selected) => selected,
                Err(_) => {
                    log::warn!("Filtering on grain with invalid JSONPath: {}", json_path);
                    return false;
                }
            };

            log::info!("Selected: {:?}", selected);

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
                    .map(|s| value_to_simple_str(s.clone()))
                    .collect::<Vec<String>>()
                    .join(", "),
            };

            log::debug!("Selected stringified: {}", selected_str);

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

        return true;
    });
}

pub fn filter_minions_on_packages(minions: &mut Vec<Minion>, filters: &Vec<Filter>) {
    // Filtering on packages is much easier, as we don't use JSONPath's here. The JSON object is a simple "Map<String,String> | null".
    minions.retain(|minion| {
        // Parse Grains from JSON
        let packages = minion.pkgs.clone().unwrap_or_default();
        let packages: Value = serde_json::from_str(&packages).unwrap_or_default();
        // Iterate filters with index
        for filter in filters {
            // Skip filters that are not of type Grain, when filtering grains
            if filter.field_type != FilterFieldType::Package {
                continue;
            }

            let filter_value = filter.value.trim().to_owned();
            let version = match &packages[&filter.field] {
                Value::String(s) => Some(s),
                _ => None,
            };

            match filter.operand {
                FilterOperand::Contains => {
                    if filter_value.len() == 0 {
                        if version.is_none() {
                            return false;
                        }
                    } else if version.is_none() || !version.unwrap().contains(&filter_value) {
                        return false;
                    }
                }
                FilterOperand::NotContains => {
                    if filter_value.len() == 0 {
                        if version.is_some() {
                            return false;
                        }
                    } else if version.is_some() && version.unwrap().contains(&filter_value) {
                        return false;
                    }
                }
                FilterOperand::Equals => {
                    if version.is_none() || version.unwrap() != &filter_value {
                        return false;
                    }
                }
                FilterOperand::NotEquals => {
                    if version.is_none() || version.unwrap() == &filter_value {
                        return false;
                    }
                }
                FilterOperand::StartsWith => {
                    if version.is_none() || !version.unwrap().starts_with(&filter_value) {
                        return false;
                    }
                }
                FilterOperand::EndsWith => {
                    if version.is_none() || !version.unwrap().ends_with(&filter_value) {
                        return false;
                    }
                }
                FilterOperand::GreaterThanOrEqual => {
                    if version.is_none() {
                        return false;
                    }
                    match version_compare::compare_to(version.unwrap(), &filter_value, Cmp::Ge) {
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
                    match version_compare::compare_to(version.unwrap(), &filter_value, Cmp::Le) {
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
        return true;
    });
}
