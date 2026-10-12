# LifeBoard — Kế hoạch chuyển đổi sang Desktop App theo Sprint

> **Mục tiêu:** Chuyển repository LifeBoard hiện tại từ kiến trúc React + ASP.NET Core Web API + MySQL + Docker sang ứng dụng desktop local-first dùng Tauri 2, React + TypeScript + Vite, Rust Commands/Services và SQLite.
>
> **Nguyên tắc quan trọng:** Refactor ngay trên repository hiện tại, không tạo repository mới. Thực hiện theo từng sprint có tiêu chí nghiệm thu rõ ràng; không xóa hệ thống cũ trước khi phần thay thế chạy ổn định.
>
> **Phạm vi nền tảng ban đầu:** Windows. Kiến trúc nên tránh khóa cứng vào Windows để có thể mở rộng sang Linux/macOS sau này.
>
> **Không nằm trong phạm vi:** CI/CD, đồng bộ cloud, tài khoản người dùng, multi-device sync hoặc viết lại giao diện từ đầu nếu không cần thiết.

---

## 1. Hiện trạng và đích đến

### 1.1. Kiến trúc hiện tại

- Frontend: `lifeboard-client/`, React + TypeScript + Vite.
- UI: các trang và feature như dashboard, tasks, calendar, habits, goals, notes, mood, focus/Pomodoro, stopwatch, countdown, search, analytics/statistics và settings.
- Data fetching: API modules theo feature, hooks, Axios và TanStack Query.
- State: Zustand cho một số trạng thái giao diện/cài đặt.
- Backend: `LifeBoard.API/`, ASP.NET Core Web API, Controllers, Services, Repositories, DTOs và Dapper.
- Database: MySQL; migration SQL đang nằm trong `LifeBoard.API/Migrations/`.
- Triển khai: Docker Compose, Dockerfile và các cấu hình liên quan.
- Tác vụ nền: `DailyTaskScheduler.cs`.
- Tài liệu: `docs/01-Vision.md` đến `docs/12-Deployment-Guide.md`, cùng `docs/stage4-breakdown.md` và `docs/timeline.md`.

### 1.2. Kiến trúc mục tiêu

```text
┌──────────────────────────────────────────┐
│              LifeBoard Desktop           │
│  Tauri 2 + WebView                        │
│                                          │
│  React + TypeScript + Vite               │
│  Pages / Components / Feature UI         │
└───────────────────┬──────────────────────┘
                    │ Tauri IPC / Commands
┌───────────────────▼──────────────────────┐
│                 Rust                     │
│  Commands → Services → Repositories      │
│  Validation / Domain rules / Errors      │
└───────────────────┬──────────────────────┘
                    │ SQLx
┌───────────────────▼──────────────────────┐
│              SQLite file                 │
│  App data directory, local-only           │
└──────────────────────────────────────────┘
```

Sau khi hoàn tất, người dùng không cần tự cài Docker, MySQL Server hay khởi chạy ASP.NET Core API. Frontend không gọi REST API qua `localhost`; thay vào đó, UI gọi Tauri Commands qua IPC.

### 1.3. Stack mục tiêu

| Lớp | Công nghệ | Ghi chú |
|---|---|---|
| Desktop shell | Tauri 2 | Cửa sổ, tích hợp hệ điều hành, bundle/installer |
| UI | React + TypeScript + Vite | Giữ lại phần lớn UI hiện tại |
| IPC | Tauri Commands | Thay các lời gọi HTTP tới backend |
| Native backend | Rust | Nghiệp vụ, validation, truy cập dữ liệu |
| Database | SQLite | File local, không cần database server |
| DB access | SQLx + SQLite | Dùng prepared/parameterized queries |
| Migration | SQLx migrations | Schema SQLite được quản lý bằng migration |
| UI state | Zustand | Giữ nơi còn phù hợp |
| Query/cache | TanStack Query | Có thể giữ, nhưng queryFn gọi command adapter |
| Frontend tests | Vitest + React Testing Library | Nếu đã có test runner thì ưu tiên tái sử dụng |
| Rust tests | `cargo test` | Unit/integration tests |
| Installer | Tauri bundler, NSIS hoặc MSI | Kiểm thử trên máy Windows sạch |

**Lưu ý lựa chọn SQLx:** SQLx là lựa chọn đề xuất, không phải bắt buộc. Nếu trong quá trình triển khai cần giảm độ phức tạp, có thể đánh giá `rusqlite`; quyết định phải được ghi trong SDD trước khi triển khai database layer, không đổi driver tùy tiện giữa các sprint.

---

## 2. Quy tắc làm việc bắt buộc

1. **Không làm việc trực tiếp trên nhánh chính.** Mỗi sprint dùng một nhánh riêng, ví dụ `refactor/desktop-sprint-01-foundation`.
2. Trước khi sửa, tạo tag hoặc nhánh đánh dấu trạng thái hiện tại, ví dụ `pre-desktop-migration`.
3. Commit trạng thái hiện tại trước khi bắt đầu. Không dựa vào `bin/`, `obj/`, `node_modules/` hay build output làm bản backup.
4. Giữ nguyên backend MySQL cũ trong giai đoạn chuyển tiếp. Không xóa `LifeBoard.API/`, `docker-compose.yml` hoặc migrations MySQL cho đến khi có kiểm thử nghiệm thu và kế hoạch rollback.
5. Không chuyển toàn bộ UI cùng lúc. Chuyển theo **vertical slice**: UI → command → service → repository → SQLite → test.
6. Không viết lại business rule chỉ dựa vào tên method. Đọc implementation C# hiện tại và tài liệu `docs/` để giữ đúng hành vi.
7. Mỗi sprint phải kết thúc ở trạng thái có thể build/run được. Không để hàng loạt phần dang dở kéo dài qua nhiều sprint.
8. Không để React truy cập SQLite trực tiếp, không nhúng SQL vào component và không tạo HTTP server localhost chỉ để giữ kiến trúc cũ.
9. Không lưu database trong thư mục cài đặt hoặc thư mục build. Dùng app data directory của hệ điều hành.
10. Không xóa hoặc ghi đè dữ liệu người dùng trong quá trình migration. Backup và restore phải được kiểm thử.
11. Dùng `cargo fmt`, `cargo clippy` và test tương ứng khi Rust code đã xuất hiện; dùng lint/typecheck/test frontend nếu repository đã có hoặc khi được thiết lập trong Sprint 0.
12. Mọi command được expose qua Tauri phải có input/output rõ ràng, validate input và trả lỗi có cấu trúc. Không expose API hệ thống rộng hơn mức cần thiết.

