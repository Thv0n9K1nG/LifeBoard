import React, { useState, useEffect, useMemo } from "react";
import { Play, Pause, Square, List, Calendar, Trash2, Tag } from "lucide-react";
import { useSettings } from "../settings/hooks/useSettings";
import { useCreateFocusSession, useFocusSessions } from "./hooks/useFocus";
import { useStopwatch } from "./hooks/useStopwatch";
import { Button } from "../../components/ui/Button";
import { focusApi } from "./api/focusApi";
import { toLocalDateString } from "@/lib/utils";

interface FocusPageProps {
  defaultMode?: 'stopwatch' | 'pomodoro';
}

// ── Split type for display only (supports camelCase and PascalCase) ────────
interface SplitDisplay {
  id?: number;
  Id?: number;
  elapsed?: string;
  Elapsed?: string;
  interval?: string;
  Interval?: string;
  timestamp?: string;
  Timestamp?: string;
  note?: string;
  Note?: string;
}

const getSplitTime = (s: SplitDisplay): string => {
  return s.interval ?? s.Interval ?? s.elapsed ?? s.Elapsed ?? "—";
};

const getSplitInfo = (s: SplitDisplay): string => {
  const note = (s.note ?? s.Note)?.trim();
  const ts = s.timestamp ?? s.Timestamp;

  let formattedTs: string | undefined;
  if (ts) {
    formattedTs = new Date(ts).toLocaleTimeString("vi-VN") !== "Invalid Date"
      ? new Date(ts).toLocaleTimeString("vi-VN")
      : ts;
  }

  if (note && formattedTs) return `${note} (${formattedTs})`;
  if (note) return note;
  if (formattedTs) return formattedTs;
  return "—";
};

