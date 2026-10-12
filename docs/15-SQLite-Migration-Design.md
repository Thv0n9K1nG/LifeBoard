# 15 - SQLite Migration Design

## 1. Nguyên tắc chuyển đổi kiểu dữ liệu MySQL ➔ SQLite

| Kiểu dữ liệu MySQL | Kiểu dữ liệu SQLite | Quy ước lưu trữ |
|---|---|---|
| `INT AUTO_INCREMENT PRIMARY KEY` | `INTEGER PRIMARY KEY AUTOINCREMENT` | Khóa chính số nguyên tự tăng |
| `VARCHAR(N)`, `TEXT`, `LONGTEXT` | `TEXT` | UTF-8 String |
| `ENUM(...)` | `TEXT` | Kiểm tra giá trị bằng CHECK constraint hoặc application layer |
| `TINYINT(1)` | `INTEGER` | `0` = false, `1` = true |
| `DATETIME`, `TIMESTAMP` | `TEXT` | Định dạng chuẩn ISO-8601 UTC: `YYYY-MM-DDTHH:MM:SS.sssZ` |
| `DATE` | `TEXT` | Định dạng chuẩn: `YYYY-MM-DD` |
| `DECIMAL(10,2)` | `REAL` | Số thực dấu phẩy động |

## 2. Toàn văn SQLite Migration Script (11 bảng)

```sql
PRAGMA foreign_keys = ON;

-- 1. tasks
CREATE TABLE IF NOT EXISTS tasks (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    title               TEXT NOT NULL,
    description         TEXT,
    priority            TEXT NOT NULL DEFAULT 'medium',
    status              TEXT NOT NULL DEFAULT 'pending',
    deadline            TEXT,
    is_recurring        INTEGER NOT NULL DEFAULT 0,
    recurrence_type     TEXT,
    recurrence_end_date TEXT,
    parent_task_id      INTEGER,
    planned_date        TEXT,
    created_at          TEXT NOT NULL,
    completed_at        TEXT,
    updated_at          TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_planned_date ON tasks(planned_date);
CREATE INDEX IF NOT EXISTS idx_tasks_deadline ON tasks(deadline);

-- 2. tags
CREATE TABLE IF NOT EXISTS tags (
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    name  TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#6B7280'
);

-- 3. task_tags
CREATE TABLE IF NOT EXISTS task_tags (
    task_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)  REFERENCES tags(id)  ON DELETE CASCADE
);

-- 4. habits
CREATE TABLE IF NOT EXISTS habits (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT,
    frequency   TEXT NOT NULL DEFAULT 'daily',
    icon        TEXT,
    color       TEXT NOT NULL DEFAULT '#6B7280',
    is_active   INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT NOT NULL
);

-- 5. habit_checkins
CREATE TABLE IF NOT EXISTS habit_checkins (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    habit_id     INTEGER NOT NULL,
    checkin_date TEXT NOT NULL,
    is_completed INTEGER NOT NULL DEFAULT 0,
    UNIQUE(habit_id, checkin_date),
    FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE
);

-- 6. focus_sessions
CREATE TABLE IF NOT EXISTS focus_sessions (
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type           TEXT NOT NULL,
    label                  TEXT,
    start_time             TEXT NOT NULL,
    end_time               TEXT,
    duration_seconds       INTEGER NOT NULL DEFAULT 0,
    session_date           TEXT NOT NULL,
    splits                 TEXT,
    stopwatch_state        TEXT,
    paused_duration_seconds INTEGER NOT NULL DEFAULT 0,
    current_segment_start  TEXT,
    created_at             TEXT NOT NULL
);

-- 7. goals
CREATE TABLE IF NOT EXISTS goals (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    title         TEXT NOT NULL,
    description   TEXT,
    current_value REAL NOT NULL DEFAULT 0.0,
    target_value  REAL NOT NULL,
    unit          TEXT NOT NULL,
    deadline      TEXT,
    status        TEXT NOT NULL DEFAULT 'active',
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- 8. daily_notes
CREATE TABLE IF NOT EXISTS daily_notes (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    note_date  TEXT NOT NULL UNIQUE,
    content    TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 9. mood_entries
CREATE TABLE IF NOT EXISTS mood_entries (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_date TEXT NOT NULL UNIQUE,
    score      INTEGER NOT NULL CHECK(score BETWEEN 1 AND 5),
    note       TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 10. countdowns
CREATE TABLE IF NOT EXISTS countdowns (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    target_date TEXT NOT NULL,
    icon        TEXT,
    color       TEXT DEFAULT '#6B7280',
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

-- 11. settings
CREATE TABLE IF NOT EXISTS settings (
    id                      INTEGER PRIMARY KEY,
    theme                   TEXT NOT NULL DEFAULT 'dark',
    pomodoro_focus_minutes  INTEGER NOT NULL DEFAULT 25,
    pomodoro_break_minutes  INTEGER NOT NULL DEFAULT 5,
    pomodoro_rounds         INTEGER NOT NULL DEFAULT 4,
    habit_grace_period_days INTEGER NOT NULL DEFAULT 0,
    language                TEXT NOT NULL DEFAULT 'vi',
    timezone                TEXT NOT NULL DEFAULT 'Asia/Ho_Chi_Minh',
    updated_at              TEXT NOT NULL
);

INSERT OR IGNORE INTO settings (id, theme, pomodoro_focus_minutes, pomodoro_break_minutes, pomodoro_rounds, habit_grace_period_days, language, timezone, updated_at)
VALUES (1, 'dark', 25, 5, 4, 0, 'vi', 'Asia/Ho_Chi_Minh', datetime('now'));
```
