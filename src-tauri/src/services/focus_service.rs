use crate::error::AppError;
use crate::models::focus::{
    CreateFocusSessionPayload, FocusSession, FocusSessionResponse, SplitRecord, StopwatchState,
    UpdateFocusSessionPayload,
};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

fn map_session_row(row: &rusqlite::Row) -> Result<FocusSession, rusqlite::Error> {
    Ok(FocusSession {
        id: row.get(0)?,
        session_type: row.get(1)?,
        label: row.get(2)?,
        start_time: row.get(3)?,
        end_time: row.get(4)?,
        duration_seconds: row.get(5)?,
        session_date: row.get(6)?,
        splits: row.get(7)?,
        stopwatch_state: row.get(8)?,
        paused_duration_seconds: row.get(9)?,
        created_at: row.get(10)?,
    })
}

pub fn get_sessions(
    conn: &Connection,
    from: &str,
    to: &str,
    session_type: Option<String>,
) -> Result<FocusSessionResponse, AppError> {
    let clean_from = from.split('T').next().unwrap_or(from);
    let clean_to = to.split('T').next().unwrap_or(to);

    let mut sql = String::from(
        "SELECT id, session_type, label, start_time, end_time, duration_seconds,
                session_date, splits, stopwatch_state, paused_duration_seconds, created_at
         FROM focus_sessions
         WHERE session_date BETWEEN ?1 AND ?2",
    );
    let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = vec![
        Box::new(clean_from.to_string()),
        Box::new(clean_to.to_string()),
    ];

    if let Some(st) = session_type {
        if !st.is_empty() {
            sql.push_str(" AND session_type = ?");
            query_params.push(Box::new(st));
        }
    }
    sql.push_str(" ORDER BY start_time DESC");

    let params_slice: Vec<&dyn rusqlite::ToSql> = query_params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let sessions = stmt
        .query_map(params_slice.as_slice(), map_session_row)?
        .collect::<Result<Vec<FocusSession>, _>>()?;

    let total_duration_seconds: i64 = sessions.iter().map(|s| s.duration_seconds).sum();

    Ok(FocusSessionResponse {
        data: sessions,
        total_duration_seconds,
    })
}

pub fn create_session(
    conn: &Connection,
    payload: CreateFocusSessionPayload,
) -> Result<FocusSession, AppError> {
    let now = Utc::now().to_rfc3339();
    let duration = if let Some(ref end) = payload.end_time {
        if let (Ok(s), Ok(e)) = (
            DateTime::parse_from_rfc3339(&payload.start_time),
            DateTime::parse_from_rfc3339(end),
        ) {
            let diff = (e - s).num_seconds();
            if diff > 0 { diff } else { 0 }
        } else {
            0
        }
    } else {
        0
    };

    let session_date = payload.session_date.unwrap_or_else(|| {
        payload.start_time.split('T').next().unwrap_or("").to_string()
    });

    conn.execute(
        "INSERT INTO focus_sessions (
            session_type, label, start_time, end_time, duration_seconds,
            session_date, splits, stopwatch_state, paused_duration_seconds,
            current_segment_start, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'stopped', 0, NULL, ?8)",
        params![
            payload.session_type,
            payload.label,
            payload.start_time,
            payload.end_time,
            duration,
            session_date,
            payload.splits,
            now,
        ],
    )?;

    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, session_type, label, start_time, end_time, duration_seconds,
                session_date, splits, stopwatch_state, paused_duration_seconds, created_at
         FROM focus_sessions WHERE id = ?1",
        params![id],
        map_session_row,
    ).map_err(|e| AppError::Database(e))
}

pub fn update_session(
    conn: &Connection,
    id: i64,
    payload: UpdateFocusSessionPayload,
) -> Result<FocusSession, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, session_type, label, start_time, end_time, duration_seconds,
                session_date, splits, stopwatch_state, paused_duration_seconds, created_at
         FROM focus_sessions WHERE id = ?1",
    )?;
    let mut session = stmt
        .query_row(params![id], map_session_row)
        .map_err(|_| AppError::NotFound(format!("Focus session {} not found", id)))?;

    if let Some(st) = payload.session_type {
        session.session_type = st;
    }
    if let Some(l) = payload.label {
        session.label = Some(l);
    }
    if let Some(st) = payload.start_time {
        session.start_time = st;
    }
    if let Some(et) = payload.end_time {
        session.end_time = Some(et);
    }
    if let Some(sd) = payload.session_date {
        session.session_date = sd;
    }
    if let Some(sp) = payload.splits {
        session.splits = Some(sp);
    }

    conn.execute(
        "UPDATE focus_sessions SET session_type = ?1, label = ?2, start_time = ?3,
                end_time = ?4, session_date = ?5, splits = ?6
         WHERE id = ?7",
        params![
            session.session_type,
            session.label,
            session.start_time,
            session.end_time,
            session.session_date,
            session.splits,
            id,
        ],
    )?;

    Ok(session)
}

