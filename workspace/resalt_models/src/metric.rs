use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricResult {
    pub title: String,
    pub chart: String,
    pub labels: Vec<String>,
    pub data: Vec<MetricResultData>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricResultData {
    pub label: String,
    pub data: Vec<i32>,
}
