# 14 - Desktop Command Specification (Tauri IPC Contracts)

Tài liệu này định nghĩa chi tiết các Tauri IPC Commands giữa React Frontend và Rust Core.

---

## 1. Task & Tag Commands

* `get_tasks(filter: TaskFilter) -> Result<TaskListDto, AppError>`
* `get_task_by_id(id: i64) -> Result<TaskDto, AppError>`
* `create_task(payload: CreateTaskPayload) -> Result<TaskDto, AppError>`
* `update_task(id: i64, payload: UpdateTaskPayload) -> Result<TaskDto, AppError>`
* `complete_task(id: i64) -> Result<TaskDto, AppError>` (Áp dụng rule `done` vs `done_late` theo deadline/planned_date)
* `uncomplete_task(id: i64) -> Result<TaskDto, AppError>`
* `archive_task(id: i64) -> Result<TaskDto, AppError>`
* `delete_task(id: i64) -> Result<(), AppError>`
* `get_tags() -> Result<Vec<TagDto>, AppError>`
* `create_tag(payload: CreateTagPayload) -> Result<TagDto, AppError>`
* `update_tag(id: i64, payload: UpdateTagPayload) -> Result<TagDto, AppError>`
* `delete_tag(id: i64) -> Result<(), AppError>`

---

## 2. Habits Commands

* `get_habits() -> Result<Vec<HabitDto>, AppError>` (Bao gồm tính streak 60 ngày & checkedInToday)
* `create_habit(payload: CreateHabitPayload) -> Result<HabitDto, AppError>`
* `update_habit(id: i64, payload: UpdateHabitPayload) -> Result<HabitDto, AppError>`
* `delete_habit(id: i64) -> Result<(), AppError>`
* `checkin_habit(id: i64, date: String, is_completed: bool) -> Result<(), AppError>`

---

## 3. Goals Commands

* `get_goals(status: Option<String>) -> Result<Vec<GoalDto>, AppError>`
* `get_goal_by_id(id: i64) -> Result<GoalDto, AppError>`
* `create_goal(payload: CreateGoalPayload) -> Result<GoalDto, AppError>`
* `update_goal(id: i64, payload: UpdateGoalPayload) -> Result<GoalDto, AppError>`
* `update_goal_progress(id: i64, current_value: f64) -> Result<GoalDto, AppError>`
* `delete_goal(id: i64) -> Result<(), AppError>`

---

## 4. Daily Notes & Mood Commands

* `get_daily_notes() -> Result<Vec<DailyNoteDto>, AppError>`
* `get_daily_note_by_date(date: String) -> Result<Option<DailyNoteDto>, AppError>`
* `upsert_daily_note(payload: UpsertDailyNotePayload) -> Result<DailyNoteDto, AppError>`
* `delete_daily_note(id: i64) -> Result<(), AppError>`
* `get_mood_entries() -> Result<Vec<MoodEntryDto>, AppError>`
* `get_mood_entry_by_date(date: String) -> Result<Option<MoodEntryDto>, AppError>`
* `upsert_mood_entry(payload: UpsertMoodPayload) -> Result<MoodEntryDto, AppError>`
* `delete_mood_entry(id: i64) -> Result<(), AppError>`

---

## 5. Countdowns Commands

* `get_countdowns() -> Result<Vec<CountdownDto>, AppError>`
* `create_countdown(payload: CreateCountdownPayload) -> Result<CountdownDto, AppError>`
* `update_countdown(id: i64, payload: UpdateCountdownPayload) -> Result<CountdownDto, AppError>`
* `delete_countdown(id: i64) -> Result<(), AppError>`

---

## 6. Focus & Stopwatch Commands

* `get_focus_sessions(from: String, to: String, session_type: Option<String>) -> Result<FocusSessionListDto, AppError>`
* `create_focus_session(payload: CreateFocusSessionPayload) -> Result<FocusSessionDto, AppError>`
* `update_focus_session(id: i64, payload: UpdateFocusSessionPayload) -> Result<FocusSessionDto, AppError>`
* `delete_focus_session(id: i64) -> Result<(), AppError>`
* `get_stopwatch_state() -> Result<Option<StopwatchStateDto>, AppError>`
* `start_stopwatch(label: Option<String>, session_date: Option<String>) -> Result<StopwatchStateDto, AppError>`
* `pause_stopwatch() -> Result<StopwatchStateDto, AppError>`
* `resume_stopwatch() -> Result<StopwatchStateDto, AppError>`
* `stop_stopwatch() -> Result<StopwatchStateDto, AppError>`
* `split_stopwatch(note: Option<String>) -> Result<StopwatchStateDto, AppError>`
* `update_stopwatch_label(label: String) -> Result<StopwatchStateDto, AppError>`

---

## 7. Analytics & Search Commands

* `get_weekly_analytics(week_start: String) -> Result<WeeklyAnalyticsDto, AppError>`
* `get_monthly_analytics(year: i32, month: u32) -> Result<MonthlyAnalyticsDto, AppError>`
* `get_yearly_analytics(year: i32) -> Result<YearlyAnalyticsDto, AppError>`
* `get_statistics(from: String, to: String) -> Result<StatisticsDto, AppError>`

---

## 8. Settings & Backup Commands

* `get_settings() -> Result<SettingsDto, AppError>`
* `update_settings(payload: SettingsDto) -> Result<SettingsDto, AppError>`
* `export_backup_json() -> Result<String, AppError>`
* `import_backup_json(json_content: String) -> Result<(), AppError>`
