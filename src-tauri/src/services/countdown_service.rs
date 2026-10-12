use crate::error::AppError;
use crate::models::countdown::{Countdown, CreateCountdownPayload, UpdateCountdownPayload};
use chrono::Utc;
use rusqlite::{params, Connection};

fn map_countdown_row(row: &rusqlite::Row) -> Result<Countdown, rusqlite::Error> {
    Ok(Countdown {
        id: row.get(0)?,
        title: row.get(1)?,
        target_date: row.get(2)?,
        icon: row.get(3)?,
        color: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

pub fn get_all(conn: &Connection) -> Result<Vec<Countdown>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title, target_date, icon, color, created_at, updated_at
         FROM countdowns ORDER BY target_date ASC",
    )?;
    let items = stmt
        .query_map([], map_countdown_row)?
        .collect::<Result<Vec<Countdown>, _>>()?;
    Ok(items)
}

pub fn create(conn: &Connection, payload: CreateCountdownPayload) -> Result<Countdown, AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("Countdown title cannot be empty".into()));
    }
    let color = payload.color.unwrap_or_else(|| "#6B7280".to_string());
    let clean_target = payload.target_date.split('T').next().unwrap_or(&payload.target_date);
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO countdowns (title, target_date, icon, color, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![title, clean_target, payload.icon, color, now, now],
    )?;

    let id = conn.last_insert_rowid();
    Ok(Countdown {
        id,
        title: title.to_string(),
        target_date: clean_target.to_string(),
        icon: payload.icon,
        color: Some(color),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn update(
    conn: &Connection,
    id: i64,
    payload: UpdateCountdownPayload,
) -> Result<Countdown, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title, target_date, icon, color, created_at, updated_at
         FROM countdowns WHERE id = ?1",
    )?;

    let (mut title, mut target_date, mut icon, mut color) = stmt
        .query_row(params![id], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })
        .map_err(|_| AppError::NotFound(format!("Countdown {} not found", id)))?;

    if let Some(t) = payload.title {
        let trimmed = t.trim();
        if trimmed.is_empty() {
            return Err(AppError::Validation("Countdown title cannot be empty".into()));
        }
        title = trimmed.to_string();
    }
    if let Some(td) = payload.target_date {
        target_date = td.split('T').next().unwrap_or(&td).to_string();
    }
    if let Some(ic) = payload.icon {
        icon = Some(ic);
    }
    if let Some(c) = payload.color {
        color = Some(c);
    }
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE countdowns SET title = ?1, target_date = ?2, icon = ?3, color = ?4, updated_at = ?5
         WHERE id = ?6",
        params![title, target_date, icon, color, now, id],
    )?;

    Ok(Countdown {
        id,
        title,
        target_date,
        icon,
        color,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM countdowns WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Countdown {} not found", id)));
    }
    Ok(())
}
