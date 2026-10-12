use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoodEntry {
    pub id: i64,
    pub entry_date: String,
    pub score: i64,
    pub note: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertMoodPayload {
    pub entry_date: String,
    pub score: i64,
    pub note: Option<String>,
}
