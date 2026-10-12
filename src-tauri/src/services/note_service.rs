use crate::error::AppError;
use crate::models::note::{DailyNote, UpsertDailyNotePayload};
use chrono::Utc;
use rusqlite::{params, Connection};

fn map_note_row(row: &rusqlite::Row) -> Result<DailyNote, rusqlite::Error> {
    Ok(DailyNote {
        id: row.get(0)?,
        note_date: row.get(1)?,
        content: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
    })
}

pub fn get_all(conn: &Connection) -> Result<Vec<DailyNote>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, note_date, content, created_at, updated_at
         FROM daily_notes ORDER BY note_date DESC",
    )?;
    let notes = stmt
        .query_map([], map_note_row)?
        .collect::<Result<Vec<DailyNote>, _>>()?;
    Ok(notes)
}

pub fn get_by_date(conn: &Connection, date: &str) -> Result<Option<DailyNote>, AppError> {
    let clean_date = date.split('T').next().unwrap_or(date);
    let mut stmt = conn.prepare(
        "SELECT id, note_date, content, created_at, updated_at
         FROM daily_notes WHERE note_date = ?1",
    )?;
    let note = stmt
        .query_row(params![clean_date], map_note_row)
        .map(Some)
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other),
        })?;
    Ok(note)
}

pub fn upsert(conn: &Connection, payload: UpsertDailyNotePayload) -> Result<DailyNote, AppError> {
    let clean_date = payload.note_date.split('T').next().unwrap_or(&payload.note_date);
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO daily_notes (note_date, content, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(note_date)
         DO UPDATE SET content = ?2, updated_at = ?4",
        params![clean_date, payload.content, now, now],
    )?;

    get_by_date(conn, clean_date)?
        .ok_or_else(|| AppError::Internal("Failed to retrieve upserted note".into()))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM daily_notes WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Daily note {} not found", id)));
    }
    Ok(())
}