### Quy trình bắt đầu mỗi sprint

```bash
git status
git switch main
git pull
git switch -c refactor/desktop-sprint-XX-short-name
```

Thay `main` bằng tên nhánh mặc định thực tế nếu repo đang dùng tên khác. Nếu dự án làm một mình và không có remote mới nhất, vẫn cần kiểm tra `git status` và commit thay đổi hiện có trước khi tạo nhánh.

### Definition of Done dùng cho mọi sprint

- [ ] Phần việc nằm trong phạm vi sprint đã hoàn thành.
- [ ] App/build liên quan chạy được.
- [ ] Không có lỗi TypeScript/Rust mới chưa được giải thích.
- [ ] Test liên quan đã chạy và kết quả được ghi lại.
- [ ] Đã kiểm tra ít nhất một luồng thành công và các lỗi quan trọng.
- [ ] Tài liệu bị ảnh hưởng đã được cập nhật.
- [ ] Không commit secret, database người dùng, build artifacts hoặc file môi trường cá nhân.
- [ ] Có commit rõ nghĩa; không gộp thay đổi ngoài phạm vi nếu chưa đánh giá.

---

## 3. Tổng quan các Sprint

| Sprint | Chủ đề | Kết quả chính |
|---|---|---|
| 0 | Audit, baseline và bảo vệ repository | Biết chính xác trạng thái hiện tại, có điểm quay lại |
| 1 | Tái cấu trúc tài liệu và kiến trúc đích | SDD, command contract, database plan thống nhất |
| 2 | Khởi tạo Tauri trong repository hiện tại | Desktop window chạy được, frontend cũ vẫn được bảo toàn |
| 3 | SQLite foundation | DB file, migrations, repository skeleton và test |
| 4 | Vertical slice Tasks + Tags | Luồng dữ liệu đầu tiên hoàn chỉnh qua IPC |
| 5 | Di chuyển các feature CRUD cốt lõi | Habits, Goals, Notes, Mood, Countdown |
| 6 | Focus, Pomodoro, Stopwatch và tác vụ theo ngày | Xử lý thời gian, phiên tập trung và scheduler |
| 7 | Dashboard, Calendar, Daily Detail, Search, Analytics | Các tính năng tổng hợp dùng SQLite |
| 8 | Settings, timezone, backup/restore và import dữ liệu | Bảo toàn dữ liệu và cài đặt local |
| 9 | Dọn kiến trúc cũ và hoàn thiện desktop UX | Không còn runtime dependency vào Docker/MySQL/API cũ |
| 10 | Kiểm thử hồi quy, đóng gói và phát hành | Installer và checklist release hoàn chỉnh |

Số lượng sprint là kế hoạch gợi ý, không phải cam kết thời gian cố định. Nếu một sprint lớn hơn dự kiến, hãy chia nhỏ; không bỏ tiêu chí nghiệm thu để chạy theo lịch.

---

# Sprint 0 — Audit, baseline và bảo vệ repository

## Mục tiêu

Hiểu rõ hiện trạng trước khi thay đổi kiến trúc; xác nhận frontend/backend hiện chạy ra sao và tạo điểm rollback an toàn.

## Công việc

### 0.1. Bảo vệ trạng thái hiện tại

- [ ] Kiểm tra `git status`, branch hiện tại và các thay đổi chưa commit.
- [ ] Commit những thay đổi có chủ đích; xử lý hoặc lưu riêng các thay đổi chưa hoàn tất.
- [ ] Tạo tag `pre-desktop-migration` trỏ tới commit baseline.
- [ ] Xác nhận `.gitignore` loại trừ `node_modules/`, `bin/`, `obj/`, output build, file DB local, secret và file môi trường cá nhân.
- [ ] Không đưa database có dữ liệu cá nhân vào Git.

Ví dụ:

```bash
git status
git log -5 --oneline
git tag pre-desktop-migration
```

Chỉ tạo tag sau khi đã xác nhận commit hiện tại là điểm baseline mong muốn.

### 0.2. Ghi lại baseline kỹ thuật

- [ ] Đọc `README.md`.
- [ ] Đọc `docs/01-Vision.md`, `docs/02-Product-Backlog.md`, `docs/05-Domain-Model.md`, `docs/06-SRS.md`, `docs/07-ERD-Database-Design.md`, `docs/08-SDD.md`, `docs/09-API-Specification.md`, `docs/11-Test-Plan.md`, `docs/12-Deployment-Guide.md`.
- [ ] Đọc `lifeboard-client/package.json`, `src/lib/api.ts`, `src/lib/queryClient.ts`, `src/router/index.tsx`.
- [ ] Đọc `LifeBoard.API/Program.cs`, `Infrastructure/DbConnectionFactory.cs`, các Controllers, Services, Repositories và migrations.
- [ ] Lập bảng endpoint hiện có ↔ API module frontend ↔ service/repository ↔ bảng MySQL.
- [ ] Ghi lại lệnh build/test hiện có và những lệnh nào chưa chạy được.
- [ ] Ghi lại các lỗi hiện hữu trước refactor để tránh nhầm chúng với regression.

### 0.3. Phân loại code

Gắn từng phần vào một trong bốn nhóm:

- **Giữ:** UI components, theme, layout, domain concepts và business rules còn phù hợp.
- **Sửa:** API adapters, data hooks, router/bootstrap, settings persistence.
- **Viết lại:** database access, backend entry points, migrations, scheduler nếu phụ thuộc ASP.NET.
- **Loại bỏ sau nghiệm thu:** Docker deployment, REST API runtime và cấu hình MySQL cũ.

## Tiêu chí nghiệm thu

- [ ] Có tag `pre-desktop-migration`.
- [ ] Có sơ đồ kiến trúc hiện tại và bảng mapping feature/API/database.
- [ ] Biết cách chạy frontend và backend hiện tại, hoặc ghi rõ bước nào đang lỗi.
- [ ] Chưa xóa bất kỳ backend hay dữ liệu nào.

## Đầu ra

- `docs/migration/00-current-state-audit.md`
- Tag Git `pre-desktop-migration`

---

# Sprint 1 — Chốt kiến trúc đích và hợp đồng dữ liệu

## Mục tiêu

Tránh việc vừa code vừa thay đổi quyết định kiến trúc. Sprint này chỉ chốt những hợp đồng cần thiết cho các sprint sau.

## Công việc

### 1.1. Cập nhật SDD

