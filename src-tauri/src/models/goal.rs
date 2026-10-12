use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub current_value: f64,
    pub target_value: f64,
    pub unit: String,
    pub deadline: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGoalPayload {
    pub title: String,
    pub description: Option<String>,
    pub current_value: Option<f64>,
    pub target_value: f64,
    pub unit: String,
    pub deadline: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGoalPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub current_value: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
    pub deadline: Option<String>,
    pub status: Option<String>,
}
