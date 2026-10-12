use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayAnalytics {
    pub date: String,
    pub completed_tasks: i64,
    pub focus_minutes: i64,
    pub habits_completed: i64,
    pub habits_total: i64,
    pub mood_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklySummary {
    pub total_completed_tasks: i64,
    pub total_focus_minutes: i64,
    pub completion_rate: f64,
    pub avg_mood_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyAnalytics {
    pub week_start: String,
    pub week_end: String,
    pub days: Vec<DayAnalytics>,
    pub summary: WeeklySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlySummary {
    pub total_completed_tasks: i64,
    pub total_focus_hours: f64,
    pub completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyAnalytics {
    pub year: i32,
    pub month: u32,
    pub days: Vec<DayAnalytics>,
    pub summary: MonthlySummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatmapDay {
    pub date: String,
    pub task_count: i64,
    pub focus_minutes: i64,
    pub habit_done: i64,
    pub intensity_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearlyAnalytics {
    pub year: i32,
    pub heatmap: Vec<HeatmapDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodDto {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatsDto {
    pub total: i64,
    pub completed: i64,
    pub completion_rate: f64,
    pub by_priority: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusStatsDto {
    pub total_sessions: i64,
    pub total_minutes: i64,
    pub avg_minutes_per_day: f64,
    pub by_type: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongestStreakDto {
    pub habit_name: String,
    pub streak: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitStatsDto {
    pub longest_streak: Option<LongestStreakDto>,
    pub avg_completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoodStatsDto {
    pub avg_score: f64,
    pub distribution: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub period: PeriodDto,
    pub tasks: TaskStatsDto,
    pub focus: FocusStatsDto,
    pub habits: HabitStatsDto,
    pub mood: MoodStatsDto,
}
