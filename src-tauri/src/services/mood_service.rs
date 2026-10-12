use crate::error::AppError;
use crate::models::mood::{MoodEntry, UpsertMoodPayload};
use chrono::Utc;
use rusqlite::{params, Connection};

fn map_mood_row(row: &rusqlite::Row) -> Result<MoodEntry, rusqlite::Error> {
    Ok(MoodEntry {
        id: row.get(0)?,
        entry_date: row.get(1)?,
        score: row.get(2)?,
        note: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

pub fn get_all(conn: &Connection) -> Result<Vec<MoodEntry>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, entry_date, score, note, created_at, updated_at
         FROM mood_entries ORDER BY entry_date DESC",
    )?;
    let entries = stmt
        .query_map([], map_mood_row)?
        .collect::<Result<Vec<MoodEntry>, _>>()?;
    Ok(entries)
}

pub fn get_by_date(conn: &Connection, date: &str) -> Result<Option<MoodEntry>, AppError> {
    let clean_date = date.split('T').next().unwrap_or(date);
    let mut stmt = conn.prepare(
        "SELECT id, entry_date, score, note, created_at, updated_at
         FROM mood_entries WHERE entry_date = ?1",
    )?;
    let entry = stmt
        .query_row(params![clean_date], map_mood_row)
        .map(Some)
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other),
        })?;
    Ok(entry)
}

pub fn upsert(conn: &Connection, payload: UpsertMoodPayload) -> Result<MoodEntry, AppError> {
    if payload.score < 1 || payload.score > 5 {
        return Err(AppError::Validation("Mood score must be between 1 and 5".into()));
    }
    let clean_date = payload.entry_date.split('T').next().unwrap_or(&payload.entry_date);
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO mood_entries (entry_date, score, note, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(entry_date)
         DO UPDATE SET score = ?2, note = ?3, updated_at = ?5",
        params![clean_date, payload.score, payload.note, now, now],
    )?;

    get_by_date(conn, clean_date)?
        .ok_or_else(|| AppError::Internal("Failed to retrieve upserted mood entry".into()))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM mood_entries WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Mood entry {} not found", id)));
    }
    Ok(())
}
