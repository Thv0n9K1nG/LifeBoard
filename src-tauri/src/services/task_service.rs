use crate::error::AppError;
use crate::models::tag::Tag;
use crate::models::task::{
    CreateTaskPayload, GetTasksParams, Task, TaskListResponse, UpdateTaskPayload,
};
use chrono::Utc;
use rusqlite::{params, Connection};

fn get_tags_for_task(conn: &Connection, task_id: i64) -> Result<Vec<Tag>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color
         FROM tags t
         JOIN task_tags tt ON tt.tag_id = t.id
         WHERE tt.task_id = ?1
         ORDER BY t.name ASC",
    )?;
    let tags = stmt
        .query_map(params![task_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<Tag>, _>>()?;
    Ok(tags)
}

fn map_task_row(row: &rusqlite::Row) -> Result<Task, rusqlite::Error> {
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        priority: row.get(3)?,
        status: row.get(4)?,
        deadline: row.get(5)?,
        is_recurring: row.get::<_, i64>(6)? != 0,
        recurrence_type: row.get(7)?,
        recurrence_end_date: row.get(8)?,
        parent_task_id: row.get(9)?,
        planned_date: row.get(10)?,
        created_at: row.get(11)?,
        completed_at: row.get(12)?,
        updated_at: row.get(13)?,
        tags: Vec::new(),
    })
}

pub fn get_tasks(
    conn: &Connection,
    params: GetTasksParams,
) -> Result<TaskListResponse, AppError> {
    let mut sql = String::from(
        "SELECT id, title, description, priority, status, deadline, is_recurring,
                recurrence_type, recurrence_end_date, parent_task_id, planned_date,
                created_at, completed_at, updated_at
         FROM tasks WHERE 1=1",
    );
    let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(status) = params.status {
        if !status.is_empty() {
            sql.push_str(" AND status = ?");
            query_params.push(Box::new(status));
        }
    }
    if let Some(priority) = params.priority {
        if !priority.is_empty() {
            sql.push_str(" AND priority = ?");
            query_params.push(Box::new(priority));
        }
    }
    if let Some(planned_date) = params.planned_date {
        if !planned_date.is_empty() {
            // Match either ISO substring or date string
            let date_prefix = planned_date.split('T').next().unwrap_or(&planned_date);
            sql.push_str(" AND planned_date LIKE ?");
            query_params.push(Box::new(format!("{}%", date_prefix)));
        }
    }
    if let Some(search) = params.search {
        if !search.is_empty() {
            sql.push_str(" AND (title LIKE ? OR description LIKE ?)");
            let pattern = format!("%{}%", search);
            query_params.push(Box::new(pattern.clone()));
            query_params.push(Box::new(pattern));
        }
    }

    sql.push_str(" ORDER BY created_at DESC");

    let params_slice: Vec<&dyn rusqlite::ToSql> = query_params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql)?;
    let tasks = stmt
        .query_map(params_slice.as_slice(), map_task_row)?
        .collect::<Result<Vec<Task>, _>>()?;

    let total = tasks.len();

    // Pagination
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(50);
    let start = (page.saturating_sub(1)) * page_size;
    let paginated: Vec<Task> = tasks
        .into_iter()
        .skip(start)
        .take(page_size)
        .collect();

    // Populate tags
    let mut result_tasks = Vec::new();
    for mut task in paginated {
        task.tags = get_tags_for_task(conn, task.id)?;
        result_tasks.push(task);
    }

    Ok(TaskListResponse {
        data: result_tasks,
        total,
    })
}

pub fn get_by_id(conn: &Connection, id: i64) -> Result<Task, AppError> {
    let mut task = conn.query_row(
        "SELECT id, title, description, priority, status, deadline, is_recurring,
                recurrence_type, recurrence_end_date, parent_task_id, planned_date,
                created_at, completed_at, updated_at
         FROM tasks WHERE id = ?1",
        params![id],
        map_task_row,
    ).map_err(|_| AppError::NotFound(format!("Task {} not found", id)))?;

    task.tags = get_tags_for_task(conn, id)?;
    Ok(task)
}

pub fn create(conn: &Connection, payload: CreateTaskPayload) -> Result<Task, AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::Validation("Task title cannot be empty".into()));
    }
    let priority = payload.priority.unwrap_or_else(|| "medium".to_string());
    let now = Utc::now().to_rfc3339();
    let is_recurring = if payload.is_recurring.unwrap_or(false) { 1 } else { 0 };

    conn.execute(
        "INSERT INTO tasks (
            title, description, priority, status, deadline, is_recurring,
            recurrence_type, recurrence_end_date, parent_task_id, planned_date,
            created_at, completed_at, updated_at
        ) VALUES (?1, ?2, ?3, 'pending', ?4, ?5, ?6, ?7, NULL, ?8, ?9, NULL, ?10)",
        params![
            title,
            payload.description,
            priority,
            payload.deadline,
            is_recurring,
            payload.recurrence_type,
            payload.recurrence_end_date,
            payload.planned_date,
            now,
            now,
        ],
    )?;

    let task_id = conn.last_insert_rowid();

    if let Some(tag_ids) = payload.tag_ids {
        for tag_id in tag_ids {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![task_id, tag_id],
            );
        }
    }

    get_by_id(conn, task_id)
}

