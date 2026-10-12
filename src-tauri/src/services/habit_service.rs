use crate::error::AppError;
use crate::models::habit::{CreateHabitPayload, Habit, UpdateHabitPayload};
use chrono::{Duration, NaiveDate, Utc};
use rusqlite::{params, Connection};

fn calculate_streak(
    conn: &Connection,
    habit_id: i64,
    today_date: NaiveDate,
    grace_period: i64,
) -> (i64, bool) {
    // Get all completed checkins within the last 60 days
    let start_date = today_date - Duration::days(60);
    let mut stmt = match conn.prepare(
        "SELECT checkin_date, is_completed
         FROM habit_checkins
         WHERE habit_id = ?1 AND checkin_date >= ?2
         ORDER BY checkin_date DESC",
    ) {
        Ok(s) => s,
        Err(_) => return (0, false),
    };

    struct Checkin {
        date: NaiveDate,
        completed: bool,
    }

    let checkins: Vec<Checkin> = stmt
        .query_map(params![habit_id, start_date.format("%Y-%m-%d").to_string()], |row| {
            let date_str: String = row.get(0)?;
            let is_comp: i64 = row.get(1)?;
            let parsed = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or(today_date);
            Ok(Checkin {
                date: parsed,
                completed: is_comp != 0,
            })
        })
        .and_then(|mapped| mapped.collect::<Result<Vec<Checkin>, _>>())
        .unwrap_or_default();

    let checked_in_today = checkins
        .iter()
        .any(|c| c.date == today_date && c.completed);

    if checkins.is_empty() {
        return (0, checked_in_today);
    }

    let mut streak = 0;
    let mut current_target = if checked_in_today {
        today_date
    } else {
        today_date - Duration::days(1)
    };

    let mut missed_in_streak = 0;

    loop {
        let has_checkin = checkins.iter().any(|c| c.date == current_target && c.completed);
        if has_checkin {
            streak += 1;
            missed_in_streak = 0;
            current_target -= Duration::days(1);
        } else {
            if missed_in_streak < grace_period {
                missed_in_streak += 1;
                current_target -= Duration::days(1);
            } else {
                break;
            }
        }
    }

    (streak, checked_in_today)
}

pub fn get_all(conn: &Connection) -> Result<Vec<Habit>, AppError> {
    let today = Utc::now().date_naive();
    let grace_period: i64 = conn
        .query_row(
            "SELECT habit_grace_period_days FROM settings WHERE id = 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let mut stmt = conn.prepare(
        "SELECT id, name, description, frequency, icon, color, is_active, created_at
         FROM habits
         WHERE is_active = 1
         ORDER BY id ASC",
    )?;

    let raw_habits = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, String>(7)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    let mut habits = Vec::new();
    for (id, name, desc, freq, icon, color, is_active, created_at) in raw_habits {
        let (streak, checked_today) = calculate_streak(conn, id, today, grace_period);
        habits.push(Habit {
            id,
            name,
            description: desc,
            frequency: freq,
            icon,
            color,
            is_active,
            created_at,
            streak,
            checked_in_today: checked_today,
        });
    }

    Ok(habits)
}

pub fn create(conn: &Connection, payload: CreateHabitPayload) -> Result<Habit, AppError> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("Habit name cannot be empty".into()));
    }
    let freq = payload.frequency.unwrap_or_else(|| "daily".to_string());
    let color = payload.color.unwrap_or_else(|| "#6B7280".to_string());
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO habits (name, description, frequency, icon, color, is_active, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6)",
        params![name, payload.description, freq, payload.icon, color, now],
    )?;

    let id = conn.last_insert_rowid();
    Ok(Habit {
        id,
        name: name.to_string(),
        description: payload.description,
        frequency: freq,
        icon: payload.icon,
        color,
        is_active: true,
        created_at: now,
        streak: 0,
        checked_in_today: false,
    })
}

pub fn update(conn: &Connection, id: i64, payload: UpdateHabitPayload) -> Result<Habit, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, frequency, icon, color, is_active, created_at
         FROM habits WHERE id = ?1",
    )?;

    let (hid, mut name, mut desc, mut freq, mut icon, mut color, mut is_active, created_at) = stmt
        .query_row(params![id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)? != 0,
                row.get::<_, String>(7)?,
            ))
        })
        .map_err(|_| AppError::NotFound(format!("Habit {} not found", id)))?;

    if let Some(n) = payload.name {
        let trimmed = n.trim();
        if trimmed.is_empty() {
            return Err(AppError::Validation("Habit name cannot be empty".into()));
        }
        name = trimmed.to_string();
    }
    if let Some(d) = payload.description {
        desc = Some(d);
    }
    if let Some(f) = payload.frequency {
        freq = f;
    }
    if let Some(ic) = payload.icon {
        icon = Some(ic);
    }
    if let Some(c) = payload.color {
        color = c;
    }
    if let Some(act) = payload.is_active {
        is_active = act;
    }

    conn.execute(
        "UPDATE habits SET name = ?1, description = ?2, frequency = ?3, icon = ?4, color = ?5, is_active = ?6
         WHERE id = ?7",
        params![name, desc, freq, icon, color, if is_active { 1 } else { 0 }, id],
    )?;

    let today = Utc::now().date_naive();
    let (streak, checked_today) = calculate_streak(conn, hid, today, 0);

    Ok(Habit {
        id: hid,
        name,
        description: desc,
        frequency: freq,
        icon,
        color,
        is_active,
        created_at,
        streak,
        checked_in_today: checked_today,
    })
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM habits WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("Habit {} not found", id)));
    }
    Ok(())
}

pub fn upsert_checkin(
    conn: &Connection,
    id: i64,
    date: &str,
    is_completed: bool,
) -> Result<(), AppError> {
    let clean_date = date.split('T').next().unwrap_or(date);
    conn.execute(
        "INSERT INTO habit_checkins (habit_id, checkin_date, is_completed)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(habit_id, checkin_date)
         DO UPDATE SET is_completed = ?3",
        params![id, clean_date, if is_completed { 1 } else { 0 }],
    )?;
    Ok(())
}
