use crate::database::DbState;
use crate::error::AppError;
use crate::models::analytics::*;
use crate::models::countdown::{Countdown, CreateCountdownPayload, UpdateCountdownPayload};
use crate::models::focus::{
    CreateFocusSessionPayload, FocusSession, FocusSessionResponse, StopwatchState,
    UpdateFocusSessionPayload,
};
use crate::models::goal::{CreateGoalPayload, Goal, UpdateGoalPayload};
use crate::models::habit::{CreateHabitPayload, Habit, UpdateHabitPayload};
use crate::models::mood::{MoodEntry, UpsertMoodPayload};
use crate::models::note::{DailyNote, UpsertDailyNotePayload};
use crate::models::settings::{Settings, UpdateSettingsPayload};
use crate::models::tag::{CreateTagPayload, Tag, UpdateTagPayload};
use crate::models::task::{
    CreateTaskPayload, GetTasksParams, Task, TaskListResponse, UpdateTaskPayload,
};
use crate::services::*;
use tauri::State;

// ── Tag Commands ──────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_tags(state: State<DbState>) -> Result<Vec<Tag>, AppError> {
    let conn = state.conn.lock().unwrap();
    tag_service::get_all(&conn)
}

#[tauri::command]
pub fn create_tag(state: State<DbState>, payload: CreateTagPayload) -> Result<Tag, AppError> {
    let conn = state.conn.lock().unwrap();
    tag_service::create(&conn, payload)
}

#[tauri::command]
pub fn update_tag(
    state: State<DbState>,
    id: i64,
    payload: UpdateTagPayload,
) -> Result<Tag, AppError> {
    let conn = state.conn.lock().unwrap();
    tag_service::update(&conn, id, payload)
}

#[tauri::command]
pub fn delete_tag(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    tag_service::delete(&conn, id)
}

// ── Task Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_tasks(
    state: State<DbState>,
    params: Option<GetTasksParams>,
) -> Result<TaskListResponse, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::get_tasks(&conn, params.unwrap_or_default())
}

#[tauri::command]
pub fn get_task_by_id(state: State<DbState>, id: i64) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::get_by_id(&conn, id)
}

#[tauri::command]
pub fn create_task(state: State<DbState>, payload: CreateTaskPayload) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::create(&conn, payload)
}

#[tauri::command]
pub fn update_task(
    state: State<DbState>,
    id: i64,
    payload: UpdateTaskPayload,
) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::update(&conn, id, payload)
}

#[tauri::command]
pub fn complete_task(state: State<DbState>, id: i64) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::complete(&conn, id)
}

#[tauri::command]
pub fn uncomplete_task(state: State<DbState>, id: i64) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::uncomplete(&conn, id)
}

#[tauri::command]
pub fn archive_task(state: State<DbState>, id: i64) -> Result<Task, AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::archive(&conn, id)
}

#[tauri::command]
pub fn delete_task(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    task_service::delete(&conn, id)
}

// ── Habit Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_habits(state: State<DbState>) -> Result<Vec<Habit>, AppError> {
    let conn = state.conn.lock().unwrap();
    habit_service::get_all(&conn)
}

#[tauri::command]
pub fn create_habit(state: State<DbState>, payload: CreateHabitPayload) -> Result<Habit, AppError> {
    let conn = state.conn.lock().unwrap();
    habit_service::create(&conn, payload)
}

#[tauri::command]
pub fn update_habit(
    state: State<DbState>,
    id: i64,
    payload: UpdateHabitPayload,
) -> Result<Habit, AppError> {
    let conn = state.conn.lock().unwrap();
    habit_service::update(&conn, id, payload)
}

#[tauri::command]
pub fn delete_habit(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    habit_service::delete(&conn, id)
}

#[tauri::command]
pub fn checkin_habit(
    state: State<DbState>,
    id: i64,
    date: String,
    is_completed: bool,
) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    habit_service::upsert_checkin(&conn, id, &date, is_completed)
}

