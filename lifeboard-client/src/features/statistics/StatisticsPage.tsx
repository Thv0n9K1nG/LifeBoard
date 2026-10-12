import React, { useState } from "react";
import { BarChart3, CheckCircle2, Clock, Flame, Smile, Calendar } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { analyticsApi, Statistics } from "../analytics/api/analyticsApi";

export const StatisticsPage: React.FC = () => {
  const today = new Date();
  const thirtyDaysAgo = new Date();
  thirtyDaysAgo.setDate(today.getDate() - 30);

  const [from, setFrom] = useState(thirtyDaysAgo.toISOString().split("T")[0]);
  const [to, setTo] = useState(today.toISOString().split("T")[0]);

  const { data: stats, isLoading } = useQuery({
    queryKey: ["statistics", from, to],
    queryFn: () => analyticsApi.getStatistics(from, to),
  });

  return (
    <div style={{ maxWidth: 1000, margin: "0 auto" }}>
      {/* Header & Date Range */}
      <div style={{
        padding: "var(--space-6)",
        background: "var(--bg-surface)",
        borderRadius: "var(--radius-lg)",
        border: "1px solid var(--border-subtle)",
        marginBottom: "var(--space-5)",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        flexWrap: "wrap",
        gap: 16,
      }}>
        <div>
          <h2 style={{ fontSize: "var(--text-xl)", fontWeight: 700, color: "var(--text-primary)", marginBottom: 4 }}>
            Thống kê hiệu suất
          </h2>
          <p style={{ color: "var(--text-secondary)", fontSize: "var(--text-sm)" }}>
            Phân tích số liệu tổng hợp trong khoảng thời gian
          </p>
        </div>

        <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: "var(--text-sm)", color: "var(--text-secondary)" }}>
            <Calendar size={15} />
            <span>Từ:</span>
            <input
              type="date"
              value={from}
              onChange={(e) => setFrom(e.target.value)}
              style={{
                background: "var(--bg-elevated)",
                border: "1px solid var(--border-default)",
                borderRadius: "var(--radius-sm)",
                padding: "4px 8px",
                color: "var(--text-primary)",
                fontSize: "var(--text-sm)",
              }}
            />
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: "var(--text-sm)", color: "var(--text-secondary)" }}>
            <span>Đến:</span>
            <input
              type="date"
              value={to}
              onChange={(e) => setTo(e.target.value)}
              style={{
                background: "var(--bg-elevated)",
                border: "1px solid var(--border-default)",
                borderRadius: "var(--radius-sm)",
                padding: "4px 8px",
                color: "var(--text-primary)",
                fontSize: "var(--text-sm)",
              }}
            />
          </div>
        </div>
      </div>

      {isLoading || !stats ? (
        <div style={{ padding: 60, textAlign: "center", color: "var(--text-muted)" }}>
          Đang tải số liệu thống kê...
        </div>
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
          {/* Summary KPI Cards */}
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 16 }}>
            {/* Task KPI */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <div style={{ display: "flex", alignItems: "center", gap: 10, color: "var(--text-secondary)", fontSize: "var(--text-sm)" }}>
                <CheckCircle2 size={18} color="var(--primary)" />
                <span>Hoàn thành nhiệm vụ</span>
              </div>
              <div style={{ fontSize: "var(--text-2xl)", fontWeight: 700, color: "var(--text-primary)", marginTop: 8 }}>
                {stats.tasks.completed} <span style={{ fontSize: "var(--text-sm)", fontWeight: 400, color: "var(--text-muted)" }}>/ {stats.tasks.total}</span>
              </div>
              <div style={{ fontSize: "var(--text-xs)", color: "var(--text-muted)", marginTop: 4 }}>
                Tỉ lệ đạt: {Math.round(stats.tasks.completionRate * 100)}%
              </div>
            </div>

            {/* Focus KPI */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <div style={{ display: "flex", alignItems: "center", gap: 10, color: "var(--text-secondary)", fontSize: "var(--text-sm)" }}>
                <Clock size={18} color="#3B82F6" />
                <span>Thời gian tập trung</span>
              </div>
              <div style={{ fontSize: "var(--text-2xl)", fontWeight: 700, color: "var(--text-primary)", marginTop: 8 }}>
                {Math.round((stats.focus.totalMinutes / 60) * 10) / 10} <span style={{ fontSize: "var(--text-sm)", fontWeight: 400, color: "var(--text-muted)" }}>giờ</span>
              </div>
              <div style={{ fontSize: "var(--text-xs)", color: "var(--text-muted)", marginTop: 4 }}>
                {stats.focus.totalSessions} phiên • TB {stats.focus.avgMinutesPerDay} phút/ngày
              </div>
            </div>

            {/* Habit Streak KPI */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <div style={{ display: "flex", alignItems: "center", gap: 10, color: "var(--text-secondary)", fontSize: "var(--text-sm)" }}>
                <Flame size={18} color="#F97316" />
                <span>Chuỗi dài nhất</span>
              </div>
              <div style={{ fontSize: "var(--text-2xl)", fontWeight: 700, color: "var(--text-primary)", marginTop: 8 }}>
                {stats.habits.longestStreak ? stats.habits.longestStreak.streak : 0} <span style={{ fontSize: "var(--text-sm)", fontWeight: 400, color: "var(--text-muted)" }}>ngày</span>
              </div>
              <div style={{ fontSize: "var(--text-xs)", color: "var(--text-muted)", marginTop: 4 }}>
                {stats.habits.longestStreak ? stats.habits.longestStreak.habitName : "Chưa có chuỗi"}
              </div>
            </div>

            {/* Mood KPI */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <div style={{ display: "flex", alignItems: "center", gap: 10, color: "var(--text-secondary)", fontSize: "var(--text-sm)" }}>
                <Smile size={18} color="#EAB308" />
                <span>Điểm tâm trạng TB</span>
              </div>
              <div style={{ fontSize: "var(--text-2xl)", fontWeight: 700, color: "var(--text-primary)", marginTop: 8 }}>
                {stats.mood.avgScore} <span style={{ fontSize: "var(--text-sm)", fontWeight: 400, color: "var(--text-muted)" }}>/ 5.0</span>
              </div>
              <div style={{ fontSize: "var(--text-xs)", color: "var(--text-muted)", marginTop: 4 }}>
                Dựa trên nhật ký tâm trạng
              </div>
            </div>
          </div>

          {/* Breakdown Sections */}
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 20 }}>
            {/* Tasks by priority */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <h3 style={{ fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)", marginBottom: 16 }}>
                Nhiệm vụ theo độ ưu tiên
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
                {[
                  { key: "high", label: "Ưu tiên Cao", color: "var(--priority-high)" },
                  { key: "medium", label: "Ưu tiên Vừa", color: "var(--priority-medium)" },
                  { key: "low", label: "Ưu tiên Thấp", color: "var(--priority-low)" },
                ].map((item) => {
                  const count = stats.tasks.byPriority[item.key] || 0;
                  const pct = stats.tasks.total > 0 ? Math.round((count / stats.tasks.total) * 100) : 0;
                  return (
                    <div key={item.key}>
                      <div style={{ display: "flex", justifyContent: "space-between", fontSize: "var(--text-xs)", marginBottom: 4 }}>
                        <span style={{ color: "var(--text-secondary)" }}>{item.label}</span>
                        <span style={{ fontWeight: 600, color: "var(--text-primary)" }}>{count} ({pct}%)</span>
                      </div>
                      <div style={{ height: 8, background: "var(--bg-elevated)", borderRadius: 4, overflow: "hidden" }}>
                        <div style={{ height: "100%", width: `${pct}%`, background: item.color, borderRadius: 4 }} />
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Focus by session type */}
            <div style={{
              background: "var(--bg-surface)",
              border: "1px solid var(--border-subtle)",
              borderRadius: "var(--radius-lg)",
              padding: 20,
            }}>
              <h3 style={{ fontSize: "var(--text-md)", fontWeight: 600, color: "var(--text-primary)", marginBottom: 16 }}>
                Phiên tập trung theo loại
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: 14 }}>
                {[
                  { key: "pomodoro", label: "Pomodoro Timer", icon: "🍅", color: "var(--priority-high)" },
                  { key: "stopwatch", label: "Stopwatch", icon: "⏱️", color: "#3B82F6" },
                ].map((item) => {
                  const count = stats.focus.byType[item.key] || 0;
                  const pct = stats.focus.totalSessions > 0 ? Math.round((count / stats.focus.totalSessions) * 100) : 0;
                  return (
                    <div key={item.key}>
                      <div style={{ display: "flex", justifyContent: "space-between", fontSize: "var(--text-xs)", marginBottom: 4 }}>
                        <span style={{ color: "var(--text-secondary)" }}>{item.icon} {item.label}</span>
                        <span style={{ fontWeight: 600, color: "var(--text-primary)" }}>{count} phiên ({pct}%)</span>
                      </div>
                      <div style={{ height: 8, background: "var(--bg-elevated)", borderRadius: 4, overflow: "hidden" }}>
                        <div style={{ height: "100%", width: `${pct}%`, background: item.color, borderRadius: 4 }} />
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
