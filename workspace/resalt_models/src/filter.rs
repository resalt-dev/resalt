use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "fieldType")]
    pub field_type: FilterFieldType,
    pub field: String,
    pub operand: FilterOperand,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterFieldType {
    #[serde(rename = "")]
    None,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "grain")]
    Grain,
    #[serde(rename = "package")]
    Package,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterOperand {
    #[serde(rename = "c")]
    Contains,
    #[serde(rename = "nc")]
    NotContains,
    #[serde(rename = "e")]
    Equals,
    #[serde(rename = "ne")]
    NotEquals,
    #[serde(rename = "sw")]
    StartsWith,
    #[serde(rename = "ew")]
    EndsWith,
    #[serde(rename = "gte")]
    GreaterThanOrEqual,
    #[serde(rename = "lte")]
    LessThanOrEqual,
}
