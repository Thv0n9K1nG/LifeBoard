use crate::error::AppError;
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupData {
    pub version: u32,
    pub exported_at: String,
    pub data: serde_json::Value,
}

pub fn export_backup_json(conn: &Connection) -> Result<String, AppError> {
    // 1. Tags
    let mut tag_stmt = conn.prepare("SELECT id, name, color FROM tags")?;
    let tags = tag_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "name": r.get::<_, String>(1)?,
                "color": r.get::<_, String>(2)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 2. Tasks
    let mut task_stmt = conn.prepare(
        "SELECT id, title, description, priority, status, deadline, is_recurring,
                recurrence_type, recurrence_end_date, parent_task_id, planned_date,
                created_at, completed_at, updated_at FROM tasks",
    )?;
    let tasks = task_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "title": r.get::<_, String>(1)?,
                "description": r.get::<_, Option<String>>(2)?,
                "priority": r.get::<_, String>(3)?,
                "status": r.get::<_, String>(4)?,
                "deadline": r.get::<_, Option<String>>(5)?,
                "isRecurring": r.get::<_, i64>(6)? != 0,
                "recurrenceType": r.get::<_, Option<String>>(7)?,
                "recurrenceEndDate": r.get::<_, Option<String>>(8)?,
                "parentTaskId": r.get::<_, Option<i64>>(9)?,
                "plannedDate": r.get::<_, Option<String>>(10)?,
                "createdAt": r.get::<_, String>(11)?,
                "completedAt": r.get::<_, Option<String>>(12)?,
                "updatedAt": r.get::<_, String>(13)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 3. Task Tags
    let mut tt_stmt = conn.prepare("SELECT task_id, tag_id FROM task_tags")?;
    let task_tags = tt_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "taskId": r.get::<_, i64>(0)?,
                "tagId": r.get::<_, i64>(1)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 4. Habits
    let mut habit_stmt = conn.prepare(
        "SELECT id, name, description, frequency, icon, color, is_active, created_at FROM habits",
    )?;
    let habits = habit_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "name": r.get::<_, String>(1)?,
                "description": r.get::<_, Option<String>>(2)?,
                "frequency": r.get::<_, String>(3)?,
                "icon": r.get::<_, Option<String>>(4)?,
                "color": r.get::<_, String>(5)?,
                "isActive": r.get::<_, i64>(6)? != 0,
                "createdAt": r.get::<_, String>(7)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 5. Habit Checkins
    let mut hc_stmt = conn.prepare(
        "SELECT id, habit_id, checkin_date, is_completed FROM habit_checkins",
    )?;
    let habit_checkins = hc_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "habitId": r.get::<_, i64>(1)?,
                "checkinDate": r.get::<_, String>(2)?,
                "isCompleted": r.get::<_, i64>(3)? != 0,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 6. Focus Sessions
    let mut fs_stmt = conn.prepare(
        "SELECT id, session_type, label, start_time, end_time, duration_seconds,
                session_date, splits, stopwatch_state, paused_duration_seconds, created_at
         FROM focus_sessions",
    )?;
    let focus_sessions = fs_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "sessionType": r.get::<_, String>(1)?,
                "label": r.get::<_, Option<String>>(2)?,
                "startTime": r.get::<_, String>(3)?,
                "endTime": r.get::<_, Option<String>>(4)?,
                "durationSeconds": r.get::<_, i64>(5)?,
                "sessionDate": r.get::<_, String>(6)?,
                "splits": r.get::<_, Option<String>>(7)?,
                "stopwatchState": r.get::<_, Option<String>>(8)?,
                "pausedDurationSeconds": r.get::<_, Option<i64>>(9)?,
                "createdAt": r.get::<_, String>(10)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 7. Goals
    let mut goal_stmt = conn.prepare(
        "SELECT id, title, description, current_value, target_value, unit, deadline, status, created_at, updated_at FROM goals",
    )?;
    let goals = goal_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "title": r.get::<_, String>(1)?,
                "description": r.get::<_, Option<String>>(2)?,
                "currentValue": r.get::<_, f64>(3)?,
                "targetValue": r.get::<_, f64>(4)?,
                "unit": r.get::<_, String>(5)?,
                "deadline": r.get::<_, Option<String>>(6)?,
                "status": r.get::<_, String>(7)?,
                "createdAt": r.get::<_, String>(8)?,
                "updatedAt": r.get::<_, String>(9)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 8. Daily Notes
    let mut note_stmt = conn.prepare(
        "SELECT id, note_date, content, created_at, updated_at FROM daily_notes",
    )?;
    let daily_notes = note_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "noteDate": r.get::<_, String>(1)?,
                "content": r.get::<_, String>(2)?,
                "createdAt": r.get::<_, String>(3)?,
                "updatedAt": r.get::<_, String>(4)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 9. Mood Entries
    let mut mood_stmt = conn.prepare(
        "SELECT id, entry_date, score, note, created_at, updated_at FROM mood_entries",
    )?;
    let mood_entries = mood_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "entryDate": r.get::<_, String>(1)?,
                "score": r.get::<_, i64>(2)?,
                "note": r.get::<_, Option<String>>(3)?,
                "createdAt": r.get::<_, String>(4)?,
                "updatedAt": r.get::<_, String>(5)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 10. Countdowns
    let mut cd_stmt = conn.prepare(
        "SELECT id, title, target_date, icon, color, created_at, updated_at FROM countdowns",
    )?;
    let countdowns = cd_stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "title": r.get::<_, String>(1)?,
                "targetDate": r.get::<_, String>(2)?,
                "icon": r.get::<_, Option<String>>(3)?,
                "color": r.get::<_, Option<String>>(4)?,
                "createdAt": r.get::<_, String>(5)?,
                "updatedAt": r.get::<_, String>(6)?,
            }))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    // 11. Settings
    let mut set_stmt = conn.prepare(
        "SELECT id, theme, pomodoro_focus_minutes, pomodoro_break_minutes, pomodoro_rounds,
                habit_grace_period_days, language, timezone, updated_at FROM settings WHERE id = 1",
    )?;
    let settings = set_stmt
        .query_row([], |r| {
            Ok(serde_json::json!({
                "id": r.get::<_, i64>(0)?,
                "theme": r.get::<_, String>(1)?,
                "pomodoroFocusMinutes": r.get::<_, i64>(2)?,
                "pomodoroBreakMinutes": r.get::<_, i64>(3)?,
                "pomodoroRounds": r.get::<_, i64>(4)?,
                "habitGracePeriodDays": r.get::<_, i64>(5)?,
                "language": r.get::<_, String>(6)?,
                "timezone": r.get::<_, String>(7)?,
                "updatedAt": r.get::<_, String>(8)?,
            }))
        })
        .unwrap_or(serde_json::json!({}));

    let envelope = BackupData {
        version: 1,
        exported_at: Utc::now().to_rfc3339(),
        data: serde_json::json!({
            "tags": tags,
            "tasks": tasks,
            "taskTags": task_tags,
            "habits": habits,
            "habitCheckins": habit_checkins,
            "focusSessions": focus_sessions,
            "goals": goals,
            "dailyNotes": daily_notes,
            "moodEntries": mood_entries,
            "countdowns": countdowns,
            "settings": settings,
        }),
    };

    serde_json::to_string_pretty(&envelope)
        .map_err(|e| AppError::Internal(format!("Serialization error: {}", e)))
}

