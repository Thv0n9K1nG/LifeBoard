use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub id: i64,
    pub theme: String,
    pub pomodoro_focus_minutes: i64,
    pub pomodoro_break_minutes: i64,
    pub pomodoro_rounds: i64,
    pub habit_grace_period_days: i64,
    pub language: String,
    pub timezone: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsPayload {
    pub theme: Option<String>,
    pub pomodoro_focus_minutes: Option<i64>,
    pub pomodoro_break_minutes: Option<i64>,
    pub pomodoro_rounds: Option<i64>,
    pub habit_grace_period_days: Option<i64>,
    pub language: Option<String>,
    pub timezone: Option<String>,
}
