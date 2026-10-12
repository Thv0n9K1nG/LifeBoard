use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSession {
    pub id: i64,
    pub session_type: String,
    pub label: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: i64,
    pub session_date: String,
    pub splits: Option<String>,
    pub stopwatch_state: Option<String>,
    pub paused_duration_seconds: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSessionResponse {
    pub data: Vec<FocusSession>,
    pub total_duration_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFocusSessionPayload {
    pub session_type: String,
    pub label: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub session_date: Option<String>,
    pub splits: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFocusSessionPayload {
    pub session_type: Option<String>,
    pub label: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub session_date: Option<String>,
    pub splits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitRecord {
    pub id: i64,
    pub interval: String,
    pub elapsed: String,
    pub elapsed_sec: i64,
    pub timestamp: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopwatchState {
    pub id: i64,
    pub state: String,
    pub label: Option<String>,
    pub segment_started_at: Option<String>,
    pub accumulated_seconds: i64,
    pub splits: Option<String>,
    pub session_date: String,
}
