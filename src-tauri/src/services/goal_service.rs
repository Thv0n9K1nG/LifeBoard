use crate::error::AppError;
use crate::models::goal::{CreateGoalPayload, Goal, UpdateGoalPayload};
use chrono::Utc;
use rusqlite::{params, Connection};

fn map_goal_row(row: &rusqlite::Row) -> Result<Goal, rusqlite::Error> {
    Ok(Goal {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        current_value: row.get(3)?,
        target_value: row.get(4)?,
        unit: row.get(5)?,
        deadline: row.get(6)?,
        status: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
    })
}

pub fn get_all(conn: &Connection, status: Option<String>) -> Result<Vec<Goal>, AppError> {
    let mut sql = String::from(
        "SELECT id, title, description, current_value, target_value, unit, deadline, status, created_at, updated_at
         FROM goals WHERE 1=1",
    );
    let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(s) = status {
        if !s.is_empty() {
            sql.push_str(" AND status = ?");
            query_params.push(Box::new(s));
        }
    }
    sql.push_str(" ORDER BY created_at DESC");

    let params_slice: Vec<&dyn rusqlite::ToSql> = query_params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let goals = stmt
        .query_map(params_slice.as_slice(), map_goal_row)?
        .collect::<Result<Vec<Goal>, _>>()?;
    Ok(goals)
}

pub fn get_by_id(conn: &Connection, id: i64) -> Result<Goal, AppError> {
    conn.query_row(
        "SELECT id, title, description, current_value, target_value, unit, deadline, status, created_at, updated_at
         FROM goals WHERE id = ?1",
        params![id],
        map_goal_row,
    ).map_err(|_| AppError::NotFound(format!("Goal {} not found", id)))
}

pub fn create(conn: &Connection, payload: CreateGoalPayload) -> Result<Goal, AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("Goal title cannot be empty".into()));
    }
    let current_value = payload.current_value.unwrap_or(0.0);
    let now = Utc::now().to_rfc3339();
    let status = if current_value >= payload.target_value {
        "completed"
    } else {
        "active"
    };

    conn.execute(
        "INSERT INTO goals (title, description, current_value, target_value, unit, deadline, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            title,
            payload.description,
            current_value,
            payload.target_value,
            payload.unit,
            payload.deadline,
            status,
            now,
            now,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn update(conn: &Connection, id: i64, payload: UpdateGoalPayload) -> Result<Goal, AppError> {
    let existing = get_by_id(conn, id)?;
    let now = Utc::now().to_rfc3339();

    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.or(existing.description);
    let current_value = payload.current_value.unwrap_or(existing.current_value);
    let target_value = payload.target_value.unwrap_or(existing.target_value);
    let unit = payload.unit.unwrap_or(existing.unit);
    let deadline = payload.deadline.or(existing.deadline);
    let status = payload.status.unwrap_or_else(|| {
        if current_value >= target_value {
            "completed".to_string()
        } else {
            existing.status
        }
    });

    conn.execute(
        "UPDATE goals SET
            title = ?1, description = ?2, current_value = ?3, target_value = ?4,
            unit = ?5, deadline = ?6, status = ?7, updated_at = ?8
         WHERE id = ?9",
        params![
            title,
            description,
            current_value,
            target_value,
            unit,
            deadline,
            status,
            now,
            id,
        ],
    )?;

    get_by_id(conn, id)
}

pub fn update_progress(conn: &Connection, id: i64, current_value: f64) -> Result<Goal, AppError> {
    let existing = get_by_id(conn, id)?;
    let now = Utc::now().to_rfc3339();
    let status = if current_value >= existing.target_value {
        "completed"
    } else if existing.status == "completed" {
        "active"
    } else {
        &existing.status
    };

    conn.execute(
        "UPDATE goals SET current_value = ?1, status = ?2, updated_at = ?3 WHERE id = ?4",
        params![current_value, status, now, id],
    )?;

    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM goals WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Goal {} not found", id)));
    }
    Ok(())
}
