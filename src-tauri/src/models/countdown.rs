use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Countdown {
    pub id: i64,
    pub title: String,
    pub target_date: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCountdownPayload {
    pub title: String,
    pub target_date: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCountdownPayload {
    pub title: Option<String>,
    pub target_date: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}