export const FocusPage: React.FC<FocusPageProps> = ({ defaultMode = 'pomodoro' }) => {
  const { data: settings } = useSettings();
  const createSession = useCreateFocusSession();

  const [mode, setMode] = useState<'stopwatch' | 'pomodoro'>(defaultMode);

  // ── Pomodoro-only state ───────────────────────────────────────────────────
  const [isActive, setIsActive]         = useState(false);
  const [pomodoroSeconds, setPomodoroSeconds] = useState(25 * 60);
  const [pomodoroLabel, setPomodoroLabel] = useState("");
  const [pomodoroStart, setPomodoroStart] = useState<Date | null>(null);

  // ── Backend-authoritative stopwatch ──────────────────────────────────────
  const sw = useStopwatch();
  // Local label — only used BEFORE a session starts (not backed by API)
  // After start(), label ownership moves to the server via sw.label
  const [localLabel, setLocalLabel] = useState("");

  // ── Query by date history state ───────────────────────────────────────────
  const [queryDate, setQueryDate] = useState<string>(toLocalDateString(new Date()));
  const { data: historySessions, refetch: refetchHistory } = useFocusSessions(queryDate, queryDate);

  // ── Pomodoro default from settings ───────────────────────────────────────
  useEffect(() => {
    if (!isActive && settings && mode === "pomodoro") {
      setPomodoroSeconds(settings.pomodoroFocusMinutes * 60);
    }
  }, [settings, mode, isActive]);

  // ── Pomodoro interval ─────────────────────────────────────────────────────
  useEffect(() => {
    let interval: ReturnType<typeof setInterval> | null = null;
    if (isActive && mode === "pomodoro") {
      interval = setInterval(() => {
        setPomodoroSeconds(prev => (prev > 0 ? prev - 1 : 0));
      }, 1000);
    }
    return () => { if (interval) clearInterval(interval); };
  }, [isActive, mode]);

  // ── Auto-stop Pomodoro ────────────────────────────────────────────────────
  useEffect(() => {
    if (mode === "pomodoro" && isActive && pomodoroSeconds === 0) handlePomodoroStop();
  }, [pomodoroSeconds, mode, isActive]);

  // ── Refetch history when stopwatch stops ──────────────────────────────────
  useEffect(() => {
    if (sw.status === 'idle') {
      refetchHistory();
    }
  }, [sw.status, refetchHistory]);

  // ── Format helpers ────────────────────────────────────────────────────────
  const fmtPomodoro = (s: number) =>
    `${Math.floor(s / 60).toString().padStart(2, "0")}:${(s % 60).toString().padStart(2, "0")}`;

  const fmtMs = (ms: number): string => {
    const h  = Math.floor(ms / 3_600_000).toString().padStart(2, "0");
    const m  = Math.floor((ms % 3_600_000) / 60_000).toString().padStart(2, "0");
    const s  = Math.floor((ms % 60_000) / 1_000).toString().padStart(2, "0");
    const ms3 = (ms % 1_000).toString().padStart(3, "0");
    return `${h}:${m}:${s}.${ms3}`;
  };

  const fmtTotalTime = (totalSec: number): string => {
    const h = Math.floor(totalSec / 3600);
    const m = Math.floor((totalSec % 3600) / 60);
    const s = totalSec % 60;
    const parts: string[] = [];
    if (h > 0) parts.push(`${h} giờ`);
    if (m > 0) parts.push(`${m} phút`);
    parts.push(`${s} giây`);
    return parts.join(" ");
  };

  // ── Derived data ──────────────────────────────────────────────────────────
  const displaySessions = useMemo(() => {
    if (!historySessions?.data) return [];
    // Exclude the currently active stopwatch session (it's shown live above)
    return historySessions.data.filter(s =>
      !(s.sessionType === "stopwatch" && s.id === sw.sessionId)
    );
  }, [historySessions, sw.sessionId]);

  const totalTodaySeconds = useMemo(() => {
    const completed = displaySessions.reduce((acc, s) => acc + s.durationSeconds, 0);
    const liveStopwatch = sw.status !== 'idle' ? Math.floor(sw.elapsedMs / 1000) : 0;
    const livePomodoro  = mode === "pomodoro" && isActive
      ? (settings?.pomodoroFocusMinutes ?? 25) * 60 - pomodoroSeconds
      : 0;
    return completed + liveStopwatch + livePomodoro;
  }, [displaySessions, sw.elapsedMs, sw.status, mode, isActive, pomodoroSeconds, settings]);

  // ── Parse splits for display ──────────────────────────────────────────────
  const liveSplits: SplitDisplay[] = useMemo(() => {
    if (!sw.splitsJson) return [];
    try { return JSON.parse(sw.splitsJson); } catch { return []; }
  }, [sw.splitsJson]);

  // ── Stopwatch actions ─────────────────────────────────────────────────────
  const handleSwToggle = async () => {
    if (sw.status === 'idle') {
      await sw.start(localLabel.trim() || undefined, toLocalDateString(new Date()));
      setLocalLabel(""); // clear local label after handing off to server
      return;
    }
    if (sw.status === 'running') return sw.pause();
    if (sw.status === 'paused')  return sw.resume();
  };

  const handleSwStop = async () => {
    await sw.stop();
    refetchHistory();
  };

  const handleSwSplit = async () => {
    if (sw.status === 'running') await sw.addSplit();
  };

  // ── Pomodoro actions ──────────────────────────────────────────────────────
  const handlePomodoroToggle = () => {
    if (!isActive) {
      if (!pomodoroStart) setPomodoroStart(new Date());
      setIsActive(true);
    } else {
      setIsActive(false);
    }
  };

  const handlePomodoroStop = () => {
    setIsActive(false);
    if (pomodoroStart) {
      createSession.mutate({
        sessionType: mode,
        label: pomodoroLabel.trim() || undefined,
        startTime: pomodoroStart.toISOString(),
        endTime: new Date().toISOString(),
        sessionDate: toLocalDateString(pomodoroStart),
      }, {
        onSuccess: () => {
          setPomodoroStart(null);
          setPomodoroLabel("");
          setPomodoroSeconds((settings?.pomodoroFocusMinutes || 25) * 60);
          refetchHistory();
          alert("Lưu phiên tập trung thành công!");
        },
      });
    }
  };

  // ── Delete session ────────────────────────────────────────────────────────
  const handleDeleteSession = async (id: number) => {
    if (!window.confirm("Bạn có chắc chắn muốn xóa phiên tập trung này?")) return;
    try {
      await focusApi.delete(id);
      refetchHistory();
    } catch {
      alert("Lỗi khi xóa phiên tập trung.");
    }
  };

  // ── Render ────────────────────────────────────────────────────────────────
  const isStopwatch = mode === "stopwatch";
  const swStarted   = sw.status !== 'idle';
  const swRunning   = sw.status === 'running';

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 24 }}>

      {/* ── MAIN CARD ── */}
      <div style={{
        background: "var(--bg-surface)",
        border: "1px solid var(--border-subtle)",
        borderRadius: "var(--radius-lg)",
        boxShadow: "var(--shadow-sm)",
        overflow: "hidden",
      }}>
        {/* Running accent bar */}
        {(isStopwatch ? swRunning : isActive) && (
          <div style={{
            height: 3,
            background: "linear-gradient(90deg, var(--accent-primary), var(--color-success))",
            animation: "pulse 2s infinite",
          }} />
        )}

        <div style={{ padding: "32px 32px 28px" }}>

          {/* Mode tabs */}
          <div style={{ display: "flex", justifyContent: "center", gap: 12, marginBottom: 28 }}>
            <Button
              variant={mode === "pomodoro" ? "primary" : "ghost"}
              onClick={() => { if (!isActive && sw.status === 'idle') setMode("pomodoro"); }}
            >
              Pomodoro
            </Button>
            <Button
              variant={mode === "stopwatch" ? "primary" : "ghost"}
              onClick={() => { if (!isActive && sw.status === 'idle') setMode("stopwatch"); }}
            >
              Stopwatch
            </Button>
          </div>

          {/* Session name input */}
          {isStopwatch ? (
            !swStarted ? (
              // Pre-session: label is local state only, NOT sent to API until Start is clicked
              <div style={{ marginBottom: 20, display: "flex", alignItems: "center", gap: 10, maxWidth: 400, margin: "0 auto 20px" }}>
                <Tag size={16} style={{ color: "var(--text-muted)", flexShrink: 0 }} />
                <input
                  type="text"
                  placeholder="Đặt tên cho phiên này (tuỳ chọn)..."
                  value={localLabel}
                  onChange={e => setLocalLabel(e.target.value)}
                  style={{
                    flex: 1,
                    background: "var(--bg-overlay)", border: "1px solid var(--border-default)",
                    borderRadius: "var(--radius-md)", color: "var(--text-primary)",
                    padding: "8px 12px", fontSize: "14px", outline: "none",
                  }}
                />
              </div>
            ) : sw.label ? (
              // Active session: show server-persisted label
              <div style={{ textAlign: "center", marginBottom: 12, fontSize: 13, color: "var(--text-muted)", letterSpacing: "0.04em" }}>
                <Tag size={12} style={{ display: "inline", marginRight: 5, verticalAlign: "middle" }} />
                {sw.label}
              </div>
            ) : null
          ) : (
            !pomodoroStart ? (
              <div style={{ marginBottom: 20, display: "flex", alignItems: "center", gap: 10, maxWidth: 400, margin: "0 auto 20px" }}>
                <Tag size={16} style={{ color: "var(--text-muted)", flexShrink: 0 }} />
                <input
                  type="text"
                  placeholder="Đặt tên cho phiên này (tuỳ chọn)..."
                  value={pomodoroLabel}
                  onChange={e => setPomodoroLabel(e.target.value)}
                  style={{
                    flex: 1,
                    background: "var(--bg-overlay)", border: "1px solid var(--border-default)",
                    borderRadius: "var(--radius-md)", color: "var(--text-primary)",
                    padding: "8px 12px", fontSize: "14px", outline: "none",
                  }}
                />
              </div>
            ) : pomodoroLabel ? (
              <div style={{ textAlign: "center", marginBottom: 12, fontSize: 13, color: "var(--text-muted)", letterSpacing: "0.04em" }}>
                <Tag size={12} style={{ display: "inline", marginRight: 5, verticalAlign: "middle" }} />
                {pomodoroLabel}
              </div>
            ) : null
          )}

          {/* Clock display */}
          <div style={{
            textAlign: "center", fontFamily: "monospace", fontWeight: 700,
            lineHeight: 1, letterSpacing: "-0.02em",
            fontSize: isStopwatch ? "clamp(52px, 8vw, 90px)" : "clamp(64px, 10vw, 120px)",
            color: "var(--text-primary)", padding: "20px 0 24px",
            textShadow: (isStopwatch ? swRunning : isActive) ? "0 0 24px var(--accent-glow)" : "none",
            transition: "text-shadow 0.3s",
          }}>
            {isStopwatch ? fmtMs(sw.elapsedMs) : fmtPomodoro(pomodoroSeconds)}
          </div>

          {/* Error display */}
          {isStopwatch && sw.error && (
            <div style={{ textAlign: "center", color: "var(--color-danger)", fontSize: 12, marginBottom: 8 }}>
              ⚠ {sw.error}
            </div>
          )}

          {/* Buttons row */}
          <div style={{ display: "flex", justifyContent: "center", alignItems: "center", gap: 12, flexWrap: "wrap" }}>
            {isStopwatch ? (
              !swStarted ? (
                <Button size="lg" variant="primary" onClick={handleSwToggle} style={{ minWidth: 140 }} disabled={sw.isLoading}>
                  <Play size={18} style={{ marginRight: 8 }} /> Bắt đầu
                </Button>
              ) : (
                <>
                  <Button size="lg" variant={swRunning ? "secondary" : "primary"} onClick={handleSwToggle} style={{ minWidth: 140 }} disabled={sw.isLoading}>
                    {swRunning
                      ? <><Pause size={18} style={{ marginRight: 8 }} /> Tạm dừng</>
                      : <><Play size={18} style={{ marginRight: 8 }} /> Tiếp tục</>}
                  </Button>
                  {swRunning && (
                    <Button size="lg" variant="secondary" onClick={handleSwSplit} style={{ minWidth: 100 }} disabled={sw.isLoading}>
                      <List size={18} style={{ marginRight: 8 }} /> Split
                    </Button>
                  )}
                  <Button size="lg" variant="danger" onClick={handleSwStop} style={{ minWidth: 110 }} disabled={sw.isLoading}>
                    <Square size={18} style={{ marginRight: 8 }} /> Reset
                  </Button>
                </>
              )
            ) : (
              <>
                <Button size="lg" variant={isActive ? "secondary" : "primary"} onClick={handlePomodoroToggle} style={{ minWidth: 140 }}>
                  {isActive
                    ? <><Pause size={18} style={{ marginRight: 8 }} /> Tạm dừng</>
                    : <><Play size={18} style={{ marginRight: 8 }} /> Bắt đầu</>}
                </Button>
                {(isActive || pomodoroStart) && (
                  <Button size="lg" variant="danger" onClick={handlePomodoroStop} style={{ minWidth: 140 }}>
                    <Square size={18} style={{ marginRight: 8 }} /> Kết thúc
                  </Button>
                )}
              </>
            )}
          </div>

          {/* Total time bar */}
          <div style={{
            marginTop: 24, padding: "12px 20px",
            background: "var(--bg-overlay)", borderRadius: "var(--radius-md)",
            border: "1px solid var(--border-subtle)",
            display: "flex", justifyContent: "space-between", alignItems: "center",
          }}>
            <span style={{ fontSize: 13, fontWeight: 600, color: "var(--text-secondary)" }}>
              Tổng thời gian tập trung hôm nay
            </span>
            <span style={{ fontSize: 15, fontWeight: 700, color: "var(--accent-primary)", fontFamily: "monospace" }}>
              {fmtTotalTime(totalTodaySeconds)}
            </span>
          </div>
        </div>

        {/* Splits section — stopwatch live splits */}
        {isStopwatch && liveSplits.length > 0 && (
          <div style={{ borderTop: "1px solid var(--border-subtle)" }}>
            <div style={{ padding: "14px 32px 8px", display: "flex", alignItems: "center", gap: 6 }}>
              <List size={14} style={{ color: "var(--text-muted)" }} />
              <span style={{ fontSize: 12, fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
                Splits ({liveSplits.length})
              </span>
            </div>
            <div style={{ maxHeight: 210, overflowY: "auto" }}>
              <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 12 }}>
                <thead>
                  <tr style={{ background: "var(--bg-overlay)", position: "sticky", top: 0 }}>
                    <th style={{ padding: "6px 20px", textAlign: "left", fontWeight: 600, color: "var(--text-muted)", width: 40 }}>#</th>
                    <th style={{ padding: "6px 12px", textAlign: "center", fontWeight: 600, color: "var(--text-muted)", whiteSpace: "nowrap" }}>Thời gian</th>
                    <th style={{ padding: "6px 20px", textAlign: "left", fontWeight: 600, color: "var(--text-muted)" }}>Mốc thời gian / Ghi chú</th>
                  </tr>
                </thead>
                <tbody>
                  {/* Live Splits sub-table */}
                  {liveSplits.map((s, idx) => {
                    return (
                      <tr key={s.id || s.Id || idx} style={{ borderTop: "1px solid var(--border-subtle)", background: idx % 2 === 1 ? "var(--bg-overlay)" : "transparent" }}>
                        <td style={{ padding: "7px 20px", color: "var(--text-muted)", fontWeight: 600 }}>{idx + 1}</td>
                        <td style={{ padding: "7px 12px", textAlign: "center", fontFamily: "monospace", fontWeight: 700, color: "var(--accent-primary)", whiteSpace: "nowrap" }}>
                          {getSplitTime(s)}
                        </td>
                        <td style={{ padding: "7px 20px", color: "var(--text-secondary)" }}>{getSplitInfo(s)}</td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
            <div style={{ height: 4 }} />
          </div>
        )}
      </div>

      {/* ── HISTORY CARD ── */}
      <div style={{
        background: "var(--bg-surface)", border: "1px solid var(--border-subtle)",
        borderRadius: "var(--radius-lg)", boxShadow: "var(--shadow-sm)", padding: "24px 28px",
      }}>
        {/* Header */}
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 20 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
            <Calendar size={18} style={{ color: "var(--accent-primary)" }} />
            <h3 style={{ fontSize: 16, fontWeight: 700, color: "var(--text-primary)", margin: 0 }}>
              Lịch sử tập trung
            </h3>
          </div>
          <input
            type="date" value={queryDate}
            onChange={e => setQueryDate(e.target.value)}
            style={{
              background: "var(--bg-overlay)", border: "1px solid var(--border-default)",
              borderRadius: "var(--radius-md)", color: "var(--text-primary)",
              padding: "5px 10px", fontSize: 13, outline: "none",
            }}
          />
        </div>

        {displaySessions.length > 0 ? (
          <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
            {displaySessions.map(session => {
              const sessionSplits: SplitDisplay[] = session.splits ? (() => { try { return JSON.parse(session.splits!); } catch { return []; } })() : [];
              const durH = Math.floor(session.durationSeconds / 3600);
              const durM = Math.floor((session.durationSeconds % 3600) / 60);
              const durS = session.durationSeconds % 60;
              const durStr = [
                durH > 0 ? `${durH}g` : "",
                durM > 0 ? `${durM}p` : "",
                `${durS}s`,
              ].filter(Boolean).join(" ");

              return (
                <div key={session.id} style={{
                  background: "var(--bg-overlay)", border: "1px solid var(--border-subtle)",
                  borderRadius: "var(--radius-md)", overflow: "hidden",
                }}>
                  {/* Session header row */}
                  <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", padding: "12px 16px", gap: 12 }}>
                    <div style={{ display: "flex", alignItems: "center", gap: 10, minWidth: 0 }}>
                      <span style={{
                        flexShrink: 0, fontSize: 11, fontWeight: 700, padding: "2px 8px", borderRadius: 99,
                        background: session.sessionType === "stopwatch" ? "rgba(61,142,240,0.15)" : "rgba(239,68,68,0.15)",
                        color: session.sessionType === "stopwatch" ? "var(--accent-primary)" : "var(--color-danger)",
                      }}>
                        {session.sessionType === "stopwatch" ? "Stopwatch" : "Pomodoro"}
                      </span>
                      <span style={{ fontWeight: 600, fontSize: 14, color: "var(--text-primary)", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                        {session.label || "Không có nhãn"}
                      </span>
                    </div>
                    <div style={{ display: "flex", alignItems: "center", gap: 16, flexShrink: 0 }}>
                      <div style={{ textAlign: "right" }}>
                        <div style={{ fontSize: 11, color: "var(--text-muted)" }}>
                          {new Date(session.startTime).toLocaleTimeString("vi-VN")}
                          {session.endTime ? ` → ${new Date(session.endTime).toLocaleTimeString("vi-VN")}` : " → đang chạy"}
                        </div>
                        <div style={{ fontSize: 13, fontWeight: 700, color: "var(--text-primary)", fontFamily: "monospace" }}>
                          ⏱ {durStr}
                        </div>
                      </div>
                      <button
                        onClick={() => handleDeleteSession(session.id)}
                        title="Xóa phiên"
                        style={{ background: "none", border: "none", color: "var(--color-danger)", cursor: "pointer", padding: 4, opacity: 0.6, lineHeight: 1 }}
                        onMouseEnter={e => (e.currentTarget.style.opacity = "1")}
                        onMouseLeave={e => (e.currentTarget.style.opacity = "0.6")}
                      >
                        <Trash2 size={15} />
                      </button>
                    </div>
                  </div>

                  {/* Splits sub-table */}
                  {sessionSplits.length > 0 && (
                    <div style={{ borderTop: "1px dashed var(--border-subtle)" }}>
                      <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 11 }}>
                        <thead>
                          <tr style={{ background: "var(--bg-base)" }}>
                            <th style={{ padding: "4px 20px", fontWeight: 600, color: "var(--text-muted)", textAlign: "left", width: 40 }}>#</th>
                            <th style={{ padding: "4px 12px", fontWeight: 600, color: "var(--text-muted)", textAlign: "center" }}>Thời gian</th>
                            <th style={{ padding: "4px 20px", fontWeight: 600, color: "var(--text-muted)", textAlign: "left" }}>Mốc thời gian / Ghi chú</th>
                          </tr>
                        </thead>
                        <tbody>
                          {sessionSplits.map((s, idx) => {
                            return (
                              <tr key={s.id || s.Id || idx} style={{ borderTop: "1px solid var(--border-subtle)" }}>
                                <td style={{ padding: "4px 20px", color: "var(--text-muted)", fontWeight: 600 }}>{idx + 1}</td>
                                <td style={{ padding: "4px 12px", textAlign: "center", fontFamily: "monospace", fontWeight: 600, color: "var(--accent-primary)" }}>
                                  {getSplitTime(s)}
                                </td>
                                <td style={{ padding: "4px 20px", color: "var(--text-secondary)" }}>{getSplitInfo(s)}</td>
                              </tr>
                            );
                          })}
                        </tbody>
                      </table>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        ) : (
          <div style={{ textAlign: "center", padding: "36px 0", color: "var(--text-muted)", fontSize: 14 }}>
            Không có dữ liệu phiên tập trung nào trong ngày này.
          </div>
        )}
      </div>
    </div>
  );
};
