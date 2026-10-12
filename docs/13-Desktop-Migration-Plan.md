# 13 - Desktop Migration Plan

## 1. Mục tiêu kiến trúc

Chuyển đổi toàn diện LifeBoard từ mô hình 3-tier phân tán (React Web + ASP.NET Core 9 Web API + MySQL + Docker) sang ứng dụng desktop **Local-First** nguyên bản chạy bằng **Tauri 2 + React 19 + Rust + SQLite**.

```text
┌────────────────────────────────────────────────────────┐
│               LifeBoard Desktop (Tauri 2)              │
│                                                        │
│  Frontend (React 19, TypeScript, Vite)                 │
│  - 18 Feature Pages & Modern Components                │
│  - Zustand Stores & TanStack Query                     │
│  - Native Command Adapters (`invoke`)                  │
└───────────────────────────┬────────────────────────────┘
                            │ Tauri IPC Commands
┌───────────────────────────▼────────────────────────────┐
│               Rust Native Backend Core                 │
│  - Tauri Commands (App handlers)                       │
│  - Services (Domain logic, Streak, Analytics)          │
│  - Repositories (Prepared Statements & Transactions)   │
│  - Error Handling & Structured AppError                │
└───────────────────────────┬────────────────────────────┘
                            │ rusqlite / r2d2 connection pool
┌───────────────────────────▼────────────────────────────┐
│            SQLite File: `lifeboard.db`                 │
│  - Vị trí: OS AppData (e.g. %APPDATA%/LifeBoard/)       │
│  - 11 bảng chuẩn hóa, Foreign Keys enabled             │
│  - Tự động chạy migration khi khởi động                │
└────────────────────────────────────────────────────────┘
```

## 2. Các nguyên tắc thiết kế

1. **Zero Configuration:** Người dùng chỉ cần mở app, database `lifeboard.db` tự động được sinh và áp dụng migration tại thư mục dữ liệu chuẩn của hệ điều hành.
2. **Không localhost:** Tuyệt đối không mở cổng HTTP nội bộ; mọi giao tiếp thực hiện qua IPC của Tauri.
3. **Bảo toàn nghiệp vụ:** Toàn bộ công thức tính toán từ backend C# (quy tắc Late Task, tính Streak habit với Grace period, heatmap intensity, stopwatch segment logic) được giữ nguyên vẹn 100%.
4. **Hiệu năng & Tiết kiệm tài nguyên:** Khởi động dưới 1 giây, RAM tiêu thụ dưới 50MB, kích thước file cài đặt nhỏ gọn.