Tạo `docs/13-Desktop-Migration-Plan.md` (tài liệu này) và cập nhật `docs/08-SDD.md` hoặc tạo `docs/13-Desktop-SDD.md` nếu muốn giữ lịch sử tài liệu cũ.

Tài liệu phải mô tả:

- Tauri 2 + React/TypeScript/Vite + Rust + SQLite.
- Ranh giới giữa UI, commands, services, repositories và database.
- Cách khởi tạo app state và quản lý connection/pool SQLite.
- Vị trí lưu database, logs, exports và backup.
- Xử lý lỗi và logging.
- Quy tắc timezone/date/time.
- Cách test command và repository.
- Quy tắc migration schema.

### 1.2. Thiết kế command contract

Tạo `docs/14-Desktop-Command-Specification.md`.

Mỗi command cần có:

- Tên và mục đích.
- Input schema.
- Output schema.
- Quy tắc validation.
- Các lỗi có thể trả về.
- Hành vi khi record không tồn tại.
- Các thay đổi database và transaction liên quan.

Ví dụ minh họa:

```text
create_task(input: CreateTaskInput) -> Result<TaskDto, AppError>
complete_task(task_id: string) -> Result<TaskDto, AppError>
list_tasks(filter: TaskFilter) -> Result<TaskListDto, AppError>
```

Đây là contract đề xuất; cần đối chiếu với DTO và nghiệp vụ hiện tại trước khi chốt.

### 1.3. Thiết kế SQLite

- [ ] Rà soát `LifeBoard.API/Migrations/001_init_schema.sql` và `002_add_timezone_to_settings.sql`.
- [ ] Đối chiếu ERD trong `docs/07-ERD-Database-Design.md`.
- [ ] Xác định kiểu dữ liệu SQLite cho ID, boolean, ngày giờ, enum/status và JSON nếu có.
- [ ] Xác định khóa ngoại, index, unique constraints, `ON DELETE` và transaction.
- [ ] Kiểm tra những truy vấn MySQL có cú pháp/phép toán đặc thù.
- [ ] Chốt quy ước lưu timestamp: UTC hay local time; lưu timezone IANA/Windows ra sao; xử lý ngày không có giờ thế nào.
- [ ] Không dùng chuỗi ngày giờ không nhất quán giữa các bảng.

### 1.4. Quyết định cách giữ frontend state/cache

- Zustand tiếp tục dùng cho trạng thái UI và settings nếu phù hợp.
- TanStack Query có thể tiếp tục dùng cho cache/loading/error/invalidation.
- Tạo một adapter duy nhất gọi Tauri Commands, không gọi `invoke` rải rác trong component.
- Không duy trì Axios REST client cho luồng dữ liệu desktop sau khi feature đã chuyển xong.
- Xác định quy tắc invalidation sau mutation để dữ liệu dashboard/calendar không bị stale.

## Tiêu chí nghiệm thu

- [ ] Kiến trúc được mô tả bằng sơ đồ và ranh giới module rõ ràng.
- [ ] Command contract ban đầu bao phủ các domain hiện có.
- [ ] Có mapping MySQL → SQLite và danh sách rủi ro truy vấn.
- [ ] Có chính sách timezone và app data path.
- [ ] Chưa triển khai migration dữ liệu thật hoặc xóa code cũ.

## Đầu ra

- `docs/13-Desktop-Migration-Plan.md`
- `docs/14-Desktop-Command-Specification.md`
- `docs/15-SQLite-Migration-Design.md`

---

# Sprint 2 — Khởi tạo Tauri trong repository hiện tại

## Mục tiêu

Thêm desktop shell mà không phá cấu trúc frontend hiện có. Kết thúc sprint, app mở được dưới dạng cửa sổ desktop.

## Công việc

### 2.1. Kiểm tra môi trường

Trên Windows, kiểm tra/cài các prerequisites chính thức của Tauri 2: Rust toolchain, Node.js phù hợp, Microsoft C++ Build Tools và WebView2 theo yêu cầu nền tảng. Ghi lệnh và phiên bản đã kiểm tra vào tài liệu setup.

Không yêu cầu người dùng cuối cài Rust hoặc Node.js để chạy installer đã đóng gói.

### 2.2. Tích hợp Tauri

- [ ] Khởi tạo Tauri 2 trong repository hiện tại, dùng `lifeboard-client/` làm frontend directory nếu phù hợp với cấu hình Tauri.
- [ ] Đặt `src-tauri/` ở vị trí rõ ràng trong repository.
- [ ] Kiểm tra dev command/build command của Vite.
- [ ] Cấu hình cửa sổ, app identifier, product name, icon và bundle target.
- [ ] Tạo một command thử nghiệm `ping` hoặc `get_app_info`.
- [ ] Tạo wrapper frontend tối thiểu để gọi command thử nghiệm.
- [ ] Giữ nguyên React Router và UI hiện tại; chưa chuyển API feature ở sprint này.
- [ ] Ghi rõ chế độ chạy dev và chế độ build desktop.

### 2.3. Cấu hình bảo mật Tauri

- [ ] Chỉ cấp capability/permission cần thiết.
- [ ] Không cấp quyền filesystem hoặc shell quá rộng.
- [ ] Không bật quyền tùy tiện chỉ để một plugin hoạt động.
- [ ] Không để frontend web không tin cậy có thể gọi các command nhạy cảm.
- [ ] Không đưa secret vào biến môi trường frontend hoặc bundle.

## Tiêu chí nghiệm thu

- [ ] Mở được cửa sổ LifeBoard desktop trên Windows.
- [ ] Frontend hiện tại render được, routing không bị hỏng.
- [ ] Command thử nghiệm được gọi qua IPC.
- [ ] `tauri dev` và desktop build chạy được theo tài liệu.
- [ ] Web/API/MySQL cũ vẫn còn để đối chiếu trong giai đoạn chuyển tiếp.

## Đầu ra

- `src-tauri/` và cấu hình Tauri.
- `docs/16-Development-Setup.md`

---

# Sprint 3 — SQLite foundation

## Mục tiêu

Xây dựng tầng lưu trữ local bền vững trước khi chuyển business features.

## Công việc

### 3.1. Thiết lập database lifecycle

- [ ] Dùng API path của Tauri để tìm app data directory.
- [ ] Tạo thư mục ứng dụng nếu chưa có.
- [ ] Đặt SQLite file trong app data directory, không đặt trong `src-tauri/`, thư mục cài đặt hoặc thư mục làm việc hiện tại.
- [ ] Khởi tạo SQLite khi app startup.
- [ ] Bật foreign keys trên connection/connection pool.
- [ ] Xác định journal mode, busy timeout và pool size phù hợp cho app single-user.
- [ ] Không để database connection bị mở lại tùy tiện trong mỗi command.

