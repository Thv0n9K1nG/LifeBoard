use crate::error::AppError;
use crate::models::analytics::*;
use chrono::{Datelike, Duration, NaiveDate, Utc};
use rusqlite::{params, Connection};
use std::collections::{HashMap, HashSet};

pub fn get_weekly(conn: &Connection, week_start: &str) -> Result<WeeklyAnalytics, AppError> {
    let clean_start = week_start.split('T').next().unwrap_or(week_start);
    let start_date = NaiveDate::parse_from_str(clean_start, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid week_start date format".into()))?;
    let end_date = start_date + Duration::days(6);
    let from = start_date.format("%Y-%m-%d").to_string();
    let to = end_date.format("%Y-%m-%d").to_string();

    // 1. Tasks per day
    let mut task_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', planned_date) AS day,
                SUM(CASE WHEN status IN ('done', 'done_late') THEN 1 ELSE 0 END) AS completed,
                COUNT(*) AS total
         FROM tasks
         WHERE planned_date BETWEEN ?1 AND ?2 AND status != 'archived'
         GROUP BY day",
    )?;
    let mut task_map: HashMap<String, (i64, i64)> = HashMap::new();
    let task_rows = task_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    for r in task_rows {
        let (day, comp, tot) = r?;
        if !day.is_empty() {
            task_map.insert(day, (comp, tot));
        }
    }

    // 2. Focus minutes per day
    let mut focus_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', session_date) AS day,
                CAST(ROUND(SUM(duration_seconds) / 60.0) AS INTEGER) AS minutes
         FROM focus_sessions
         WHERE session_date BETWEEN ?1 AND ?2
         GROUP BY day",
    )?;
    let mut focus_map: HashMap<String, i64> = HashMap::new();
    let focus_rows = focus_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in focus_rows {
        let (day, mins) = r?;
        if !day.is_empty() {
            focus_map.insert(day, mins);
        }
    }

    // 3. Habits per day
    let mut habit_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', hc.checkin_date) AS day,
                SUM(hc.is_completed) AS done,
                COUNT(*) AS total
         FROM habit_checkins hc
         JOIN habits h ON h.id = hc.habit_id AND h.is_active = 1
         WHERE hc.checkin_date BETWEEN ?1 AND ?2
         GROUP BY day",
    )?;
    let mut habit_map: HashMap<String, (i64, i64)> = HashMap::new();
    let habit_rows = habit_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    for r in habit_rows {
        let (day, done, tot) = r?;
        if !day.is_empty() {
            habit_map.insert(day, (done, tot));
        }
    }

    // 4. Mood per day
    let mut mood_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', entry_date) AS day, AVG(score) AS score
         FROM mood_entries
         WHERE entry_date BETWEEN ?1 AND ?2
         GROUP BY day",
    )?;
    let mut mood_map: HashMap<String, f64> = HashMap::new();
    let mood_rows = mood_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, f64>(1)?,
        ))
    })?;
    for r in mood_rows {
        let (day, score) = r?;
        if !day.is_empty() {
            mood_map.insert(day, score);
        }
    }

    let mut days = Vec::new();
    let mut total_tasks = 0;
    let mut total_completed = 0;
    let mut total_focus_minutes = 0;
    let mut mood_scores = Vec::new();

    for i in 0..7 {
        let d = (start_date + Duration::days(i)).format("%Y-%m-%d").to_string();
        let (comp, tot) = task_map.get(&d).cloned().unwrap_or((0, 0));
        let focus_mins = focus_map.get(&d).cloned().unwrap_or(0);
        let (h_done, h_tot) = habit_map.get(&d).cloned().unwrap_or((0, 0));
        let m_score = mood_map.get(&d).cloned();

        total_tasks += tot;
        total_completed += comp;
        total_focus_minutes += focus_mins;
        if let Some(s) = m_score {
            mood_scores.push(s);
        }

        days.push(DayAnalytics {
            date: d,
            completed_tasks: comp,
            focus_minutes: focus_mins,
            habits_completed: h_done,
            habits_total: h_tot,
            mood_score: m_score.map(|s| (s * 10.0).round() / 10.0),
        });
    }

    let completion_rate = if total_tasks > 0 {
        ((total_completed as f64 / total_tasks as f64) * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    let avg_mood = if !mood_scores.is_empty() {
        let sum: f64 = mood_scores.iter().sum();
        ((sum / mood_scores.len() as f64) * 10.0).round() / 10.0
    } else {
        0.0
    };

    Ok(WeeklyAnalytics {
        week_start: from,
        week_end: to,
        days,
        summary: WeeklySummary {
            total_completed_tasks: total_completed,
            total_focus_minutes,
            completion_rate,
            avg_mood_score: avg_mood,
        },
    })
}

pub fn get_monthly(conn: &Connection, year: i32, month: u32) -> Result<MonthlyAnalytics, AppError> {
    let start_date = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| AppError::Validation("Invalid year/month".into()))?;

    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    let end_date = next_month - Duration::days(1);

    let from = start_date.format("%Y-%m-%d").to_string();
    let to = end_date.format("%Y-%m-%d").to_string();

    let mut task_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', planned_date) AS day,
                SUM(CASE WHEN status IN ('done', 'done_late') THEN 1 ELSE 0 END) AS completed,
                COUNT(*) AS total
         FROM tasks
         WHERE planned_date BETWEEN ?1 AND ?2 AND status != 'archived'
         GROUP BY day",
    )?;
    let mut task_map: HashMap<String, (i64, i64)> = HashMap::new();
    let task_rows = task_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    for r in task_rows {
        let (day, comp, tot) = r?;
        if !day.is_empty() {
            task_map.insert(day, (comp, tot));
        }
    }

    let mut focus_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', session_date) AS day,
                CAST(ROUND(SUM(duration_seconds) / 60.0) AS INTEGER) AS minutes
         FROM focus_sessions
         WHERE session_date BETWEEN ?1 AND ?2
         GROUP BY day",
    )?;
    let mut focus_map: HashMap<String, i64> = HashMap::new();
    let focus_rows = focus_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in focus_rows {
        let (day, mins) = r?;
        if !day.is_empty() {
            focus_map.insert(day, mins);
        }
    }

    let mut habit_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', checkin_date) AS day, SUM(is_completed) AS done
         FROM habit_checkins
         WHERE checkin_date BETWEEN ?1 AND ?2
         GROUP BY day",
    )?;
    let mut habit_map: HashMap<String, i64> = HashMap::new();
    let habit_rows = habit_stmt.query_map(params![from, to], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in habit_rows {
        let (day, done) = r?;
        if !day.is_empty() {
            habit_map.insert(day, done);
        }
    }

    let days_in_month = end_date.day();
    let mut days = Vec::new();
    let mut total_completed = 0;
    let mut total_all = 0;
    let mut total_focus_minutes = 0;

    for i in 1..=days_in_month {
        let d = format!("{:04}-{:02}-{:02}", year, month, i);
        let (comp, tot) = task_map.get(&d).cloned().unwrap_or((0, 0));
        let focus_mins = focus_map.get(&d).cloned().unwrap_or(0);
        let h_done = habit_map.get(&d).cloned().unwrap_or(0);

        total_completed += comp;
        total_all += tot;
        total_focus_minutes += focus_mins;

        days.push(DayAnalytics {
            date: d,
            completed_tasks: comp,
            focus_minutes: focus_mins,
            habits_completed: h_done,
            habits_total: 0,
            mood_score: None,
        });
    }

    let completion_rate = if total_all > 0 {
        ((total_completed as f64 / total_all as f64) * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    let total_focus_hours = ((total_focus_minutes as f64 / 60.0) * 10.0).round() / 10.0;

    Ok(MonthlyAnalytics {
        year,
        month,
        days,
        summary: MonthlySummary {
            total_completed_tasks: total_completed,
            total_focus_hours,
            completion_rate,
        },
    })
}

pub fn get_yearly(conn: &Connection, year: i32) -> Result<YearlyAnalytics, AppError> {
    let year_str = year.to_string();

    let mut task_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', completed_at) AS day, COUNT(*) AS count
         FROM tasks
         WHERE strftime('%Y', completed_at) = ?1 AND status IN ('done', 'done_late')
         GROUP BY day",
    )?;
    let mut task_map: HashMap<String, i64> = HashMap::new();
    let task_rows = task_stmt.query_map(params![year_str], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in task_rows {
        let (day, count) = r?;
        if !day.is_empty() {
            task_map.insert(day, count);
        }
    }

    let mut focus_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', session_date) AS day,
                CAST(ROUND(SUM(duration_seconds) / 60.0) AS INTEGER) AS minutes
         FROM focus_sessions
         WHERE strftime('%Y', session_date) = ?1
         GROUP BY day",
    )?;
    let mut focus_map: HashMap<String, i64> = HashMap::new();
    let focus_rows = focus_stmt.query_map(params![year_str], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in focus_rows {
        let (day, mins) = r?;
        if !day.is_empty() {
            focus_map.insert(day, mins);
        }
    }

    let mut habit_stmt = conn.prepare(
        "SELECT strftime('%Y-%m-%d', checkin_date) AS day, SUM(is_completed) AS done
         FROM habit_checkins
         WHERE strftime('%Y', checkin_date) = ?1 AND is_completed = 1
         GROUP BY day",
    )?;
    let mut habit_map: HashMap<String, i64> = HashMap::new();
    let habit_rows = habit_stmt.query_map(params![year_str], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?.unwrap_or_default(),
            row.get::<_, i64>(1)?,
        ))
    })?;
    for r in habit_rows {
        let (day, done) = r?;
        if !day.is_empty() {
            habit_map.insert(day, done);
        }
    }

    let mut all_dates: HashSet<String> = HashSet::new();
    all_dates.extend(task_map.keys().cloned());
    all_dates.extend(focus_map.keys().cloned());
    all_dates.extend(habit_map.keys().cloned());

    let mut heatmap: Vec<HeatmapDay> = all_dates
        .into_iter()
        .map(|date| {
            let tasks = task_map.get(&date).cloned().unwrap_or(0);
            let focus = focus_map.get(&date).cloned().unwrap_or(0);
            let habits = habit_map.get(&date).cloned().unwrap_or(0);
            let score = tasks + (focus / 30) + habits;
            let intensity = if score == 0 {
                0
            } else if score <= 2 {
                1
            } else if score <= 5 {
                2
            } else if score <= 10 {
                3
            } else {
                4
            };
            HeatmapDay {
                date,
                task_count: tasks,
                focus_minutes: focus,
                habit_done: habits,
                intensity_level: intensity,
            }
        })
        .collect();

    heatmap.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(YearlyAnalytics { year, heatmap })
}

