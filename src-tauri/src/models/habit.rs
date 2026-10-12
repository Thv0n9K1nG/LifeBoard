use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub icon: Option<String>,
    pub color: String,
    pub is_active: bool,
    pub created_at: String,
    pub streak: i64,
    pub checked_in_today: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHabitPayload {
    pub name: String,
    pub description: Option<String>,
    pub frequency: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub frequency: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}
