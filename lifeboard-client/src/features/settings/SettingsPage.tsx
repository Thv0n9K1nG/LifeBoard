import React, { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Save, Globe, Clock, RefreshCw, CheckCircle2, Download, Upload, Database } from "lucide-react";
import { useSettings, useUpdateSettings } from "./hooks/useSettings";
import { settingsApi } from "./api/settingsApi";
import { Button } from "../../components/ui/Button";
import { Input } from "../../components/ui/Input";
import { applyTimezone, getStoredTimezone } from "../../stores/timezoneStore";

// ── Danh sách múi giờ phổ biến ──────────────────────────────────────────────
const TIMEZONE_LIST: { value: string; label: string; offset: string }[] = [
  // Việt Nam & Đông Nam Á
  { value: "Asia/Ho_Chi_Minh",  label: "Việt Nam (Hà Nội / TP.HCM)",   offset: "UTC+7"  },
  { value: "Asia/Bangkok",       label: "Thái Lan (Bangkok)",             offset: "UTC+7"  },
  { value: "Asia/Jakarta",       label: "Indonesia (Jakarta)",            offset: "UTC+7"  },
  { value: "Asia/Singapore",     label: "Singapore",                     offset: "UTC+8"  },
  { value: "Asia/Kuala_Lumpur",  label: "Malaysia (Kuala Lumpur)",       offset: "UTC+8"  },
  { value: "Asia/Manila",        label: "Philippines (Manila)",          offset: "UTC+8"  },
  { value: "Asia/Rangoon",       label: "Myanmar (Yangon)",              offset: "UTC+6:30"},
  { value: "Asia/Phnom_Penh",   label: "Campuchia (Phnom Penh)",        offset: "UTC+7"  },
  // Đông Á
  { value: "Asia/Shanghai",      label: "Trung Quốc (Shanghai / Beijing)",offset: "UTC+8" },
  { value: "Asia/Tokyo",         label: "Nhật Bản (Tokyo)",             offset: "UTC+9"  },
  { value: "Asia/Seoul",         label: "Hàn Quốc (Seoul)",             offset: "UTC+9"  },
  { value: "Asia/Taipei",        label: "Đài Loan (Taipei)",            offset: "UTC+8"  },
  // Nam Á
  { value: "Asia/Kolkata",       label: "Ấn Độ (Kolkata / Mumbai)",    offset: "UTC+5:30"},
  { value: "Asia/Dhaka",         label: "Bangladesh (Dhaka)",           offset: "UTC+6"  },
  { value: "Asia/Karachi",       label: "Pakistan (Karachi)",           offset: "UTC+5"  },
  // Trung Đông
  { value: "Asia/Dubai",         label: "UAE (Dubai)",                  offset: "UTC+4"  },
  { value: "Asia/Riyadh",        label: "Ả Rập Saudi (Riyadh)",         offset: "UTC+3"  },
  { value: "Asia/Tehran",        label: "Iran (Tehran)",                offset: "UTC+3:30"},
  // Châu Âu
  { value: "Europe/London",      label: "Anh (London)",                 offset: "UTC+0/+1"},
  { value: "Europe/Paris",       label: "Pháp (Paris)",                 offset: "UTC+1/+2"},
  { value: "Europe/Berlin",      label: "Đức (Berlin)",                 offset: "UTC+1/+2"},
  { value: "Europe/Moscow",      label: "Nga (Moscow)",                 offset: "UTC+3"  },
  // Châu Mỹ
  { value: "America/New_York",   label: "Mỹ - Đông (New York)",         offset: "UTC-5/-4"},
  { value: "America/Chicago",    label: "Mỹ - Trung (Chicago)",         offset: "UTC-6/-5"},
  { value: "America/Denver",     label: "Mỹ - Núi (Denver)",            offset: "UTC-7/-6"},
  { value: "America/Los_Angeles",label: "Mỹ - Tây (Los Angeles)",       offset: "UTC-8/-7"},
  { value: "America/Sao_Paulo",  label: "Brazil (São Paulo)",           offset: "UTC-3"  },
  // Châu Đại Dương
  { value: "Australia/Sydney",   label: "Úc (Sydney)",                  offset: "UTC+10/+11"},
  { value: "Pacific/Auckland",   label: "New Zealand (Auckland)",       offset: "UTC+12/+13"},
  // UTC
  { value: "UTC",                label: "UTC (Chuẩn quốc tế)",          offset: "UTC+0"  },
];

