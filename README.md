# LifeBoard — Personal Productivity Desktop System

> Local-first native desktop application for task management, habit tracking, focus sessions & analytics.

<img src="docs/overview.png" alt="LifeBoard overview" />

## Architecture

LifeBoard is a 100% offline-first, local desktop application built on **Tauri 2**, **React 19**, **Rust**, and **SQLite (WAL mode)**. It requires zero server setup, zero containers, and zero external databases.

| Layer | Technology | Details |
|---|---|---|
| **Desktop Shell** | [Tauri 2](https://v2.tauri.app/) | Native lightweight Windows webview window, secure IPC bridge |
| **Frontend** | React 19 + TypeScript + Vite | Tailwind CSS, Lucide icons, Recharts, Zustand, TanStack Query |
| **Desktop Backend** | Rust (`src-tauri`) | Native business logic, thread-safe SQLite connection pool, zero HTTP overhead |
| **Local Storage** | SQLite (`rusqlite` bundled) | Single file database with WAL journal mode, Foreign Keys enabled |
| **Data Directory** | `%APPDATA%\LifeBoard\lifeboard.db` | User personal data stored entirely on local disk |

---

## Key Features

- ✅ **Tasks & Subtasks**: Status progression (`todo`, `in_progress`, `done`, `done_late`, `cancelled`), priorities, tags, and deadline tracking.
- ✅ **Habits & Streak Engine**: 60-day visual streak tracker with automatic grace-period calculation and weekly frequency tracking.
- ✅ **Goals & Milestones**: Progress bar tracking, deadline monitoring, auto-completion upon reaching target metrics.
- ✅ **Daily Notes & Mood**: Rich markdown notes and daily mood logging (1–5 scale with mood tags).
- ✅ **Focus & Stopwatch**: Pomodoro focus session logging, active stopwatch state machine with lap splits and tags.
- ✅ **Universal Search**: Fast local instant search across tasks, habits, and notes.
- ✅ **Analytics & Statistics**: Productivity heatmaps, priority distribution, focus charts, and completion ratios.
- ✅ **Backup & Restore**: Native 1-click JSON backup export and transactional restore across all 11 tables.

---

## Prerequisites (For Developers)

To develop or build LifeBoard from source:
1. **Node.js**: v20+ (`node -v`)
2. **Rust & Cargo**: v1.80+ (`rustc --version`)
3. **Windows C++ Build Tools**: Visual Studio Build Tools with C++ workload.

*Note for End Users: Distributed installers (`.msi` or `.exe`) run out of the box on Windows with standard WebView2 runtime—no Rust, Node.js, or Docker required.*

---

## Quick Start (Development)

### 1. Install Dependencies
```bash
cd lifeboard-client
npm install
```

### 2. Run Tauri Desktop Dev
From the root directory:
```bash
npm run tauri:dev
```
Or directly from `lifeboard-client`:
```bash
npm run tauri dev
```
Tauri will launch the Vite development server and open the native LifeBoard desktop window with Hot Module Replacement (HMR).

### 3. Run Backend Tests
Run the complete suite of Rust native service and domain tests:
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

---

## Building Production Desktop App

To build the standalone Windows desktop binary and installer:
```bash
npm run tauri:build
```
The compiled binary and setup bundles will be generated in:
```text
src-tauri/target/release/
src-tauri/target/release/bundle/msi/
src-tauri/target/release/bundle/nsis/
```

---

## Local Data Storage & Backups

- **Database location:** `%APPDATA%\LifeBoard\lifeboard.db`
- **Settings & Timezone:** Stored locally in SQLite `settings` table.
- **Export & Import:** Accessible anytime via **Settings → Sao lưu & Phục hồi** to export full JSON backups or restore your workspace.

---

## Project Structure

```
LifeBoard/
├── docs/                      # Architectural docs & migration records
│   ├── 13-Desktop-Migration-Plan.md
│   ├── 14-Desktop-Command-Specification.md
│   ├── 15-SQLite-Migration-Design.md
│   ├── 17-Feature-Migration-Matrix.md
│   └── 20-Release-Checklist.md
├── lifeboard-client/          # React 19 + TypeScript frontend
│   └── src/
│       ├── components/        # Layout, modals, shared UI
│       ├── features/          # 15 domain pages (tasks, habits, notes, focus, etc.)
│       ├── lib/               # Tauri IPC commands adapter (commands.ts)
│       ├── stores/            # Zustand stores
│       └── styles/            # CSS tokens & themes
├── src-tauri/                 # Tauri 2 Desktop backend (Rust)
│   ├── src/
│   │   ├── commands/          # Tauri IPC command handlers
│   │   ├── models/            # Domain entities and DTOs
│   │   ├── services/          # Business logic, streak calculations & backup
│   │   ├── database.rs        # SQLite schema migration & connection management
│   │   ├── error.rs           # Error handling & IPC serialization
│   │   ├── lib.rs             # Tauri application entry & plugin registration
│   │   ├── main.rs            # Desktop executable entry
│   │   └── tests.rs           # In-memory SQLite integration tests
│   ├── Cargo.toml             # Rust dependencies (rusqlite, serde, chrono)
│   └── tauri.conf.json        # Tauri desktop configuration & window properties
└── package.json               # Root scripts
```

---

## Documentation

| File | Description |
|---|---|
| [13-Desktop-Migration-Plan](docs/13-Desktop-Migration-Plan.md) | Desktop architecture & transition roadmap |
| [14-Desktop-Command-Specification](docs/14-Desktop-Command-Specification.md) | Tauri IPC command contract & signatures |
| [15-SQLite-Migration-Design](docs/15-SQLite-Migration-Design.md) | SQLite schema, indexes & query mapping |
| [17-Feature-Migration-Matrix](docs/17-Feature-Migration-Matrix.md) | Feature migration audit matrix |
| [20-Release-Checklist](docs/20-Release-Checklist.md) | Quality assurance & release verification checklist |