pub fn import_backup_json(conn: &mut Connection, json_content: &str) -> Result<(), AppError> {
    let envelope: BackupData = serde_json::from_str(json_content)
        .map_err(|e| AppError::Validation(format!("Invalid backup format: {}", e)))?;

    let tx = conn.transaction()?;

    // Clear tables in reverse dependency order
    tx.execute("DELETE FROM task_tags", [])?;
    tx.execute("DELETE FROM habit_checkins", [])?;
    tx.execute("DELETE FROM tasks", [])?;
    tx.execute("DELETE FROM tags", [])?;
    tx.execute("DELETE FROM habits", [])?;
    tx.execute("DELETE FROM focus_sessions", [])?;
    tx.execute("DELETE FROM goals", [])?;
    tx.execute("DELETE FROM daily_notes", [])?;
    tx.execute("DELETE FROM mood_entries", [])?;
    tx.execute("DELETE FROM countdowns", [])?;

    // 1. Tags
    if let Some(tags) = envelope.data.get("tags").and_then(|v| v.as_array()) {
        for t in tags {
            let id = t["id"].as_i64().unwrap_or(0);
            let name = t["name"].as_str().unwrap_or("");
            let color = t["color"].as_str().unwrap_or("#6B7280");
            tx.execute(
                "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
                params![id, name, color],
            )?;
        }
    }

    // 2. Tasks
    if let Some(tasks) = envelope.data.get("tasks").and_then(|v| v.as_array()) {
        for t in tasks {
            let id = t["id"].as_i64().unwrap_or(0);
            let title = t["title"].as_str().unwrap_or("");
            let description = t["description"].as_str();
            let priority = t["priority"].as_str().unwrap_or("medium");
            let status = t["status"].as_str().unwrap_or("pending");
            let deadline = t["deadline"].as_str();
            let is_recurring = if t["isRecurring"].as_bool().unwrap_or(false) { 1 } else { 0 };
            let recurrence_type = t["recurrenceType"].as_str();
            let recurrence_end_date = t["recurrenceEndDate"].as_str();
            let parent_task_id = t["parentTaskId"].as_i64();
            let planned_date = t["plannedDate"].as_str();
            let created_at = t["createdAt"].as_str().unwrap_or("");
            let completed_at = t["completedAt"].as_str();
            let updated_at = t["updatedAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO tasks (
                    id, title, description, priority, status, deadline, is_recurring,
                    recurrence_type, recurrence_end_date, parent_task_id, planned_date,
                    created_at, completed_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                params![
                    id, title, description, priority, status, deadline, is_recurring,
                    recurrence_type, recurrence_end_date, parent_task_id, planned_date,
                    created_at, completed_at, updated_at
                ],
            )?;
        }
    }

    // 3. Task Tags
    if let Some(task_tags) = envelope.data.get("taskTags").and_then(|v| v.as_array()) {
        for tt in task_tags {
            let task_id = tt["taskId"].as_i64().unwrap_or(0);
            let tag_id = tt["tagId"].as_i64().unwrap_or(0);
            tx.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![task_id, tag_id],
            )?;
        }
    }

    // 4. Habits
    if let Some(habits) = envelope.data.get("habits").and_then(|v| v.as_array()) {
        for h in habits {
            let id = h["id"].as_i64().unwrap_or(0);
            let name = h["name"].as_str().unwrap_or("");
            let description = h["description"].as_str();
            let frequency = h["frequency"].as_str().unwrap_or("daily");
            let icon = h["icon"].as_str();
            let color = h["color"].as_str().unwrap_or("#6B7280");
            let is_active = if h["isActive"].as_bool().unwrap_or(true) { 1 } else { 0 };
            let created_at = h["createdAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO habits (id, name, description, frequency, icon, color, is_active, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id, name, description, frequency, icon, color, is_active, created_at],
            )?;
        }
    }

    // 5. Habit Checkins
    if let Some(checkins) = envelope.data.get("habitCheckins").and_then(|v| v.as_array()) {
        for c in checkins {
            let id = c["id"].as_i64().unwrap_or(0);
            let habit_id = c["habitId"].as_i64().unwrap_or(0);
            let checkin_date = c["checkinDate"].as_str().unwrap_or("");
            let is_completed = if c["isCompleted"].as_bool().unwrap_or(false) { 1 } else { 0 };

            tx.execute(
                "INSERT INTO habit_checkins (id, habit_id, checkin_date, is_completed)
                 VALUES (?1, ?2, ?3, ?4)",
                params![id, habit_id, checkin_date, is_completed],
            )?;
        }
    }

    // 6. Focus Sessions
    if let Some(sessions) = envelope.data.get("focusSessions").and_then(|v| v.as_array()) {
        for s in sessions {
            let id = s["id"].as_i64().unwrap_or(0);
            let session_type = s["sessionType"].as_str().unwrap_or("stopwatch");
            let label = s["label"].as_str();
            let start_time = s["startTime"].as_str().unwrap_or("");
            let end_time = s["endTime"].as_str();
            let duration_seconds = s["durationSeconds"].as_i64().unwrap_or(0);
            let session_date = s["sessionDate"].as_str().unwrap_or("");
            let splits = s["splits"].as_str();
            let stopwatch_state = s["stopwatchState"].as_str();
            let paused_duration_seconds = s["pausedDurationSeconds"].as_i64().unwrap_or(0);
            let created_at = s["createdAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO focus_sessions (
                    id, session_type, label, start_time, end_time, duration_seconds,
                    session_date, splits, stopwatch_state, paused_duration_seconds,
                    created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    id, session_type, label, start_time, end_time, duration_seconds,
                    session_date, splits, stopwatch_state, paused_duration_seconds,
                    created_at
                ],
            )?;
        }
    }

    // 7. Goals
    if let Some(goals) = envelope.data.get("goals").and_then(|v| v.as_array()) {
        for g in goals {
            let id = g["id"].as_i64().unwrap_or(0);
            let title = g["title"].as_str().unwrap_or("");
            let description = g["description"].as_str();
            let current_value = g["currentValue"].as_f64().unwrap_or(0.0);
            let target_value = g["targetValue"].as_f64().unwrap_or(0.0);
            let unit = g["unit"].as_str().unwrap_or("");
            let deadline = g["deadline"].as_str();
            let status = g["status"].as_str().unwrap_or("active");
            let created_at = g["createdAt"].as_str().unwrap_or("");
            let updated_at = g["updatedAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO goals (
                    id, title, description, current_value, target_value, unit,
                    deadline, status, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    id, title, description, current_value, target_value, unit,
                    deadline, status, created_at, updated_at
                ],
            )?;
        }
    }

    // 8. Daily Notes
    if let Some(notes) = envelope.data.get("dailyNotes").and_then(|v| v.as_array()) {
        for n in notes {
            let id = n["id"].as_i64().unwrap_or(0);
            let note_date = n["noteDate"].as_str().unwrap_or("");
            let content = n["content"].as_str().unwrap_or("");
            let created_at = n["createdAt"].as_str().unwrap_or("");
            let updated_at = n["updatedAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO daily_notes (id, note_date, content, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, note_date, content, created_at, updated_at],
            )?;
        }
    }

    // 9. Mood Entries
    if let Some(moods) = envelope.data.get("moodEntries").and_then(|v| v.as_array()) {
        for m in moods {
            let id = m["id"].as_i64().unwrap_or(0);
            let entry_date = m["entryDate"].as_str().unwrap_or("");
            let score = m["score"].as_i64().unwrap_or(3);
            let note = m["note"].as_str();
            let created_at = m["createdAt"].as_str().unwrap_or("");
            let updated_at = m["updatedAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO mood_entries (id, entry_date, score, note, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, entry_date, score, note, created_at, updated_at],
            )?;
        }
    }

    // 10. Countdowns
    if let Some(cds) = envelope.data.get("countdowns").and_then(|v| v.as_array()) {
        for c in cds {
            let id = c["id"].as_i64().unwrap_or(0);
            let title = c["title"].as_str().unwrap_or("");
            let target_date = c["targetDate"].as_str().unwrap_or("");
            let icon = c["icon"].as_str();
            let color = c["color"].as_str().unwrap_or("#6B7280");
            let created_at = c["createdAt"].as_str().unwrap_or("");
            let updated_at = c["updatedAt"].as_str().unwrap_or("");

            tx.execute(
                "INSERT INTO countdowns (id, title, target_date, icon, color, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![id, title, target_date, icon, color, created_at, updated_at],
            )?;
        }
    }

    // 11. Settings
    if let Some(s) = envelope.data.get("settings") {
        if s.is_object() {
            let theme = s["theme"].as_str().unwrap_or("dark");
            let pomodoro_focus = s["pomodoroFocusMinutes"].as_i64().unwrap_or(25);
            let pomodoro_break = s["pomodoroBreakMinutes"].as_i64().unwrap_or(5);
            let pomodoro_rounds = s["pomodoroRounds"].as_i64().unwrap_or(4);
            let habit_grace = s["habitGracePeriodDays"].as_i64().unwrap_or(0);
            let language = s["language"].as_str().unwrap_or("vi");
            let timezone = s["timezone"].as_str().unwrap_or("Asia/Ho_Chi_Minh");
            let updated_at = Utc::now().to_rfc3339();

            tx.execute(
                "UPDATE settings SET
                    theme = ?1, pomodoro_focus_minutes = ?2, pomodoro_break_minutes = ?3,
                    pomodoro_rounds = ?4, habit_grace_period_days = ?5, language = ?6,
                    timezone = ?7, updated_at = ?8
                 WHERE id = 1",
                params![
                    theme, pomodoro_focus, pomodoro_break, pomodoro_rounds,
                    habit_grace, language, timezone, updated_at
                ],
            )?;
        }
    }

    tx.commit()?;
    Ok(())
}