// ── Schema ────────────────────────────────────────────────────────────────────
const settingsSchema = z.object({
  theme: z.enum(["light", "dark"]),
  pomodoroFocusMinutes: z.number().min(1).max(120),
  pomodoroBreakMinutes: z.number().min(1).max(60),
  pomodoroRounds: z.number().min(1).max(10),
  habitGracePeriodDays: z.number().min(0).max(7),
  language: z.string(),
  timezone: z.string(),
});

type SettingsFormData = z.infer<typeof settingsSchema>;

// ── Timezone display helper ───────────────────────────────────────────────────
function getCurrentTimeInTz(tz: string): string {
  try {
    return new Intl.DateTimeFormat("vi-VN", {
      timeZone: tz,
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    }).format(new Date());
  } catch {
    return "--:--:--";
  }
}

function getOffsetLabel(tz: string): string {
  const entry = TIMEZONE_LIST.find(t => t.value === tz);
  return entry?.offset ?? "";
}

// ── Component ─────────────────────────────────────────────────────────────────
export const SettingsPage: React.FC = () => {
  const { data: settings, isLoading } = useSettings();
  const updateSettings = useUpdateSettings();

  // Live clock for the currently-selected timezone in the form
  const [liveTime, setLiveTime] = useState("");
  const [savedTz, setSavedTz] = useState(getStoredTimezone());
  const [showReloadBanner, setShowReloadBanner] = useState(false);
  const [backupMsg, setBackupMsg] = useState<{ text: string; error?: boolean } | null>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);

  const handleExport = async () => {
    try {
      setIsExporting(true);
      const json = await settingsApi.exportBackup();
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `lifeboard-backup-${new Date().toISOString().split("T")[0]}.json`;
      a.click();
      URL.revokeObjectURL(url);
      setBackupMsg({ text: "Xuất dữ liệu sao lưu thành công!" });
    } catch (e: unknown) {
      const err = e as { message?: string };
      setBackupMsg({ text: `Lỗi xuất sao lưu: ${err.message || String(e)}`, error: true });
    } finally {
      setIsExporting(false);
    }
  };

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    if (!window.confirm("Khôi phục sao lưu sẽ thay thế toàn bộ dữ liệu hiện tại bằng bản sao lưu. Bạn có chắc chắn muốn tiếp tục?")) {
      e.target.value = "";
      return;
    }
    try {
      setIsImporting(true);
      const text = await file.text();
      await settingsApi.importBackup(text);
      setBackupMsg({ text: "Khôi phục dữ liệu thành công! Đang tải lại ứng dụng..." });
      setTimeout(() => window.location.reload(), 1200);
    } catch (err: unknown) {
      const error = err as { message?: string };
      setBackupMsg({ text: `Lỗi khôi phục: ${error.message || String(err)}`, error: true });
    } finally {
      setIsImporting(false);
      e.target.value = "";
    }
  };

  const { register, handleSubmit, reset, watch, control, formState: { errors } } = useForm<SettingsFormData>({
    resolver: zodResolver(settingsSchema),
    defaultValues: {
      timezone: getStoredTimezone(),
    },
  });

  const watchedTz = watch("timezone");

  // Sync form when settings loaded from server
  useEffect(() => {
    if (settings) {
      reset({
        ...settings,
        timezone: settings.timezone || getStoredTimezone(),
      });
      // Also sync localStorage with whatever the server has
      applyTimezone(settings.timezone || getStoredTimezone());
      setSavedTz(settings.timezone || getStoredTimezone());
    }
  }, [settings, reset]);

  // Live clock ticker
  useEffect(() => {
    setLiveTime(getCurrentTimeInTz(watchedTz));
    const timer = setInterval(() => setLiveTime(getCurrentTimeInTz(watchedTz)), 1000);
    return () => clearInterval(timer);
  }, [watchedTz]);

  const onSubmit = (data: SettingsFormData) => {
    updateSettings.mutate(data, {
      onSuccess: (saved) => {
        // Apply theme immediately
        document.documentElement.setAttribute("data-theme", data.theme);
        // Apply timezone to localStorage
        applyTimezone(data.timezone);
        setSavedTz(data.timezone);
        // If timezone changed → show reload banner
        if (data.timezone !== savedTz) {
          setShowReloadBanner(true);
        }
      },
    });
  };

  const handleReload = () => window.location.reload();

  if (isLoading) {
    return (
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", padding: 64 }}>
        <div style={{ textAlign: "center", color: "var(--text-muted)" }}>
          <RefreshCw size={32} style={{ animation: "spin 1s linear infinite", marginBottom: 12 }} />
          <div>Đang tải cấu hình...</div>
        </div>
      </div>
    );
  }

  return (
    <div style={{ maxWidth: 720, margin: "0 auto", display: "flex", flexDirection: "column", gap: 20 }}>

      {/* Page header */}
      <div style={{
        background: "linear-gradient(135deg, var(--bg-surface) 0%, var(--bg-elevated) 100%)",
        border: "1px solid var(--border-subtle)",
        borderRadius: "var(--radius-lg)",
        padding: "20px 24px",
        position: "relative", overflow: "hidden",
      }}>
        <div style={{
          position: "absolute", top: -40, right: -40, width: 160, height: 160,
          background: "var(--accent-glow)", borderRadius: "50%", filter: "blur(60px)",
          pointerEvents: "none",
        }} />
        <div style={{ position: "relative" }}>
          <h2 style={{ fontSize: "var(--text-xl)", fontWeight: 700, color: "var(--text-primary)", margin: 0 }}>
            Cấu hình hệ thống
          </h2>
          <p style={{ color: "var(--text-secondary)", fontSize: "var(--text-sm)", marginTop: 4 }}>
            Tùy chỉnh LifeBoard theo cách bạn muốn.
          </p>
        </div>
      </div>

      {/* Reload banner */}
      {showReloadBanner && (
        <div style={{
          display: "flex", alignItems: "center", justifyContent: "space-between",
          gap: 12, padding: "12px 18px",
          background: "rgba(82,215,191,0.08)",
          border: "1px solid rgba(82,215,191,0.3)",
          borderRadius: "var(--radius-md)",
          animation: "slideUp 250ms ease",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
            <CheckCircle2 size={18} color="var(--color-success)" />
            <div>
              <div style={{ fontSize: 13, fontWeight: 600, color: "var(--color-success)" }}>
                Đã lưu! Múi giờ thay đổi cần tải lại trang để áp dụng.
              </div>
              <div style={{ fontSize: 11, color: "var(--text-muted)", marginTop: 2 }}>
                Tất cả ngày/giờ sẽ hiển thị theo múi giờ mới sau khi reload.
              </div>
            </div>
          </div>
          <button
            onClick={handleReload}
            style={{
              display: "flex", alignItems: "center", gap: 6,
              padding: "7px 14px", borderRadius: "var(--radius-md)",
              background: "var(--color-success)", border: "none",
              color: "#fff", fontSize: 12, fontWeight: 700,
              cursor: "pointer", flexShrink: 0,
              transition: "all 150ms",
            }}
            onMouseEnter={e => (e.currentTarget.style.opacity = "0.85")}
            onMouseLeave={e => (e.currentTarget.style.opacity = "1")}
          >
            <RefreshCw size={13} />
            Tải lại trang
          </button>
        </div>
      )}

      <form onSubmit={handleSubmit(onSubmit)} style={{ display: "flex", flexDirection: "column", gap: 16 }}>

        {/* ── Giao diện & Ngôn ngữ ── */}
        <section style={{
          background: "var(--bg-surface)", border: "1px solid var(--border-subtle)",
          borderRadius: "var(--radius-lg)", padding: "20px 24px",
        }}>
          <h3 style={{
            fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)",
            marginBottom: 16, paddingBottom: 12,
            borderBottom: "1px solid var(--border-subtle)",
            display: "flex", alignItems: "center", gap: 8,
          }}>
            <span style={{
              width: 28, height: 28, borderRadius: "var(--radius-sm)",
              background: "var(--accent-subtle)", display: "flex",
              alignItems: "center", justifyContent: "center",
              fontSize: 14,
            }}>🎨</span>
            Giao diện &amp; Ngôn ngữ
          </h3>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Giao diện (Theme)
              </label>
              <select {...register("theme")} style={{ width: "100%" }}>
                <option value="light">☀️  Sáng (Light)</option>
                <option value="dark">🌙  Tối (Dark)</option>
              </select>
            </div>
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Ngôn ngữ
              </label>
              <select {...register("language")} style={{ width: "100%" }}>
                <option value="vi">🇻🇳  Tiếng Việt</option>
                <option value="en">🇺🇸  English</option>
              </select>
            </div>
          </div>
        </section>

        {/* ── Múi giờ ── */}
        <section style={{
          background: "var(--bg-surface)", border: "1px solid var(--border-subtle)",
          borderRadius: "var(--radius-lg)", padding: "20px 24px",
        }}>
          <h3 style={{
            fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)",
            marginBottom: 16, paddingBottom: 12,
            borderBottom: "1px solid var(--border-subtle)",
            display: "flex", alignItems: "center", gap: 8,
          }}>
            <span style={{
              width: 28, height: 28, borderRadius: "var(--radius-sm)",
              background: "rgba(96,165,250,0.15)", display: "flex",
              alignItems: "center", justifyContent: "center",
              fontSize: 14,
            }}>🌏</span>
            Múi giờ (Timezone)
          </h3>

          <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
            {/* Dropdown */}
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Chọn múi giờ của bạn
              </label>
              <Controller
                name="timezone"
                control={control}
                render={({ field }) => (
                  <select
                    {...field}
                    style={{ width: "100%" }}
                  >
                    {TIMEZONE_LIST.map(tz => (
                      <option key={tz.value} value={tz.value}>
                        {tz.offset}  —  {tz.label}
                      </option>
                    ))}
                  </select>
                )}
              />
            </div>

            {/* Live preview card */}
            <div style={{
              display: "flex", alignItems: "center", gap: 16,
              padding: "14px 18px",
              background: "var(--bg-elevated)",
              border: "1px solid var(--border-default)",
              borderRadius: "var(--radius-md)",
              transition: "all 200ms ease",
            }}>
              {/* Clock icon */}
              <div style={{
                width: 44, height: 44, borderRadius: "var(--radius-md)",
                background: "rgba(96,165,250,0.12)",
                border: "1px solid rgba(96,165,250,0.25)",
                display: "flex", alignItems: "center", justifyContent: "center",
                flexShrink: 0,
              }}>
                <Globe size={20} color="var(--color-info)" />
              </div>

              <div style={{ flex: 1 }}>
                <div style={{ fontSize: 12, color: "var(--text-muted)", fontWeight: 500, marginBottom: 3 }}>
                  Giờ hiện tại theo múi giờ này
                </div>
                <div style={{
                  fontSize: 22, fontWeight: 800, fontFamily: "var(--font-mono)",
                  color: "var(--color-info)",
                  letterSpacing: "0.04em",
                  lineHeight: 1.2,
                }}>
                  {liveTime}
                </div>
              </div>

              <div style={{ textAlign: "right" }}>
                <div style={{ fontSize: 11, color: "var(--text-muted)", marginBottom: 3 }}>Độ lệch</div>
                <div style={{
                  fontSize: 13, fontWeight: 700,
                  color: "var(--text-secondary)",
                  background: "var(--bg-overlay)",
                  padding: "3px 8px",
                  borderRadius: "var(--radius-full)",
                  border: "1px solid var(--border-default)",
                }}>
                  {getOffsetLabel(watchedTz)}
                </div>
              </div>
            </div>

            <div style={{
              display: "flex", alignItems: "flex-start", gap: 8,
              padding: "10px 14px",
              background: "rgba(251,191,36,0.06)",
              border: "1px solid rgba(251,191,36,0.2)",
              borderRadius: "var(--radius-md)",
              fontSize: 12,
              color: "var(--text-secondary)",
            }}>
              <span style={{ flexShrink: 0, marginTop: 1 }}>💡</span>
              <span>
                Múi giờ ảnh hưởng đến cách LifeBoard xác định "hôm nay" khi tạo nhiệm vụ, ghi chú,
                và thống kê. Sau khi lưu thay đổi, trang sẽ cần <strong>tải lại</strong> để áp dụng.
              </span>
            </div>
          </div>
        </section>

        {/* ── Pomodoro Timer ── */}
        <section style={{
          background: "var(--bg-surface)", border: "1px solid var(--border-subtle)",
          borderRadius: "var(--radius-lg)", padding: "20px 24px",
        }}>
          <h3 style={{
            fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)",
            marginBottom: 16, paddingBottom: 12,
            borderBottom: "1px solid var(--border-subtle)",
            display: "flex", alignItems: "center", gap: 8,
          }}>
            <span style={{
              width: 28, height: 28, borderRadius: "var(--radius-sm)",
              background: "rgba(248,113,113,0.12)", display: "flex",
              alignItems: "center", justifyContent: "center",
              fontSize: 14,
            }}>🍅</span>
            Pomodoro Timer
          </h3>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 16 }}>
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Thời gian Focus (phút)
              </label>
              <Input
                type="number"
                {...register("pomodoroFocusMinutes", { valueAsNumber: true })}
                error={errors.pomodoroFocusMinutes?.message}
              />
            </div>
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Thời gian nghỉ (phút)
              </label>
              <Input
                type="number"
                {...register("pomodoroBreakMinutes", { valueAsNumber: true })}
                error={errors.pomodoroBreakMinutes?.message}
              />
            </div>
            <div>
              <label style={{ display: "block", fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-secondary)", marginBottom: 6 }}>
                Số vòng lặp (Rounds)
              </label>
              <Input
                type="number"
                {...register("pomodoroRounds", { valueAsNumber: true })}
                error={errors.pomodoroRounds?.message}
              />
            </div>
          </div>
        </section>

        {/* ── Backup & Restore ── */}
        <section style={{
          background: "var(--bg-surface)",
          border: "1px solid var(--border-subtle)",
          borderRadius: "var(--radius-lg)",
          padding: 24,
        }}>
          <h3 style={{
            fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)",
            marginBottom: 16, paddingBottom: 12,
            borderBottom: "1px solid var(--border-subtle)",
            display: "flex", alignItems: "center", gap: 8,
          }}>
            <Database size={18} color="var(--primary)" />
            Sao lưu & Khôi phục dữ liệu (Local SQLite)
          </h3>
          <p style={{ fontSize: "var(--text-sm)", color: "var(--text-secondary)", marginBottom: 16, lineHeight: 1.6 }}>
            Toàn bộ dữ liệu của bạn được lưu an toàn trong file SQLite cục bộ trên máy tính. Bạn có thể xuất file sao lưu JSON bất kỳ lúc nào để chuyển sang máy khác hoặc bảo quản dự phòng.
          </p>

          {backupMsg && (
            <div style={{
              padding: "10px 14px",
              borderRadius: "var(--radius-md)",
              marginBottom: 16,
              fontSize: "var(--text-sm)",
              background: backupMsg.error ? "rgba(239,68,68,0.12)" : "rgba(34,197,94,0.12)",
              color: backupMsg.error ? "var(--priority-high)" : "#10B981",
              border: `1px solid ${backupMsg.error ? "rgba(239,68,68,0.3)" : "rgba(34,197,94,0.3)"}`,
            }}>
              {backupMsg.text}
            </div>
          )}

          <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
            <Button
              type="button"
              variant="secondary"
              onClick={handleExport}
              loading={isExporting}
              leftIcon={<Download size={16} />}
            >
              Xuất file sao lưu (JSON)
            </Button>

            <label style={{ display: "inline-block", cursor: "pointer" }}>
              <input
                type="file"
                accept=".json"
                style={{ display: "none" }}
                onChange={handleImport}
                disabled={isImporting}
              />
              <span style={{
                display: "inline-flex",
                alignItems: "center",
                gap: 8,
                padding: "8px 16px",
                borderRadius: "var(--radius-md)",
                border: "1px solid var(--border-default)",
                background: "var(--bg-elevated)",
                color: "var(--text-primary)",
                fontSize: "var(--text-sm)",
                fontWeight: 500,
                cursor: "pointer",
              }}>
                <Upload size={16} />
                {isImporting ? "Đang khôi phục..." : "Khôi phục từ file JSON"}
              </span>
            </label>
          </div>
        </section>

        {/* ── Save button ── */}
        <div style={{ display: "flex", justifyContent: "flex-end", gap: 10 }}>
          {showReloadBanner && (
            <Button
              type="button"
              variant="secondary"
              onClick={handleReload}
            >
              <RefreshCw size={15} style={{ marginRight: 6 }} />
              Tải lại trang
            </Button>
          )}
          <Button
            type="submit"
            variant="primary"
            loading={updateSettings.isPending}
            leftIcon={<Save size={16} />}
          >
            Lưu thay đổi
          </Button>
        </div>

      </form>
    </div>
  );
};