### 3.2. Migration framework

- [ ] Cấu hình SQLx với SQLite.
- [ ] Tạo migration SQLite đầu tiên trong `src-tauri/migrations/`.
- [ ] Chuyển schema theo từng migration nhỏ, có thứ tự.
- [ ] Thêm index và foreign keys cần thiết.
- [ ] Không sao chép nguyên cú pháp MySQL.
- [ ] Kiểm tra migration chạy được từ database rỗng.
- [ ] Kiểm tra mở lại app không chạy lại migration đã áp dụng.
- [ ] Xác định chính sách khi migration thất bại; không xóa database để “sửa nhanh”.

### 3.3. Error model và repository foundation

- [ ] Tạo `AppError`/error enum có thể serialize an toàn sang frontend.
- [ ] Không trả stack trace hoặc chi tiết đường dẫn nhạy cảm cho UI.
- [ ] Tạo cấu trúc `commands/`, `services/`, `repositories/`, `models/`.
- [ ] Viết test cho khởi tạo database, migration và ràng buộc khóa ngoại.
- [ ] Ghi log lỗi ở phía native, tránh log nội dung nhạy cảm không cần thiết.

## Tiêu chí nghiệm thu

- [ ] Database được tạo tự động ở app data directory.
- [ ] Migrations chạy từ database rỗng và không phá dữ liệu khi mở app lại.
- [ ] Foreign keys được thực thi.
- [ ] Có test khởi tạo/migration và test lỗi.
- [ ] Chưa phụ thuộc MySQL hoặc API cũ trong tầng SQLite mới.

## Đầu ra

- `src-tauri/src/database.rs`
- `src-tauri/migrations/`
- `src-tauri/src/models/`
- `src-tauri/src/repositories/`
- `docs/15-SQLite-Migration-Design.md` được cập nhật theo implementation thực tế.

---

# Sprint 4 — Vertical slice đầu tiên: Tasks + Tags

## Mục tiêu

Chứng minh mô hình mới end-to-end bằng một nhóm domain đủ đại diện. Không chuyển toàn bộ tính năng trước khi mô hình này ổn định.

## Công việc

### 4.1. Chuyển domain model

Đối chiếu các file hiện có:

- Backend: `TasksController`, `TaskService`/service tương ứng, `TaskRepository`, DTO/entity và `TagsController`/`TagRepository`.
- Frontend: `features/tasks/`, `features/tags/`, hooks và API modules tương ứng.
- Tài liệu: use cases, SRS, domain model, ERD và API specification.

Tạo Rust models, validation và repository phù hợp; không dịch máy móc từng lớp C# nếu cấu trúc đó không còn mang lại giá trị.

### 4.2. Chuyển command

Tối thiểu bao phủ các thao tác hiện được UI hỗ trợ:

- List/query tasks theo filter.
- Create, update, complete/uncomplete, delete task.
- Liên kết/gỡ tags và thao tác quản lý tag hiện có.
- Hỗ trợ các thuộc tính đang tồn tại như priority, status, due date, recurring rule hoặc tag relation — chỉ theo schema/nghiệp vụ thực tế.
- Validate ID, trường bắt buộc, enum/status và giới hạn input.
- Dùng transaction cho mutation cập nhật nhiều bảng.

### 4.3. Chuyển frontend adapter

- [ ] Tạo `lib/commands.ts` hoặc adapter theo domain.
- [ ] Chuyển `taskApi.ts`, `tagApi.ts` và hooks sang gọi adapter.
- [ ] Giữ nguyên UI và luồng người dùng nếu có thể.
- [ ] Cập nhật loading, error, empty state và query invalidation.
- [ ] Loại bỏ lời gọi HTTP cho Tasks/Tags sau khi feature đạt nghiệm thu.
- [ ] Không xóa API backend cũ của Tasks/Tags cho tới khi so sánh hành vi xong.

### 4.4. Test

- [ ] Rust unit tests cho validation/business rules.
- [ ] Repository tests với SQLite tạm thời.
- [ ] Test unique/FK/transaction.
- [ ] Test create → list → update → complete → delete.
- [ ] Test lỗi input không hợp lệ và record không tồn tại.
- [ ] UI smoke test: tạo và sửa task qua desktop app.
- [ ] Test app restart để xác nhận dữ liệu còn.

## Tiêu chí nghiệm thu

- [ ] Tasks và Tags hoạt động hoàn toàn qua Tauri IPC + SQLite.
- [ ] Không có HTTP request tới ASP.NET cho hai feature đã chuyển.
- [ ] UI hiện tại không bị suy giảm chức năng quan trọng.
- [ ] Test pass và dữ liệu tồn tại sau khi khởi động lại.
- [ ] Có ghi nhận các khác biệt so với API cũ.

## Đầu ra

- Vertical slice hoàn chỉnh.
- `docs/17-Feature-Migration-Matrix.md` với trạng thái Tasks/Tags = migrated.

---

# Sprint 5 — Chuyển các feature CRUD còn lại

## Mục tiêu

Chuyển các domain có thao tác CRUD và quy tắc nghiệp vụ tương đối độc lập.

## Thứ tự đề xuất

1. Habits
2. Goals
3. Notes / Daily Notes
4. Mood Entries
5. Countdowns
6. Settings cơ bản nếu settings chưa gắn với timezone/backup phức tạp

Có thể đổi thứ tự nếu dependency trong code hiện tại yêu cầu, nhưng phải cập nhật migration matrix.

## Công việc lặp cho từng feature

- [ ] Đọc controller, service, repository, DTO/entity và các query hiện tại.
- [ ] Ghi rõ business rules và edge cases.
- [ ] Tạo hoặc cập nhật SQLite migration nếu cần.
- [ ] Viết Rust model, repository, service, command.
- [ ] Viết test cho nghiệp vụ và database.
- [ ] Chuyển frontend API adapter/hook sang command adapter.
- [ ] Kiểm tra cache invalidation và cập nhật các màn hình phụ thuộc.
- [ ] Test thao tác CRUD, validation, record không tồn tại, restart app.
- [ ] Đánh dấu feature migrated sau khi đạt DoD.
- [ ] Giữ implementation cũ tới khi hoàn thành đối chiếu.