pub fn get_statistics(conn: &Connection, from: &str, to: &str) -> Result<Statistics, AppError> {
    let clean_from = from.split('T').next().unwrap_or(from);
    let clean_to = to.split('T').next().unwrap_or(to);

    // Tasks stats
    let mut task_stmt = conn.prepare(
        "SELECT priority, status FROM tasks
         WHERE planned_date BETWEEN ?1 AND ?2 AND status != 'archived'",
    )?;
    let task_rows = task_stmt.query_map(params![clean_from, clean_to], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    let mut total_tasks = 0;
    let mut completed_tasks = 0;
    let mut by_priority: HashMap<String, i64> = HashMap::from([
        ("high".to_string(), 0),
        ("medium".to_string(), 0),
        ("low".to_string(), 0),
    ]);

    for r in task_rows {
        let (priority, status) = r?;
        total_tasks += 1;
        if status == "done" || status == "done_late" {
            completed_tasks += 1;
        }
        *by_priority.entry(priority).or_insert(0) += 1;
    }

    let completion_rate = if total_tasks > 0 {
        ((completed_tasks as f64 / total_tasks as f64) * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    // Focus stats
    let mut focus_stmt = conn.prepare(
        "SELECT session_type, duration_seconds FROM focus_sessions
         WHERE session_date BETWEEN ?1 AND ?2",
    )?;
    let focus_rows = focus_stmt.query_map(params![clean_from, clean_to], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
        ))
    })?;

    let mut total_sessions = 0;
    let mut total_seconds = 0;
    let mut by_type: HashMap<String, i64> = HashMap::from([
        ("stopwatch".to_string(), 0),
        ("pomodoro".to_string(), 0),
    ]);

    for r in focus_rows {
        let (stype, dur) = r?;
        total_sessions += 1;
        total_seconds += dur;
        *by_type.entry(stype).or_insert(0) += 1;
    }

    let total_minutes = total_seconds / 60;
    let days_count = if let (Ok(f_date), Ok(t_date)) = (
        NaiveDate::parse_from_str(clean_from, "%Y-%m-%d"),
        NaiveDate::parse_from_str(clean_to, "%Y-%m-%d"),
    ) {
        ((t_date - f_date).num_days() + 1).max(1)
    } else {
        1
    };

    let avg_minutes_per_day = ((total_minutes as f64 / days_count as f64) * 10.0).round() / 10.0;

    // Mood stats
    let mut mood_stmt = conn.prepare(
        "SELECT score FROM mood_entries WHERE entry_date BETWEEN ?1 AND ?2",
    )?;
    let mood_rows = mood_stmt.query_map(params![clean_from, clean_to], |row| {
        Ok(row.get::<_, i64>(0)?)
    })?;

    let mut mood_distribution: HashMap<String, i64> = HashMap::from([
        ("1".to_string(), 0),
        ("2".to_string(), 0),
        ("3".to_string(), 0),
        ("4".to_string(), 0),
        ("5".to_string(), 0),
    ]);
    let mut mood_sum = 0;
    let mut mood_count = 0;

    for r in mood_rows {
        let score = r?;
        mood_sum += score;
        mood_count += 1;
        *mood_distribution.entry(score.to_string()).or_insert(0) += 1;
    }

    let avg_mood = if mood_count > 0 {
        ((mood_sum as f64 / mood_count as f64) * 10.0).round() / 10.0
    } else {
        0.0
    };

    // Habits longest streak
    let mut habit_stmt = conn.prepare(
        "SELECT id, name FROM habits WHERE is_active = 1",
    )?;
    let habits = habit_stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut longest_streak_val = 0;
    let mut longest_streak_name = String::new();

    let _today = Utc::now().date_naive();
    for h in habits {
        let (hid, hname) = h?;
        let (streak, _) = crate::services::habit_service::get_all(conn)?
            .into_iter()
            .find(|x| x.id == hid)
            .map(|x| (x.streak, ()))
            .unwrap_or((0, ()));
        if streak > longest_streak_val {
            longest_streak_val = streak;
            longest_streak_name = hname;
        }
    }

    let longest_streak_opt = if longest_streak_val > 0 {
        Some(LongestStreakDto {
            habit_name: longest_streak_name,
            streak: longest_streak_val,
        })
    } else {
        None
    };

    Ok(Statistics {
        period: PeriodDto {
            from: clean_from.to_string(),
            to: clean_to.to_string(),
        },
        tasks: TaskStatsDto {
            total: total_tasks,
            completed: completed_tasks,
            completion_rate,
            by_priority,
        },
        focus: FocusStatsDto {
            total_sessions,
            total_minutes,
            avg_minutes_per_day,
            by_type,
        },
        habits: HabitStatsDto {
            longest_streak: longest_streak_opt,
            avg_completion_rate: 0.0,
        },
        mood: MoodStatsDto {
            avg_score: avg_mood,
            distribution: mood_distribution,
        },
    })
}
