# 12 - Hướng dẫn Triển khai & Đóng gói Desktop (Tauri 2)

**Dự án:** LifeBoard — Personal Productivity System  
**Phiên bản:** Desktop 1.0.0 (Native Windows)  
**Kiến trúc:** Tauri 2 · React 19 · Rust · SQLite (Bundled WAL Mode)

---

## 1. Tổng quan Kiến trúc Mới

LifeBoard đã chuyển đổi hoàn toàn thành ứng dụng desktop local-first:
- **Không cần Docker**
- **Không cần MySQL Server**
- **Không cần ASP.NET Core API**
- Toàn bộ cơ sở dữ liệu được lưu cục bộ trong một file SQLite duy nhất tại: `%APPDATA%\LifeBoard\lifeboard.db`.

---

## 2. Yêu cầu Môi trường

### 2.1 Đối với Người dùng Cuối (End-Users)
- **Hệ điều hành:** Windows 10 (1809 trở lên) hoặc Windows 11.
- **Runtime:** Microsoft Edge WebView2 Evergreen Runtime (đã tích hợp sẵn mặc định trên Windows 10/11 hiện đại).
- **Yêu cầu khác:** Không cần cài bất kỳ dependency kỹ thuật nào (Node, Rust, Docker...).

### 2.2 Đối với Lập trình viên & Build Máy chủ (Developers & CI)
- **Node.js:** v20.x trở lên
- **Rust Toolchain:** v1.80.x trở lên (với target `x86_64-pc-windows-msvc`)
- **Build Tools:** Visual Studio C++ Build Tools (MSVC)
- **Tauri CLI:** `@tauri-apps/cli` (v2.x)

---

## 3. Khởi chạy Môi trường Phát triển (Development)

1. **Cài đặt thư viện frontend:**
   ```bash
   cd lifeboard-client
   npm install
   ```

2. **Khởi chạy ứng dụng Desktop (Vite HMR + Tauri Window):**
   ```bash
   # Từ thư mục gốc:
   npm run tauri:dev
   ```
   Ứng dụng sẽ tự động biên dịch backend Rust, khởi chạy Vite dev server và mở cửa sổ desktop LifeBoard.

3. **Chạy kiểm thử backend (Unit & Integration Tests):**
   ```bash
   cargo test --manifest-path src-tauri/Cargo.toml
   ```

---

## 4. Quy trình Đóng gói Phát hành (Production Build)

Để đóng gói bản cài đặt độc lập (Standalone Installer & Executable):

1. **Build bản phát hành:**
   ```bash
   npm run tauri:build
   ```

2. **Thư mục đầu ra:**
   - **File thực thi di động (Standalone EXE):**
     `src-tauri/target/release/LifeBoard.exe`
   - **Bộ cài Windows MSI:**
     `src-tauri/target/release/bundle/msi/LifeBoard_*_x64_en-US.msi`
   - **Bộ cài Windows NSIS (Setup EXE):**
     `src-tauri/target/release/bundle/nsis/LifeBoard_*_x64-setup.exe`

---

## 5. Quản lý Dữ liệu & Sao lưu

- **Vị trí file dữ liệu:** `%APPDATA%\LifeBoard\lifeboard.db`
- **Sao lưu:**
  - Người dùng có thể sao chép thủ công file `lifeboard.db` (đảm bảo app đã tắt để tránh SQLite write lock).
  - Hoặc sử dụng tính năng **Xuất bản sao lưu JSON** trong phần **Cài đặt -> Sao lưu & Phục hồi** trên giao diện ứng dụng.
- **Gỡ cài đặt (Uninstall):** Thao tác gỡ cài đặt sẽ không xóa thư mục `%APPDATA%\LifeBoard` nhằm bảo vệ dữ liệu cá nhân của người dùng.