pub fn delete_session(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM focus_sessions WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Focus session {} not found", id)));
    }
    Ok(())
}

// ── Stopwatch Engine ───────────────────────────────────────────────────────

fn calc_current_active_seconds(
    state: &str,
    current_segment_start: Option<&str>,
    paused_duration_seconds: i64,
) -> i64 {
    if state == "running" {
        if let Some(start_str) = current_segment_start {
            if let Ok(seg_start) = DateTime::parse_from_rfc3339(start_str) {
                let now = Utc::now();
                let seg_sec = (now.signed_duration_since(seg_start.with_timezone(&Utc))).num_seconds();
                return paused_duration_seconds + if seg_sec > 0 { seg_sec } else { 0 };
            }
        }
    }
    paused_duration_seconds
}

fn format_time(total_sec: i64) -> String {
    let h = total_sec / 3600;
    let m = (total_sec % 3600) / 60;
    let s = total_sec % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn parse_seconds_from_formatted(fmt: &str) -> i64 {
    let parts: Vec<&str> = fmt.split(':').collect();
    if parts.len() == 3 {
        if let (Ok(h), Ok(m), Ok(s)) = (
            parts[0].parse::<i64>(),
            parts[1].parse::<i64>(),
            parts[2].parse::<i64>(),
        ) {
            return h * 3600 + m * 60 + s;
        }
    }
    0
}

fn append_split(
    splits_json: Option<String>,
    total_elapsed_sec: i64,
    note: Option<String>,
) -> String {
    let mut splits: Vec<SplitRecord> = match splits_json {
        Some(s) if !s.is_empty() => serde_json::from_str(&s).unwrap_or_default(),
        _ => Vec::new(),
    };

    let prev_elapsed_sec = if let Some(last) = splits.last() {
        if last.elapsed_sec > 0 {
            last.elapsed_sec
        } else {
            parse_seconds_from_formatted(&last.elapsed)
        }
    } else {
        0
    };

    let lap_sec = (total_elapsed_sec - prev_elapsed_sec).max(0);
    let next_id = (splits.len() + 1) as i64;
    splits.push(SplitRecord {
        id: next_id,
        interval: format_time(lap_sec),
        elapsed: format_time(total_elapsed_sec),
        elapsed_sec: total_elapsed_sec,
        timestamp: Utc::now().to_rfc3339(),
        note,
    });

    serde_json::to_string(&splits).unwrap_or_else(|_| "[]".to_string())
}

pub fn get_stopwatch_state(conn: &Connection) -> Result<Option<StopwatchState>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, stopwatch_state, label, current_segment_start, paused_duration_seconds,
                duration_seconds, splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state IN ('running', 'paused')
         ORDER BY id DESC LIMIT 1",
    )?;

    let result = stmt.query_row([], |row| {
        let id: i64 = row.get(0)?;
        let state: String = row.get(1)?;
        let label: Option<String> = row.get(2)?;
        let current_segment_start: Option<String> = row.get(3)?;
        let paused_dur: i64 = row.get(4)?;
        let dur: i64 = row.get(5)?;
        let splits: Option<String> = row.get(6)?;
        let session_date: String = row.get(7)?;

        let accumulated = if state == "running" {
            paused_dur
        } else if state == "paused" {
            paused_dur
        } else {
            dur
        };

        Ok(StopwatchState {
            id,
            state: state.clone(),
            label,
            segment_started_at: if state == "running" {
                current_segment_start
            } else {
                None
            },
            accumulated_seconds: accumulated,
            splits,
            session_date,
        })
    });

    match result {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

pub fn start_stopwatch(
    conn: &Connection,
    label: Option<String>,
    session_date: Option<String>,
) -> Result<StopwatchState, AppError> {
    // Finalize any lingering active session cleanly
    if let Ok(Some(_active)) = get_stopwatch_state(conn) {
        let _ = stop_stopwatch(conn);
    }

    let now = Utc::now().to_rfc3339();
    let s_date = session_date.unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    conn.execute(
        "INSERT INTO focus_sessions (
            session_type, label, start_time, end_time, duration_seconds,
            session_date, splits, stopwatch_state, paused_duration_seconds,
            current_segment_start, created_at
        ) VALUES ('stopwatch', ?1, ?2, NULL, 0, ?3, '[]', 'running', 0, ?4, ?5)",
        params![label, now, s_date, now, now],
    )?;

    let id = conn.last_insert_rowid();
    Ok(StopwatchState {
        id,
        state: "running".to_string(),
        label,
        segment_started_at: Some(now),
        accumulated_seconds: 0,
        splits: Some("[]".to_string()),
        session_date: s_date,
    })
}

