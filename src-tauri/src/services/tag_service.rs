use crate::error::AppError;
use crate::models::tag::{CreateTagPayload, Tag, UpdateTagPayload};
use rusqlite::{params, Connection};

pub fn get_all(conn: &Connection) -> Result<Vec<Tag>, AppError> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name ASC")?;
    let tags = stmt
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<Tag>, _>>()?;
    Ok(tags)
}

pub fn create(conn: &Connection, payload: CreateTagPayload) -> Result<Tag, AppError> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("Tag name cannot be empty".into()));
    }
    let color = payload.color.unwrap_or_else(|| "#6B7280".to_string());

    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        params![name, color],
    )?;
    let id = conn.last_insert_rowid();

    Ok(Tag {
        id,
        name: name.to_string(),
        color,
    })
}

pub fn update(conn: &Connection, id: i64, payload: UpdateTagPayload) -> Result<Tag, AppError> {
    let mut tag = conn.query_row(
        "SELECT id, name, color FROM tags WHERE id = ?1",
        params![id],
        |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        },
    ).map_err(|_| AppError::NotFound(format!("Tag {} not found", id)))?;

    if let Some(name) = payload.name {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(AppError::Validation("Tag name cannot be empty".into()));
        }
        tag.name = trimmed.to_string();
    }
    if let Some(color) = payload.color {
        tag.color = color;
    }

    conn.execute(
        "UPDATE tags SET name = ?1, color = ?2 WHERE id = ?3",
        params![tag.name, tag.color, id],
    )?;

    Ok(tag)
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Tag {} not found", id)));
    }
    Ok(())
}
