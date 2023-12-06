use serde::{Deserialize, Serialize};

/// Pagination
pub type Paginate = Option<(i64, i64)>;

#[derive(serde::Deserialize)]
pub struct PaginateQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PaginateQuery {
    pub fn parse_query(&self) -> Paginate {
        match (self.limit, self.offset) {
            (Some(limit), Some(offset)) => Some((limit, offset)),
            (Some(limit), None) => Some((limit, 0)),
            _ => None,
        }
    }
}

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

impl FilterOperand {
    pub fn filter_str_logic(self, a: &str, b: &str) -> bool {
        match self {
            FilterOperand::Contains => a.contains(b),
            FilterOperand::NotContains => !a.contains(b),
            FilterOperand::Equals => a == b,
            FilterOperand::NotEquals => a != b,
            FilterOperand::StartsWith => a.starts_with(b),
            FilterOperand::EndsWith => a.ends_with(b),
            FilterOperand::GreaterThanOrEqual => a >= b,
            FilterOperand::LessThanOrEqual => a <= b,
        }
    }
}