// ── Goal Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_goals(state: State<DbState>, status: Option<String>) -> Result<Vec<Goal>, AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::get_all(&conn, status)
}

#[tauri::command]
pub fn get_goal_by_id(state: State<DbState>, id: i64) -> Result<Goal, AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::get_by_id(&conn, id)
}

#[tauri::command]
pub fn create_goal(state: State<DbState>, payload: CreateGoalPayload) -> Result<Goal, AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::create(&conn, payload)
}

#[tauri::command]
pub fn update_goal(
    state: State<DbState>,
    id: i64,
    payload: UpdateGoalPayload,
) -> Result<Goal, AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::update(&conn, id, payload)
}

#[tauri::command]
pub fn update_goal_progress(
    state: State<DbState>,
    id: i64,
    current_value: f64,
) -> Result<Goal, AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::update_progress(&conn, id, current_value)
}

#[tauri::command]
pub fn delete_goal(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    goal_service::delete(&conn, id)
}

// ── Note Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_daily_notes(state: State<DbState>) -> Result<Vec<DailyNote>, AppError> {
    let conn = state.conn.lock().unwrap();
    note_service::get_all(&conn)
}

#[tauri::command]
pub fn get_daily_note_by_date(
    state: State<DbState>,
    date: String,
) -> Result<Option<DailyNote>, AppError> {
    let conn = state.conn.lock().unwrap();
    note_service::get_by_date(&conn, &date)
}

#[tauri::command]
pub fn upsert_daily_note(
    state: State<DbState>,
    payload: UpsertDailyNotePayload,
) -> Result<DailyNote, AppError> {
    let conn = state.conn.lock().unwrap();
    note_service::upsert(&conn, payload)
}

#[tauri::command]
pub fn delete_daily_note(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    note_service::delete(&conn, id)
}

// ── Mood Commands ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_mood_entries(state: State<DbState>) -> Result<Vec<MoodEntry>, AppError> {
    let conn = state.conn.lock().unwrap();
    mood_service::get_all(&conn)
}

#[tauri::command]
pub fn get_mood_entry_by_date(
    state: State<DbState>,
    date: String,
) -> Result<Option<MoodEntry>, AppError> {
    let conn = state.conn.lock().unwrap();
    mood_service::get_by_date(&conn, &date)
}

#[tauri::command]
pub fn upsert_mood_entry(
    state: State<DbState>,
    payload: UpsertMoodPayload,
) -> Result<MoodEntry, AppError> {
    let conn = state.conn.lock().unwrap();
    mood_service::upsert(&conn, payload)
}

#[tauri::command]
pub fn delete_mood_entry(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    mood_service::delete(&conn, id)
}

// ── Countdown Commands ────────────────────────────────────────────────────

#[tauri::command]
pub fn get_countdowns(state: State<DbState>) -> Result<Vec<Countdown>, AppError> {
    let conn = state.conn.lock().unwrap();
    countdown_service::get_all(&conn)
}

#[tauri::command]
pub fn create_countdown(
    state: State<DbState>,
    payload: CreateCountdownPayload,
) -> Result<Countdown, AppError> {
    let conn = state.conn.lock().unwrap();
    countdown_service::create(&conn, payload)
}

#[tauri::command]
pub fn update_countdown(
    state: State<DbState>,
    id: i64,
    payload: UpdateCountdownPayload,
) -> Result<Countdown, AppError> {
    let conn = state.conn.lock().unwrap();
    countdown_service::update(&conn, id, payload)
}

#[tauri::command]
pub fn delete_countdown(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    countdown_service::delete(&conn, id)
}

// ── Focus & Stopwatch Commands ────────────────────────────────────────────

#[tauri::command]
pub fn get_focus_sessions(
    state: State<DbState>,
    from: String,
    to: String,
    session_type: Option<String>,
) -> Result<FocusSessionResponse, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::get_sessions(&conn, &from, &to, session_type)
}

