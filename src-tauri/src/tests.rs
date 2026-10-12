#[cfg(test)]
mod tests {
    use crate::database::init_in_memory;
    use crate::models::goal::CreateGoalPayload;
    use crate::models::habit::CreateHabitPayload;
    use crate::models::mood::UpsertMoodPayload;
    use crate::models::note::UpsertDailyNotePayload;
    use crate::models::tag::CreateTagPayload;
    use crate::models::task::{CreateTaskPayload, GetTasksParams};
    use crate::services::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_database_initialization() {
        let db = init_in_memory().expect("Should initialize in-memory DB");
        let conn = db.conn.lock().unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM settings", [], |r| r.get(0))
            .expect("Should query settings");
        assert_eq!(count, 1, "Default settings row should be seeded");
    }

    #[test]
    fn test_tags_and_tasks_workflow() {
        let db = init_in_memory().expect("DB init");
        let conn = db.conn.lock().unwrap();

        // 1. Create Tag
        let tag = tag_service::create(
            &conn,
            CreateTagPayload {
                name: "Work".into(),
                color: Some("#3B82F6".into()),
            },
        )
        .expect("Create tag");
        assert_eq!(tag.name, "Work");

        // 2. Create Task with Tag
        let task = task_service::create(
            &conn,
            CreateTaskPayload {
                title: "Finish Report".into(),
                description: Some("Important quarterly report".into()),
                priority: Some("high".into()),
                deadline: Some((Utc::now() + Duration::days(2)).to_rfc3339()),
                is_recurring: Some(false),
                recurrence_type: None,
                recurrence_end_date: None,
                planned_date: Some(Utc::now().format("%Y-%m-%d").to_string()),
                tag_ids: Some(vec![tag.id]),
            },
        )
        .expect("Create task");

        assert_eq!(task.title, "Finish Report");
        assert_eq!(task.tags.len(), 1);
        assert_eq!(task.tags[0].name, "Work");

        // 3. Complete Task on time -> status 'done'
        let completed = task_service::complete(&conn, task.id).expect("Complete task");
        assert_eq!(completed.status, "done");

        // 4. Uncomplete Task -> status 'pending'
        let uncompleted = task_service::uncomplete(&conn, task.id).expect("Uncomplete task");
        assert_eq!(uncompleted.status, "pending");

        // 5. Test Late completion rule
        let late_task = task_service::create(
            &conn,
            CreateTaskPayload {
                title: "Overdue Task".into(),
                description: None,
                priority: Some("low".into()),
                deadline: Some((Utc::now() - Duration::days(2)).to_rfc3339()),
                is_recurring: None,
                recurrence_type: None,
                recurrence_end_date: None,
                planned_date: None,
                tag_ids: None,
            },
        )
        .expect("Create overdue task");

        let completed_late = task_service::complete(&conn, late_task.id).expect("Complete late task");
        assert_eq!(completed_late.status, "done_late");

        // 6. List Tasks
        let list = task_service::get_tasks(&conn, GetTasksParams::default()).expect("Get tasks");
        assert_eq!(list.total, 2);
    }

    #[test]
    fn test_habits_and_streak() {
        let db = init_in_memory().expect("DB init");
        let conn = db.conn.lock().unwrap();

        let habit = habit_service::create(
            &conn,
            CreateHabitPayload {
                name: "Read 20 mins".into(),
                description: None,
                frequency: Some("daily".into()),
                icon: Some("book".into()),
                color: Some("#10B981".into()),
            },
        )
        .expect("Create habit");

        let today = Utc::now().format("%Y-%m-%d").to_string();
        habit_service::upsert_checkin(&conn, habit.id, &today, true).expect("Checkin habit");

        let habits = habit_service::get_all(&conn).expect("Get habits");
        assert_eq!(habits.len(), 1);
        assert_eq!(habits[0].streak, 1);
        assert!(habits[0].checked_in_today);
    }

    #[test]
    fn test_goals_progress() {
        let db = init_in_memory().expect("DB init");
        let conn = db.conn.lock().unwrap();

        let goal = goal_service::create(
            &conn,
            CreateGoalPayload {
                title: "Save 1000$".into(),
                description: None,
                current_value: Some(200.0),
                target_value: 1000.0,
                unit: "USD".into(),
                deadline: None,
            },
        )
        .expect("Create goal");

        assert_eq!(goal.status, "active");

        let updated = goal_service::update_progress(&conn, goal.id, 1000.0).expect("Update progress");
        assert_eq!(updated.status, "completed");
    }

    #[test]
    fn test_notes_and_mood() {
        let db = init_in_memory().expect("DB init");
        let conn = db.conn.lock().unwrap();

        let today = Utc::now().format("%Y-%m-%d").to_string();

        let note = note_service::upsert(
            &conn,
            UpsertDailyNotePayload {
                note_date: today.clone(),
                content: "# Notes for today\nAll is well.".into(),
            },
        )
        .expect("Upsert note");
        assert_eq!(note.note_date, today);

        let mood = mood_service::upsert(
            &conn,
            UpsertMoodPayload {
                entry_date: today.clone(),
                score: 5,
                note: Some("Great day!".into()),
            },
        )
        .expect("Upsert mood");
        assert_eq!(mood.score, 5);
    }

    #[test]
    fn test_stopwatch_lifecycle() {
        let db = init_in_memory().expect("DB init");
        let conn = db.conn.lock().unwrap();

        let started = focus_service::start_stopwatch(&conn, Some("Focus coding".into()), None)
            .expect("Start stopwatch");
        assert_eq!(started.state, "running");

        let paused = focus_service::pause_stopwatch(&conn).expect("Pause stopwatch");
        assert_eq!(paused.state, "paused");

        let resumed = focus_service::resume_stopwatch(&conn).expect("Resume stopwatch");
        assert_eq!(resumed.state, "running");

        let stopped = focus_service::stop_stopwatch(&conn).expect("Stop stopwatch");
        assert_eq!(stopped.state, "stopped");
    }

    #[test]
    fn test_backup_and_restore() {
        let db = init_in_memory().expect("DB init");
        let mut conn = db.conn.lock().unwrap();

        let _ = tag_service::create(
            &conn,
            CreateTagPayload {
                name: "BackupTag".into(),
                color: Some("#F59E0B".into()),
            },
        )
        .expect("Create tag");

        let json = backup_service::export_backup_json(&conn).expect("Export backup");
        assert!(json.contains("BackupTag"));

        // Now import it into the connection
        backup_service::import_backup_json(&mut conn, &json).expect("Import backup");

        let tags = tag_service::get_all(&conn).expect("Get tags");
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "BackupTag");
    }
}