## Các edge case cần chú ý

### Habits

- Quy tắc lịch thực hiện và check-in.
- Chống tạo check-in trùng nếu nghiệp vụ yêu cầu.
- Streak tính theo ngày và timezone.
- Xóa habit ảnh hưởng tới lịch sử/check-in thế nào.

### Goals

- Kiểu mục tiêu và tiến độ.
- Giới hạn tiến độ, cập nhật tiến độ và trạng thái hoàn thành.
- Quan hệ với tasks/habits nếu có trong domain model.
- Quy tắc xóa/ẩn goal và dữ liệu lịch sử.

### Notes / Daily Notes

- Ngày của note và timezone.
- Cập nhật nội dung rỗng, giới hạn nội dung và autosave nếu UI có.
- Không làm mất nội dung do race condition khi người dùng chuyển ngày nhanh.

### Mood Entries

- Một ngày được có một hay nhiều entry theo domain rule.
- Giá trị mood hợp lệ.
- Thống kê lịch sử phải nhất quán với analytics.

### Countdowns

- Ngày mục tiêu, mục tiêu đã qua và timezone.
- Không tự đổi ngày do parse chuỗi ngày theo UTC/local không nhất quán.

## Tiêu chí nghiệm thu

- [ ] Các feature trong phạm vi sprint không còn gọi API HTTP cũ.
- [ ] Mỗi feature có test tương ứng và đã kiểm tra UI.
- [ ] Những màn hình dashboard/calendar/statistics phụ thuộc domain vẫn hiển thị đúng sau khi cập nhật adapter.
- [ ] Không có lỗi mất dữ liệu khi tạo/sửa/xóa.
- [ ] Migration matrix cập nhật đầy đủ.

## Đầu ra

- Các domain CRUD chạy qua SQLite.
- `docs/17-Feature-Migration-Matrix.md` được cập nhật.

---

# Sprint 6 — Focus, Pomodoro, Stopwatch và tác vụ theo ngày

## Mục tiêu

Chuyển những tính năng nhạy cảm với thời gian và trạng thái phiên chạy; kiểm tra riêng các hành vi khi app đóng, thu nhỏ hoặc máy sleep.

## Công việc

### 6.1. Focus Sessions / Pomodoro

- [ ] Đối chiếu `FocusSessionsController`, `FocusSessionService`, repository, DTO và UI `features/focus/`.
- [ ] Chuyển tạo/cập nhật/kết thúc session sang command.
- [ ] Quy định rõ nguồn thời gian: thời gian bắt đầu/kết thúc lưu thế nào và cách tính duration.
- [ ] Xử lý app bị đóng giữa phiên; xác định session tiếp tục, tạm dừng hay được phục hồi.
- [ ] Xử lý thời gian máy sleep hoặc thay đổi đồng hồ hệ thống.
- [ ] Kiểm thử không ghi trùng session khi người dùng bấm nhanh hoặc khởi động lại app.

### 6.2. Stopwatch

- [ ] Giữ logic hiển thị tick ở frontend nếu phù hợp; không ghi database mỗi tick.
- [ ] Persist các mốc trạng thái cần thiết thay vì liên tục lưu từng mili giây.
- [ ] Chuyển các thao tác start/pause/resume/reset/save sang command khi có lưu dữ liệu.
- [ ] Test thời gian chạy qua app restart theo hành vi đã chốt.

### 6.3. DailyTaskScheduler

Hiện có `LifeBoard.API/BackgroundServices/DailyTaskScheduler.cs`. Không được mặc định rằng chỉ cần chuyển file này sang Rust là xong.

- [ ] Đọc implementation và xác định chính xác nó làm gì, chạy lúc nào, có tạo record hay gửi thông báo không.
- [ ] Xác định yêu cầu khi cửa sổ desktop đóng.
- [ ] Nếu chỉ cần cập nhật dữ liệu khi app mở, dùng startup/foreground scheduling đơn giản.
- [ ] Nếu phải nhắc nhở khi app không mở, đánh giá system tray và Windows Task Scheduler/notification mechanism; ghi rõ giới hạn theo nền tảng.
- [ ] Đảm bảo tác vụ idempotent, không tạo dữ liệu trùng sau nhiều lần app startup.
- [ ] Không giả định Tauri command tự chạy nền sau khi ứng dụng đã thoát.

## Tiêu chí nghiệm thu

- [ ] Focus/Pomodoro/Stopwatch không phụ thuộc REST API cũ.
- [ ] Quy tắc thời gian có test cho start, pause, resume, stop và restart.
- [ ] Daily scheduler có hành vi đã xác định và được kiểm thử.
- [ ] Không ghi dữ liệu trùng hoặc duration sai do timezone.
- [ ] Nếu tính năng thông báo nền chưa được hỗ trợ, tài liệu nêu rõ thay vì giả vờ đã hoàn thành.

## Đầu ra

- Focus/time-related commands và tests.
- `docs/18-Time-and-Scheduling.md`

---

# Sprint 7 — Dashboard, Calendar, Daily Detail, Search và Analytics

## Mục tiêu

Chuyển các query tổng hợp sau khi các domain nguồn đã hoạt động ổn định trên SQLite.

## Công việc

### 7.1. Dashboard

- [ ] Xác định các nguồn dữ liệu và phép tính hiện dùng trong `DashboardPage.tsx` và `AnalyticsService.cs`.
- [ ] Chuyển truy vấn tổng hợp thành query/repository/service phù hợp.
- [ ] Tránh gọi nhiều command tuần tự nếu có thể trả một DTO tổng hợp hợp lý.
- [ ] Đảm bảo số liệu task, habit, focus session và goal nhất quán với màn hình chi tiết.
- [ ] Kiểm tra trạng thái loading/error/empty.

### 7.2. Calendar và Daily Detail

- [ ] Chuyển truy vấn theo khoảng ngày.
- [ ] Xác định ngày bắt đầu/kết thúc theo timezone người dùng.
- [ ] Xử lý tuần/tháng cắt qua năm và tháng có số ngày khác nhau.
- [ ] Kiểm tra ngày không có dữ liệu và ngày có nhiều loại record.
- [ ] Kiểm tra khi chuyển nhanh giữa các ngày, không để response cũ ghi đè ngày mới.

### 7.3. Search

