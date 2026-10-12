pub mod commands;
pub mod database;
pub mod error;
pub mod models;
pub mod services;
#[cfg(test)]
mod tests;

use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let db = database::init_db(app.handle())
                .expect("Failed to initialize SQLite database");

            // Run daily recurring tasks & auto-import check on startup
            {
                let conn = db.conn.lock().unwrap();
                if let Err(e) = services::task_service::run_daily_scheduler(&conn) {
                    log::error!("Daily scheduler error on startup: {:?}", e);
                }
            }

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Tags
            get_tags,
            create_tag,
            update_tag,
            delete_tag,
            // Tasks
            get_tasks,
            get_task_by_id,
            create_task,
            update_task,
            complete_task,
            uncomplete_task,
            archive_task,
            delete_task,
            // Habits
            get_habits,
            create_habit,
            update_habit,
            delete_habit,
            checkin_habit,
            // Goals
            get_goals,
            get_goal_by_id,
            create_goal,
            update_goal,
            update_goal_progress,
            delete_goal,
            // Daily Notes
            get_daily_notes,
            get_daily_note_by_date,
            upsert_daily_note,
            delete_daily_note,
            // Mood
            get_mood_entries,
            get_mood_entry_by_date,
            upsert_mood_entry,
            delete_mood_entry,
            // Countdowns
            get_countdowns,
            create_countdown,
            update_countdown,
            delete_countdown,
            // Focus & Stopwatch
            get_focus_sessions,
            create_focus_session,
            update_focus_session,
            delete_focus_session,
            get_stopwatch_state,
            start_stopwatch,
            pause_stopwatch,
            resume_stopwatch,
            stop_stopwatch,
            split_stopwatch,
            update_stopwatch_label,
            // Analytics
            get_weekly_analytics,
            get_monthly_analytics,
            get_yearly_analytics,
            get_statistics,
            // Settings & Backup
            get_settings,
            update_settings,
            export_backup_json,
            import_backup_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
