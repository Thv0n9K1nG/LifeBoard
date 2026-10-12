# 17 - Feature Migration Matrix

Bảng theo dõi và nghiệm thu chuyển đổi toàn diện các tính năng sang Desktop Native Tauri 2 + React + Rust + SQLite.

| Feature / Domain | UI Component | Tauri IPC Command / Service | SQLite Table(s) | Test Suite | Trạng thái Nghiệm thu |
|---|:---:|:---:|:---:|:---:|:---:|
| **Tasks & Subtasks** | ✅ Sẵn sàng | ✅ `create_task`, `update_task`, `list_tasks`, `delete_task` | `tasks`, `task_tags` | ✅ PASS | 🏆 VERIFIED |
| **Tags** | ✅ Sẵn sàng | ✅ `create_tag`, `list_tags`, `delete_tag` | `tags`, `task_tags` | ✅ PASS | 🏆 VERIFIED |
| **Habits & Streaks** | ✅ Sẵn sàng | ✅ `create_habit`, `check_in_habit`, `list_habits`, `delete_habit` | `habits`, `habit_checkins` | ✅ PASS | 🏆 VERIFIED |
| **Goals & Milestones** | ✅ Sẵn sàng | ✅ `create_goal`, `update_goal`, `list_goals`, `delete_goal` | `goals` | ✅ PASS | 🏆 VERIFIED |
| **Daily Notes** | ✅ Sẵn sàng | ✅ `upsert_daily_note`, `get_daily_note` | `daily_notes` | ✅ PASS | 🏆 VERIFIED |
| **Mood Entries** | ✅ Sẵn sàng | ✅ `upsert_mood_entry`, `get_mood_entry` | `mood_entries` | ✅ PASS | 🏆 VERIFIED |
| **Countdowns** | ✅ Sẵn sàng | ✅ `create_countdown`, `list_countdowns`, `delete_countdown` | `countdowns` | ✅ PASS | 🏆 VERIFIED |
| **Focus Sessions** | ✅ Sẵn sàng | ✅ `create_focus_session`, `list_focus_sessions`, `delete_focus_session` | `focus_sessions` | ✅ PASS | 🏆 VERIFIED |
| **Stopwatch & Splits** | ✅ Sẵn sàng | ✅ `stopwatch_start`, `stopwatch_pause`, `stopwatch_resume`, `stopwatch_stop`, `stopwatch_split` | `focus_sessions` (state in-memory + final session persist) | ✅ PASS | 🏆 VERIFIED |
| **Dashboard Aggregates**| ✅ Sẵn sàng | ✅ `get_dashboard_aggregates` | Aggregated queries | ✅ PASS | 🏆 VERIFIED |
| **Calendar / Daily Detail** | ✅ Sẵn sàng | ✅ `get_daily_detail` | Aggregated queries | ✅ PASS | 🏆 VERIFIED |
| **Universal Search** | ✅ Sẵn sàng | ✅ `search_all` | Multi-table LIKE query (`tasks`, `habits`, `daily_notes`) | ✅ PASS | 🏆 VERIFIED |
| **Analytics & Heatmap** | ✅ Sẵn sàng | ✅ `get_analytics_summary`, `get_productivity_heatmap` | Aggregated queries | ✅ PASS | 🏆 VERIFIED |
| **Statistics** | ✅ Sẵn sàng | ✅ `get_statistics` | Aggregated queries | ✅ PASS | 🏆 VERIFIED |
| **Settings & Timezone** | ✅ Sẵn sàng | ✅ `get_settings`, `update_settings` | `settings` | ✅ PASS | 🏆 VERIFIED |
| **Backup & Restore** | ✅ Sẵn sàng | ✅ `export_backup`, `import_backup` | Toàn bộ 11 bảng SQLite (Atomic Transaction) | ✅ PASS | 🏆 VERIFIED |
| **Daily Task Scheduler**| ✅ Tự động | ✅ `run_daily_scheduler` (Startup check trong Rust) | `tasks` | ✅ PASS | 🏆 VERIFIED |

---

## Tóm tắt nghiệm thu

- **Tổng số tính năng:** 17/17 hạng mục
- **Mức độ hoàn thành:** 100%
- **Trạng thái:** Toàn bộ tính năng đã được kiểm tra, không còn tính năng nào ở trạng thái `pending` hay `in-progress`.
