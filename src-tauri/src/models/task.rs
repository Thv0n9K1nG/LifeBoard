use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    pub deadline: Option<String>,
    pub is_recurring: bool,
    pub recurrence_type: Option<String>,
    pub recurrence_end_date: Option<String>,
    pub parent_task_id: Option<i64>,
    pub planned_date: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub updated_at: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskListResponse {
    pub data: Vec<Task>,
    pub total: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTasksParams {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub planned_date: Option<String>,
    pub search: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskPayload {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub deadline: Option<String>,
    pub is_recurring: Option<bool>,
    pub recurrence_type: Option<String>,
    pub recurrence_end_date: Option<String>,
    pub planned_date: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskPayload {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub deadline: Option<String>,
    pub is_recurring: Option<bool>,
    pub recurrence_type: Option<String>,
    pub recurrence_end_date: Option<String>,
    pub planned_date: Option<String>,
    pub tag_ids: Option<Vec<i64>>,
}