pub fn pause_stopwatch(conn: &Connection) -> Result<StopwatchState, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, stopwatch_state, label, current_segment_start, paused_duration_seconds,
                splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state = 'running'
         ORDER BY id DESC LIMIT 1",
    )?;

    let (id, label, current_segment_start, paused_dur, splits, session_date) = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
            ))
        })
        .map_err(|_| AppError::Validation("No running stopwatch session to pause".into()))?;

    let total_active = calc_current_active_seconds("running", current_segment_start.as_deref(), paused_dur);
    let new_splits = append_split(splits, total_active, Some("Tạm dừng".to_string()));

    conn.execute(
        "UPDATE focus_sessions SET
            stopwatch_state = 'paused',
            paused_duration_seconds = ?1,
            current_segment_start = NULL,
            splits = ?2
         WHERE id = ?3",
        params![total_active, new_splits, id],
    )?;

    Ok(StopwatchState {
        id,
        state: "paused".to_string(),
        label,
        segment_started_at: None,
        accumulated_seconds: total_active,
        splits: Some(new_splits),
        session_date,
    })
}

pub fn resume_stopwatch(conn: &Connection) -> Result<StopwatchState, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, label, paused_duration_seconds, splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state = 'paused'
         ORDER BY id DESC LIMIT 1",
    )?;

    let (id, label, paused_dur, splits, session_date) = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(|_| AppError::Validation("No paused stopwatch session to resume".into()))?;

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE focus_sessions SET
            stopwatch_state = 'running',
            current_segment_start = ?1
         WHERE id = ?2",
        params![now, id],
    )?;

    Ok(StopwatchState {
        id,
        state: "running".to_string(),
        label,
        segment_started_at: Some(now),
        accumulated_seconds: paused_dur,
        splits,
        session_date,
    })
}

pub fn stop_stopwatch(conn: &Connection) -> Result<StopwatchState, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, stopwatch_state, label, current_segment_start, paused_duration_seconds,
                splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state IN ('running', 'paused')
         ORDER BY id DESC LIMIT 1",
    )?;

    let (id, state, label, current_segment_start, paused_dur, splits, session_date) = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
            ))
        })
        .map_err(|_| AppError::Validation("No active stopwatch session to stop".into()))?;

    let total_active = calc_current_active_seconds(&state, current_segment_start.as_deref(), paused_dur);
    let now = Utc::now().to_rfc3339();

    let final_splits = if state == "running" {
        append_split(splits, total_active, Some("Reset".to_string()))
    } else {
        splits.unwrap_or_else(|| "[]".to_string())
    };

    conn.execute(
        "UPDATE focus_sessions SET
            stopwatch_state = 'stopped',
            end_time = ?1,
            duration_seconds = ?2,
            current_segment_start = NULL,
            splits = ?3
         WHERE id = ?4",
        params![now, total_active, final_splits, id],
    )?;

    Ok(StopwatchState {
        id,
        state: "stopped".to_string(),
        label,
        segment_started_at: None,
        accumulated_seconds: total_active,
        splits: Some(final_splits),
        session_date,
    })
}

pub fn split_stopwatch(
    conn: &Connection,
    note: Option<String>,
) -> Result<StopwatchState, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, label, current_segment_start, paused_duration_seconds, splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state = 'running'
         ORDER BY id DESC LIMIT 1",
    )?;

    let (id, label, current_segment_start, paused_dur, splits, session_date) = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(|_| AppError::Validation("Can only split when stopwatch is running".into()))?;

    let total_active = calc_current_active_seconds("running", current_segment_start.as_deref(), paused_dur);
    let new_splits = append_split(splits, total_active, note);

    conn.execute(
        "UPDATE focus_sessions SET splits = ?1 WHERE id = ?2",
        params![new_splits, id],
    )?;

    Ok(StopwatchState {
        id,
        state: "running".to_string(),
        label,
        segment_started_at: current_segment_start,
        accumulated_seconds: paused_dur,
        splits: Some(new_splits),
        session_date,
    })
}

pub fn update_stopwatch_label(
    conn: &Connection,
    label: &str,
) -> Result<StopwatchState, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, stopwatch_state, current_segment_start, paused_duration_seconds,
                splits, session_date
         FROM focus_sessions
         WHERE session_type = 'stopwatch' AND stopwatch_state IN ('running', 'paused')
         ORDER BY id DESC LIMIT 1",
    )?;

    let (id, state, current_segment_start, paused_dur, splits, session_date) = stmt
        .query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(|_| AppError::Validation("No active stopwatch session".into()))?;

    let trimmed = label.trim().to_string();
    conn.execute(
        "UPDATE focus_sessions SET label = ?1 WHERE id = ?2",
        params![trimmed, id],
    )?;

    Ok(StopwatchState {
        id,
        state,
        label: Some(trimmed),
        segment_started_at: current_segment_start,
        accumulated_seconds: paused_dur,
        splits,
        session_date,
    })
}