- [ ] Xác định các entity hiện được tìm kiếm.
- [ ] Dùng parameterized queries.
- [ ] Chọn cách tìm kiếm phù hợp quy mô dữ liệu: `LIKE`/`instr` trước; chỉ thêm FTS5 nếu có nhu cầu thực tế và đã test migration.
- [ ] Xử lý ký tự Unicode tiếng Việt, khoảng trắng, query rỗng và ký tự đặc biệt.
- [ ] Trả về loại entity, ID và snippet đủ để mở đúng màn hình/record.

### 7.4. Analytics và Statistics

- [ ] Đối chiếu từng phép tính với `AnalyticsService.cs` và API spec.
- [ ] Viết test với bộ dữ liệu nhỏ có kết quả kỳ vọng xác định.
- [ ] Kiểm tra ranh giới tuần/tháng/năm, năm nhuận, timezone và khoảng ngày rỗng.
- [ ] Kiểm tra hiệu năng với dữ liệu giả lập lớn hơn bộ dữ liệu phát triển.
- [ ] Không chuyển nguyên query MySQL mà chưa xác nhận tương đương SQLite.

## Tiêu chí nghiệm thu

- [ ] Dashboard, calendar, daily detail, search, analytics/statistics không còn gọi API HTTP cũ.
- [ ] Test kết quả tính toán bằng dữ liệu mẫu.
- [ ] Dữ liệu tổng hợp khớp với các màn hình domain.
- [ ] Không có lỗi lệch ngày do timezone.
- [ ] Query có parameter binding; không ghép input người dùng trực tiếp vào SQL.

## Đầu ra

- Các query tổng hợp chạy trên SQLite.
- `docs/17-Feature-Migration-Matrix.md` cập nhật trạng thái.

---

# Sprint 8 — Settings, timezone, backup/restore và import dữ liệu

## Mục tiêu

Bảo đảm người dùng không mất dữ liệu, có thể backup/restore và xử lý settings nhất quán trước khi loại bỏ backend cũ.

## Công việc

### 8.1. Settings và timezone

- [ ] Chuyển settings persistence sang SQLite hoặc app config phù hợp.
- [ ] Xác định settings nào là dữ liệu người dùng và settings nào là UI preference.
- [ ] Không lưu secret trong settings plaintext nếu sau này có credential.
- [ ] Test đổi timezone và hành vi hiển thị dữ liệu lịch sử.
- [ ] Test migration từ settings schema cũ có trường timezone.

### 8.2. Backup

- [ ] Chọn định dạng backup: SQLite snapshot an toàn hoặc file export có version/schema metadata.
- [ ] Không chỉ copy file database đang được ghi mà không xử lý tính nhất quán.
- [ ] Nếu dùng SQLite backup API hoặc checkpoint/copy strategy, ghi rõ cách thực hiện và test.
- [ ] Backup chứa version format, timestamp và metadata tối thiểu.
- [ ] Không mặc định chứa logs/cache không cần thiết.
- [ ] Hiển thị nơi lưu và kết quả thành công/thất bại cho người dùng.

### 8.3. Restore

- [ ] Validate file trước khi restore.
- [ ] Tạo backup an toàn của dữ liệu hiện tại trước khi thay thế.
- [ ] Kiểm tra schema/version tương thích.
- [ ] Có transaction hoặc chiến lược thay file an toàn để tránh database nửa chừng.
- [ ] Nếu restore thất bại, dữ liệu trước đó vẫn có thể khôi phục.
- [ ] Kiểm thử restore từ backup hợp lệ, file sai, file hỏng và version không hỗ trợ.

### 8.4. Import MySQL cũ sang SQLite (nếu cần dữ liệu cũ)

- [ ] Tạo công cụ import một lần, không giữ MySQL runtime trong desktop app.
- [ ] Đọc dữ liệu từ export/dump hoặc nguồn MySQL cũ trong môi trường migration.
- [ ] Chuyển schema và giá trị enum/date/boolean sang định dạng SQLite đã quy định.
- [ ] Import theo dependency order; giữ ID/quan hệ nếu cần.
- [ ] Chạy trong transaction hoặc batch có checkpoint/rollback.
- [ ] Xuất báo cáo số lượng record theo bảng, record bị bỏ qua và lỗi.
- [ ] So sánh row counts và kiểm tra các quan hệ quan trọng.
- [ ] Không tự xóa database MySQL cũ sau import.

Nếu không cần giữ dữ liệu cũ, có thể đánh dấu import MySQL là ngoài phạm vi release đầu tiên; vẫn phải có backup/restore SQLite.

## Tiêu chí nghiệm thu

- [ ] Settings và timezone hoạt động nhất quán.
- [ ] Backup có thể tạo và restore được.
- [ ] Restore lỗi không làm mất database hiện tại.
- [ ] Nếu import MySQL nằm trong phạm vi, có báo cáo đối soát và test dữ liệu.
- [ ] Tài liệu hướng dẫn backup/restore rõ ràng.

## Đầu ra

- Backup/restore implementation.
- `docs/19-Backup-Restore-and-Data-Migration.md`

---

# Sprint 9 — Dọn kiến trúc cũ và hoàn thiện desktop UX

## Mục tiêu

Chỉ dọn phần cũ sau khi feature migration đã hoàn tất và được nghiệm thu. Đây là sprint loại bỏ, không phải sprint để viết lại nghiệp vụ.

## Điều kiện vào sprint

- [ ] Tất cả feature cần phát hành có trạng thái `migrated` trong migration matrix.
- [ ] Không còn UI flow nào cần API cũ.
- [ ] Backup/restore đã test.
- [ ] Installer thử nghiệm đã chạy được.
- [ ] Có tag/commit trước khi bắt đầu cleanup.

## Công việc

### 9.1. Xác minh không còn phụ thuộc API

- [ ] Tìm kiếm toàn repo các URL `localhost`, `127.0.0.1`, `/api/v1`, `axios`, API base URL và biến môi trường backend.
- [ ] Xác minh các match còn lại có chủ đích hay là code cũ.
- [ ] Xóa API adapter cũ chỉ sau khi không còn import.
- [ ] Xóa dependency frontend không còn sử dụng sau khi xác nhận qua build/test.
- [ ] Xóa hoặc archive `LifeBoard.API/`, `docker-compose.yml`, Dockerfile và config Nginx chỉ sau khi đã quyết định lưu lịch sử thế nào.
- [ ] Giữ các migration MySQL hoặc snapshot tài liệu cần thiết trong Git history; không cần đóng gói vào app.
- [ ] Cập nhật README, docs deployment và sơ đồ kiến trúc.

### 9.2. Desktop UX

