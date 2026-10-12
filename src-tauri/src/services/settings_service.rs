use crate::error::AppError;
use crate::models::settings::{Settings, UpdateSettingsPayload};
use chrono::Utc;
use rusqlite::{params, Connection};

fn map_settings_row(row: &rusqlite::Row) -> Result<Settings, rusqlite::Error> {
    Ok(Settings {
        id: row.get(0)?,
        theme: row.get(1)?,
        pomodoro_focus_minutes: row.get(2)?,
        pomodoro_break_minutes: row.get(3)?,
        pomodoro_rounds: row.get(4)?,
        habit_grace_period_days: row.get(5)?,
        language: row.get(6)?,
        timezone: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

pub fn get_settings(conn: &Connection) -> Result<Settings, AppError> {
    conn.query_row(
        "SELECT id, theme, pomodoro_focus_minutes, pomodoro_break_minutes, pomodoro_rounds,
                habit_grace_period_days, language, timezone, updated_at
         FROM settings WHERE id = 1",
        [],
        map_settings_row,
    ).map_err(|e| AppError::Database(e))
}

pub fn update_settings(
    conn: &Connection,
    payload: UpdateSettingsPayload,
) -> Result<Settings, AppError> {
    let existing = get_settings(conn)?;
    let now = Utc::now().to_rfc3339();

    let theme = payload.theme.unwrap_or(existing.theme);
    let pomodoro_focus = payload.pomodoro_focus_minutes.unwrap_or(existing.pomodoro_focus_minutes);
    let pomodoro_break = payload.pomodoro_break_minutes.unwrap_or(existing.pomodoro_break_minutes);
    let pomodoro_rounds = payload.pomodoro_rounds.unwrap_or(existing.pomodoro_rounds);
    let habit_grace = payload.habit_grace_period_days.unwrap_or(existing.habit_grace_period_days);
    let language = payload.language.unwrap_or(existing.language);
    let timezone = payload.timezone.unwrap_or(existing.timezone);

    conn.execute(
        "UPDATE settings SET
            theme = ?1, pomodoro_focus_minutes = ?2, pomodoro_break_minutes = ?3,
            pomodoro_rounds = ?4, habit_grace_period_days = ?5, language = ?6,
            timezone = ?7, updated_at = ?8
         WHERE id = 1",
        params![
            theme,
            pomodoro_focus,
            pomodoro_break,
            pomodoro_rounds,
            habit_grace,
            language,
            timezone,
            now,
        ],
    )?;

    get_settings(conn)
}