#[tauri::command]
pub fn create_focus_session(
    state: State<DbState>,
    payload: CreateFocusSessionPayload,
) -> Result<FocusSession, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::create_session(&conn, payload)
}

#[tauri::command]
pub fn update_focus_session(
    state: State<DbState>,
    id: i64,
    payload: UpdateFocusSessionPayload,
) -> Result<FocusSession, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::update_session(&conn, id, payload)
}

#[tauri::command]
pub fn delete_focus_session(state: State<DbState>, id: i64) -> Result<(), AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::delete_session(&conn, id)
}

#[tauri::command]
pub fn get_stopwatch_state(state: State<DbState>) -> Result<Option<StopwatchState>, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::get_stopwatch_state(&conn)
}

#[tauri::command]
pub fn start_stopwatch(
    state: State<DbState>,
    label: Option<String>,
    session_date: Option<String>,
) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::start_stopwatch(&conn, label, session_date)
}

#[tauri::command]
pub fn pause_stopwatch(state: State<DbState>) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::pause_stopwatch(&conn)
}

#[tauri::command]
pub fn resume_stopwatch(state: State<DbState>) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::resume_stopwatch(&conn)
}

#[tauri::command]
pub fn stop_stopwatch(state: State<DbState>) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::stop_stopwatch(&conn)
}

#[tauri::command]
pub fn split_stopwatch(
    state: State<DbState>,
    note: Option<String>,
) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::split_stopwatch(&conn, note)
}

#[tauri::command]
pub fn update_stopwatch_label(
    state: State<DbState>,
    label: String,
) -> Result<StopwatchState, AppError> {
    let conn = state.conn.lock().unwrap();
    focus_service::update_stopwatch_label(&conn, &label)
}

// ── Analytics Commands ────────────────────────────────────────────────────

#[tauri::command]
pub fn get_weekly_analytics(
    state: State<DbState>,
    week_start: String,
) -> Result<WeeklyAnalytics, AppError> {
    let conn = state.conn.lock().unwrap();
    analytics_service::get_weekly(&conn, &week_start)
}

#[tauri::command]
pub fn get_monthly_analytics(
    state: State<DbState>,
    year: i32,
    month: u32,
) -> Result<MonthlyAnalytics, AppError> {
    let conn = state.conn.lock().unwrap();
    analytics_service::get_monthly(&conn, year, month)
}

#[tauri::command]
pub fn get_yearly_analytics(
    state: State<DbState>,
    year: i32,
) -> Result<YearlyAnalytics, AppError> {
    let conn = state.conn.lock().unwrap();
    analytics_service::get_yearly(&conn, year)
}

#[tauri::command]
pub fn get_statistics(
    state: State<DbState>,
    from: String,
    to: String,
) -> Result<Statistics, AppError> {
    let conn = state.conn.lock().unwrap();
    analytics_service::get_statistics(&conn, &from, &to)
}

// ── Settings & Backup Commands ────────────────────────────────────────────

#[tauri::command]
pub fn get_settings(state: State<DbState>) -> Result<Settings, AppError> {
    let conn = state.conn.lock().unwrap();
    settings_service::get_settings(&conn)
}

#[tauri::command]
pub fn update_settings(
    state: State<DbState>,
    payload: UpdateSettingsPayload,
) -> Result<Settings, AppError> {
    let conn = state.conn.lock().unwrap();
    settings_service::update_settings(&conn, payload)
}

#[tauri::command]
pub fn export_backup_json(state: State<DbState>) -> Result<String, AppError> {
    let conn = state.conn.lock().unwrap();
    backup_service::export_backup_json(&conn)
}

#[tauri::command]
pub fn import_backup_json(state: State<DbState>, json_content: String) -> Result<(), AppError> {
    let mut conn = state.conn.lock().unwrap();
    backup_service::import_backup_json(&mut conn, &json_content)
}