- [ ] App name, icon, cửa sổ mặc định và minimum size.
- [ ] Hành vi đóng cửa sổ, thoát ứng dụng và system tray (nếu có).
- [ ] Xử lý lỗi database rõ ràng.
- [ ] Xác nhận app data directory và đường dẫn export/backup.
- [ ] Keyboard shortcuts chỉ thêm khi có nhu cầu rõ.
- [ ] Xác định xử lý app chạy nhiều instance; tránh hai instance ghi dữ liệu gây xung đột nếu không hỗ trợ.
- [ ] Accessibility và responsive layout trong kích thước cửa sổ desktop.

### 9.3. Logging và hỗ trợ lỗi

- [ ] Ghi log ở app data/log directory.
- [ ] Có giới hạn kích thước/rotation nếu cần.
- [ ] Không log nội dung note, mood hoặc dữ liệu riêng tư không cần thiết.
- [ ] Hướng dẫn người dùng tìm log khi báo lỗi.
- [ ] Không đưa file DB hoặc log chứa dữ liệu cá nhân vào báo cáo bug công khai.

## Tiêu chí nghiệm thu

- [ ] Ứng dụng không cần Docker, MySQL Server hay ASP.NET Core để chạy.
- [ ] Không có API call cũ còn được dùng.
- [ ] Không còn code/dependency/config thừa có thể xóa an toàn.
- [ ] README mô tả đúng cách chạy, build, cài đặt và lưu dữ liệu.
- [ ] Người dùng hiểu database và backup nằm ở đâu.

## Đầu ra

- Repository được dọn gọn.
- `README.md` và tài liệu kiến trúc/triển khai đã cập nhật.
- Migration matrix không còn feature bắt buộc ở trạng thái pending.

---

# Sprint 10 — Regression, installer và release candidate

## Mục tiêu

Kiểm tra ứng dụng như một sản phẩm mà người dùng cuối sẽ cài đặt, không chỉ như dự án chạy trên máy developer.

## Công việc

### 10.1. Regression checklist

- [ ] App khởi động khi chưa có database.
- [ ] App khởi động với database đã tồn tại.
- [ ] Migrations không phá dữ liệu.
- [ ] CRUD cho tất cả domain.
- [ ] Tags và quan hệ dữ liệu.
- [ ] Dashboard và analytics khớp dữ liệu nguồn.
- [ ] Calendar/daily detail đúng ngày.
- [ ] Search tiếng Việt và ký tự đặc biệt.
- [ ] Focus/Pomodoro/Stopwatch khi app đóng/mở và khi máy sleep.
- [ ] Timezone, tuần/tháng/năm, năm nhuận.
- [ ] Settings được giữ sau restart.
- [ ] Backup/restore và import dữ liệu nếu hỗ trợ.
- [ ] Xử lý database không truy cập được, disk full và backup file không hợp lệ.
- [ ] Không có HTTP request tới backend cũ.
- [ ] Không có chức năng bắt buộc cần Internet.

### 10.2. Kiểm tra installer trên môi trường sạch

- [ ] Build installer từ commit release candidate.
- [ ] Kiểm tra cài đặt trên máy Windows sạch hoặc môi trường test phù hợp.
- [ ] Xác nhận không cần cài Node.js, Rust, Docker, MySQL hoặc chạy API bằng tay.
- [ ] Kiểm tra WebView2/runtime prerequisite theo cấu hình bundler và tài liệu Tauri.
- [ ] Mở app, tạo dữ liệu, thoát, mở lại và xác nhận dữ liệu còn.
- [ ] Kiểm tra nâng cấp từ phiên bản desktop trước đó nếu có.
- [ ] Kiểm tra uninstall không xóa dữ liệu người dùng một cách bất ngờ; mô tả rõ hành vi.
- [ ] Kiểm tra đường dẫn cài đặt có dấu cách/ký tự Unicode.
- [ ] Xác minh icon, tên ứng dụng, shortcut và thông tin version.

### 10.3. Hiệu năng và độ ổn định

- [ ] Đo thời gian mở app trên máy mục tiêu.
- [ ] Đo các truy vấn dashboard/search với dữ liệu mẫu lớn.
- [ ] Xác nhận truy vấn nặng không chặn UI.
- [ ] Kiểm tra thao tác nhanh liên tiếp và lỗi transaction.
- [ ] Kiểm tra database file không phình bất thường do lưu tick hoặc log không cần thiết.
- [ ] Xác minh không tạo request polling tới server cũ.

### 10.4. Release docs

- [ ] Cập nhật `README.md`.
- [ ] Cập nhật hướng dẫn cài đặt, backup/restore và vị trí dữ liệu.
- [ ] Viết changelog cho phiên bản desktop đầu tiên.
- [ ] Ghi rõ hạn chế còn tồn tại.
- [ ] Tạo tag release candidate; chỉ tạo release chính thức khi checklist đạt.

## Tiêu chí nghiệm thu

- [ ] Installer chạy được trên môi trường sạch.
- [ ] Người dùng cài và mở app mà không tự dựng môi trường backend.
- [ ] Regression checklist đã hoàn tất hoặc các mục chưa đạt được ghi rõ là blocker.
- [ ] Dữ liệu tồn tại sau restart và có thể backup/restore.
- [ ] Không còn dependency runtime vào Docker/MySQL/API cũ.
- [ ] Tài liệu release và giới hạn đã rõ.

## Đầu ra

- Windows installer.
- `docs/20-Release-Checklist.md`
- Release tag sau khi nghiệm thu.

---

## 4. Migration matrix — Theo dõi trạng thái từng feature

Tạo hoặc cập nhật `docs/17-Feature-Migration-Matrix.md` trong quá trình làm việc. Dùng trạng thái `pending`, `in-progress`, `migrated`, `verified`.

| Feature/domain | UI hiện có | Rust command/service | SQLite | Test | Trạng thái |
|---|---:|---:|---:|---:|---|
| Tasks | Có | Chưa | Chưa | Chưa | pending |
| Tags | Có | Chưa | Chưa | Chưa | pending |
| Habits | Có | Chưa | Chưa | Chưa | pending |
| Goals | Có | Chưa | Chưa | Chưa | pending |
| Notes / Daily Notes | Có | Chưa | Chưa | Chưa | pending |
| Mood | Có | Chưa | Chưa | Chưa | pending |
| Countdowns | Có | Chưa | Chưa | Chưa | pending |
| Focus sessions | Có | Chưa | Chưa | Chưa | pending |
| Stopwatch | Có | Chưa | Chưa | Chưa | pending |
| Pomodoro | Có | Chưa | Chưa | Chưa | pending |
| Dashboard | Có | Chưa | Chưa | Chưa | pending |
| Calendar / Daily Detail | Có | Chưa | Chưa | Chưa | pending |
| Search | Có | Chưa | Chưa | Chưa | pending |
| Analytics / Statistics | Có | Chưa | Chưa | Chưa | pending |
| Settings / Timezone | Có | Chưa | Chưa | Chưa | pending |
| Backup / Restore | Có API cũ | Chưa | Chưa | Chưa | pending |
| Daily scheduler | Backend cũ | Chưa | Chưa | Chưa | pending |