pub fn update(conn: &Connection, id: i64, payload: UpdateTaskPayload) -> Result<Task, AppError> {
    let existing = get_by_id(conn, id)?;
    let now = Utc::now().to_rfc3339();

    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.or(existing.description);
    let priority = payload.priority.unwrap_or(existing.priority);
    let deadline = payload.deadline.or(existing.deadline);
    let is_recurring = payload.is_recurring.unwrap_or(existing.is_recurring);
    let recurrence_type = payload.recurrence_type.or(existing.recurrence_type);
    let recurrence_end_date = payload.recurrence_end_date.or(existing.recurrence_end_date);
    let planned_date = payload.planned_date.or(existing.planned_date);

    conn.execute(
        "UPDATE tasks SET
            title = ?1, description = ?2, priority = ?3, deadline = ?4,
            is_recurring = ?5, recurrence_type = ?6, recurrence_end_date = ?7,
            planned_date = ?8, updated_at = ?9
         WHERE id = ?10",
        params![
            title,
            description,
            priority,
            deadline,
            if is_recurring { 1 } else { 0 },
            recurrence_type,
            recurrence_end_date,
            planned_date,
            now,
            id,
        ],
    )?;

    if let Some(tag_ids) = payload.tag_ids {
        conn.execute("DELETE FROM task_tags WHERE task_id = ?1", params![id])?;
        for tag_id in tag_ids {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
                params![id, tag_id],
            );
        }
    }

    get_by_id(conn, id)
}

pub fn complete(conn: &Connection, id: i64) -> Result<Task, AppError> {
    let existing = get_by_id(conn, id)?;
    let now = Utc::now().to_rfc3339();

    // Determine completion status: "done" vs "done_late"
    let status = if let Some(ref deadline) = existing.deadline {
        if now.as_str() > deadline.as_str() {
            "done_late"
        } else {
            "done"
        }
    } else if let Some(ref planned_date) = existing.planned_date {
        let today_date = now.split('T').next().unwrap_or("");
        let planned = planned_date.split('T').next().unwrap_or("");
        if today_date > planned {
            "done_late"
        } else {
            "done"
        }
    } else {
        "done"
    };

    conn.execute(
        "UPDATE tasks SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
        params![status, now, now, id],
    )?;

    get_by_id(conn, id)
}

pub fn uncomplete(conn: &Connection, id: i64) -> Result<Task, AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET status = 'pending', completed_at = NULL, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    get_by_id(conn, id)
}

pub fn archive(conn: &Connection, id: i64) -> Result<Task, AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET status = 'archived', updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Task {} not found", id)));
    }
    Ok(())
}

pub fn run_daily_scheduler(conn: &Connection) -> Result<(), AppError> {
    let today = Utc::now().format("%Y-%m-%d").to_string();

    let last_run: Option<String> = conn
        .query_row(
            "SELECT last_run_date FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(None);

    if let Some(ref lr) = last_run {
        if lr == &today {
            return Ok(()); // Already ran today
        }
    }

    // Process recurring tasks
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, recurrence_type, recurrence_end_date
         FROM tasks
         WHERE is_recurring = 1 AND parent_task_id IS NULL",
    )?;

    struct RecTask {
        id: i64,
        title: String,
        description: Option<String>,
        priority: String,
        recurrence_end_date: Option<String>,
    }

    let recurring = stmt
        .query_map([], |row| {
            Ok(RecTask {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                recurrence_end_date: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<RecTask>, _>>()?;

    let now_iso = Utc::now().to_rfc3339();

    for r in recurring {
        if let Some(ref end) = r.recurrence_end_date {
            if end < &today {
                continue; // recurrence ended
            }
        }

        // Check if today instance already exists
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE parent_task_id = ?1 AND planned_date LIKE ?2",
            params![r.id, format!("{}%", today)],
            |row| row.get(0),
        ).unwrap_or(0);

        if exists == 0 {
            conn.execute(
                "INSERT INTO tasks (
                    title, description, priority, status, deadline, is_recurring,
                    parent_task_id, planned_date, created_at, updated_at
                ) VALUES (?1, ?2, ?3, 'pending', NULL, 0, ?4, ?5, ?6, ?7)",
                params![
                    r.title,
                    r.description,
                    r.priority,
                    r.id,
                    today,
                    now_iso,
                    now_iso,
                ],
            )?;
        }
    }

    // Update settings last_run_date
    conn.execute(
        "UPDATE settings SET last_run_date = ?1, updated_at = ?2 WHERE id = 1",
        params![today, now_iso],
    )?;

    Ok(())
}
