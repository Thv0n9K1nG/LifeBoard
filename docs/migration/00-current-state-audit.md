# 00 - Current State Technical Audit (Baseline)

**Dự án:** LifeBoard — Chuyển đổi sang Desktop App
**Ngày audit:** 2026-10-12
**Commit baseline:** `6609905` (Tag: `pre-desktop-migration`)

---

## 1. Hiện trạng hệ thống

| Thành phần | Công nghệ hiện tại | Đánh giá | Hướng chuyển đổi |
|---|---|---|---|
| **Frontend** | React 19 + TypeScript + Vite | Hoàn thiện 15+ feature pages & components, styles tokens | Giữ nguyên 95%, chuyển các API adapters sang Tauri IPC |
| **Backend** | ASP.NET Core 9 Web API | 14 Controllers, Services, DTOs, Dapper | Viết lại thành native Rust Commands & Services trong `src-tauri` |
| **Database** | MySQL 8.0 Server | 11 bảng quan hệ trong `001_init_schema.sql` + settings | Chuyển sang SQLite local file (`lifeboard.db`) trong AppData |
| **Data Fetching** | Axios + TanStack Query | Các file `*Api.ts` gọi HTTP `http://localhost:5000/api/v1` | Đổi các file `*Api.ts` sang Tauri `invoke` IPC |
| **Deploy** | Docker Compose | Cồng kềnh, không phù hợp cho người dùng desktop cá nhân | Đóng gói native bundle bằng Tauri 2 CLI |

---

## 2. Bảng ánh xạ Endpoint ↔ Frontend ↔ Rust Command ↔ SQLite

| Domain | Frontend API File | ASP.NET Endpoint | Rust Tauri Command | Bảng SQLite |
|---|---|---|---|---|
| **Tasks** | `tasks/api/taskApi.ts` | `/tasks` (CRUD, complete, uncomplete, archive) | `get_tasks`, `get_task_by_id`, `create_task`, `update_task`, `complete_task`, `uncomplete_task`, `archive_task`, `delete_task` | `tasks`, `task_tags` |
| **Tags** | `tags/api/tagApi.ts` | `/tags` (CRUD) | `get_tags`, `create_tag`, `update_tag`, `delete_tag` | `tags`, `task_tags` |
| **Habits** | `habits/api/habitApi.ts` | `/habits`, `/habits/{id}/checkins` | `get_habits`, `create_habit`, `update_habit`, `delete_habit`, `checkin_habit` | `habits`, `habit_checkins` |
| **Goals** | `goals/api/goalApi.ts` | `/goals`, `/goals/{id}/progress` | `get_goals`, `get_goal_by_id`, `create_goal`, `update_goal`, `update_goal_progress`, `delete_goal` | `goals` |
| **Notes** | `notes/api/noteApi.ts` | `/daily-notes`, `/daily-notes/date/{date}` | `get_daily_notes`, `get_daily_note_by_date`, `upsert_daily_note`, `delete_daily_note` | `daily_notes` |
| **Mood** | `mood/api/moodApi.ts` | `/mood-entries`, `/mood-entries/date/{date}` | `get_mood_entries`, `get_mood_entry_by_date`, `upsert_mood_entry`, `delete_mood_entry` | `mood_entries` |
| **Countdowns** | `countdown/api/countdownApi.ts` | `/countdowns` | `get_countdowns`, `create_countdown`, `update_countdown`, `delete_countdown` | `countdowns` |
| **Focus** | `focus/api/focusApi.ts` | `/focus-sessions` | `get_focus_sessions`, `create_focus_session`, `update_focus_session`, `delete_focus_session` | `focus_sessions` |
| **Stopwatch** | `focus/api/stopwatchApi.ts` | `/stopwatch/*` | `get_stopwatch_state`, `start_stopwatch`, `pause_stopwatch`, `resume_stopwatch`, `stop_stopwatch`, `split_stopwatch`, `update_stopwatch_label` | `focus_sessions` |
| **Analytics** | `analytics/api/analyticsApi.ts` | `/analytics/weekly`, `/monthly`, `/yearly`, `/statistics` | `get_weekly_analytics`, `get_monthly_analytics`, `get_yearly_analytics`, `get_statistics` | `tasks`, `focus_sessions`, `habit_checkins`, `mood_entries` |
| **Settings** | `settings/api/settingsApi.ts` | `/settings` | `get_settings`, `update_settings` | `settings` |
| **Backup** | `settings` | `/backup` | `backup_database`, `restore_database` | Tất cả bảng |