Trạng thái trong bảng là trạng thái khởi điểm dự kiến của kế hoạch, không phải kết quả xác nhận sau khi thực thi.

---

## 5. Checklist mapping backend cũ sang desktop mới

Dùng bảng này để không bỏ sót controller hoặc domain trong backend hiện tại.

| Backend cũ | Đích mới |
|---|---|
| `TasksController` / `TaskRepository` | Task commands + task service/repository |
| `TagsController` / `TagRepository` | Tag commands + tag service/repository |
| `HabitsController` / `HabitRepository` | Habit commands + habit service/repository |
| `GoalsController` / `GoalRepository` | Goal commands + goal service/repository |
| `DailyNotesController` | Note commands + note service/repository |
| `MoodEntriesController` | Mood commands + mood service/repository |
| `CountdownsController` | Countdown commands + countdown service/repository |
| `FocusSessionsController` | Focus-session commands + service/repository |
| `StopwatchController` | Stopwatch commands + service/repository; không persist từng tick |
| `AnalyticsController` / `AnalyticsService` | Query services và DTO tổng hợp |
| `SearchController` | Search command/service |
| `SettingsController` | Settings commands và persistence local |
| `BackupController` / `BackupService` | Backup/restore service native |
| `DailyDetailController` | Daily-detail query service |
| `DailyTaskScheduler.cs` | Đánh giá lại lifecycle; không chuyển máy móc |
| `DbConnectionFactory.cs` | SQLite initialization/pool trong Rust |
| MySQL migrations | SQLite migrations; giữ lịch sử cũ để tham khảo |
| ASP.NET middleware | Error mapping/logging phù hợp cho Tauri Commands |
| Docker Compose / Dockerfiles / Nginx | Không còn cần cho desktop runtime sau nghiệm thu |

Các tên service/repository Rust là cách tổ chức đề xuất. Hãy xác minh các class thật trong repository trước khi tạo hoặc đổi tên.

---

## 6. Rủi ro chính và cách giảm thiểu

| Rủi ro | Cách giảm thiểu |
|---|---|
| Mất dữ liệu khi đổi database | Giữ backend/schema cũ; backup; import có đối soát; không xóa nguồn cũ sớm |
| Sai khác giữa MySQL và SQLite | Chuyển query theo domain; test với dữ liệu mẫu có kết quả kỳ vọng |
| Lệch ngày hoặc streak do timezone | Chốt quy ước thời gian; test ranh giới ngày/tuần/tháng/năm |
| Viết lại quá nhiều cùng lúc | Vertical slice và feature migration matrix |
| UI bị vỡ khi thay data adapter | Giữ component; chuyển từng feature; kiểm thử luồng hiện tại |
| Rust learning curve | Bắt đầu với một command đơn giản; thống nhất error/model pattern trước khi nhân rộng |
| SQLite bị khóa hoặc query làm UI chậm | Parameterized queries, transaction ngắn, pool/config phù hợp, không block UI |
| Scheduler không chạy khi app đóng | Chốt yêu cầu nền trước; dùng cơ chế OS nếu thật sự cần |
| Tauri permission quá rộng | Capability tối thiểu, review từng plugin/command |
| Installer chỉ chạy trên máy dev | Test môi trường sạch và tài liệu prerequisite rõ |
| Dọn code cũ quá sớm | Chỉ cleanup ở Sprint 9 khi migration matrix đã đạt điều kiện |

---

## 7. Những việc không nên làm

- Không xóa `LifeBoard.API/` ngay Sprint 2 chỉ vì đã tạo được cửa sổ Tauri.
- Không thay MySQL bằng SQLite bằng cách đổi tên driver/connection string rồi giữ nguyên toàn bộ SQL.
- Không chuyển mọi business rule vào React.
- Không cho mỗi component tự gọi `invoke` và tự định nghĩa error handling.
- Không expose filesystem/shell permission rộng chỉ để tiện phát triển.
- Không lưu database trong thư mục cài đặt hoặc commit database vào Git.
- Không ghi một record cho mỗi tick của stopwatch/Pomodoro.
- Không coi việc build thành công là bằng chứng toàn bộ feature đã hoạt động.
- Không xóa test, docs hoặc migration cũ chỉ vì chúng liên quan đến kiến trúc trước.
- Không bổ sung CI/CD vào kế hoạch này; ưu tiên chuyển kiến trúc và hoàn thiện local desktop app.

---

## 8. Điều kiện hoàn tất toàn bộ dự án

Chỉ coi việc chuyển đổi hoàn tất khi tất cả điều kiện bắt buộc sau đạt:

- [ ] LifeBoard chạy như desktop app qua Tauri.
- [ ] React UI giao tiếp với backend native bằng Tauri IPC.
- [ ] Rust xử lý nghiệp vụ và truy cập SQLite.
- [ ] Các feature bắt buộc được đánh dấu `verified` trong migration matrix.
- [ ] Không còn runtime dependency vào Docker, MySQL Server hoặc ASP.NET Core API.
- [ ] Database nằm trong app data directory và tồn tại sau khi restart.
- [ ] Backup/restore đã kiểm thử.
- [ ] Timezone, calendar, streak, focus và analytics đã có test cho các trường hợp biên quan trọng.
- [ ] Installer chạy trên môi trường Windows sạch.
- [ ] README, hướng dẫn cài đặt, vị trí dữ liệu, backup/restore và giới hạn đã cập nhật.
- [ ] Có tag Git trước cleanup và tag release sau nghiệm thu.

**Nguyên tắc kết thúc:** ưu tiên tính đúng đắn, bảo toàn dữ liệu và khả năng cài đặt đơn giản trước việc tối ưu từng mili giây. Tauri + Rust + SQLite loại bỏ được các thành phần server không cần thiết, nhưng chất lượng cuối cùng vẫn phụ thuộc vào việc chuyển đúng business rules, truy vấn, timezone và lifecycle của các tính năng.
