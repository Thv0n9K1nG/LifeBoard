import React, { useState, useMemo } from "react";
import { Search as SearchIcon, CheckSquare, Sparkles, FileText, ArrowRight } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { taskApi, Task } from "../tasks/api/taskApi";
import { habitApi, Habit } from "../habits/api/habitApi";
import { noteApi, DailyNote } from "../notes/api/noteApi";
import { Input } from "../../components/ui/Input";

export const SearchPage: React.FC = () => {
  const [query, setQuery] = useState("");
  const navigate = useNavigate();

  const { data: taskData, isLoading: loadingTasks } = useQuery({
    queryKey: ["tasks", "search", query],
    queryFn: () => taskApi.getAll({ search: query.trim() || undefined, pageSize: 50 }),
    enabled: query.trim().length > 0,
  });

  const { data: habitData, isLoading: loadingHabits } = useQuery({
    queryKey: ["habits"],
    queryFn: () => habitApi.getAll(),
  });

  const { data: noteData, isLoading: loadingNotes } = useQuery({
    queryKey: ["daily-notes"],
    queryFn: () => noteApi.getAll(),
  });

  const filteredHabits = useMemo(() => {
    if (!query.trim() || !habitData) return [];
    const q = query.toLowerCase();
    return habitData.filter(
      (h) => h.name.toLowerCase().includes(q) || (h.description && h.description.toLowerCase().includes(q))
    );
  }, [habitData, query]);

  const filteredNotes = useMemo(() => {
    if (!query.trim() || !noteData) return [];
    const q = query.toLowerCase();
    return noteData.filter(
      (n) => n.noteDate.includes(q) || (n.content && n.content.toLowerCase().includes(q))
    );
  }, [noteData, query]);

  const tasks: Task[] = taskData?.data || [];
  const hasSearched = query.trim().length > 0;
  const isLoading = loadingTasks || loadingHabits || loadingNotes;
  const totalResults = tasks.length + filteredHabits.length + filteredNotes.length;

  return (
    <div style={{ maxWidth: 900, margin: "0 auto" }}>
      <div style={{
        padding: "var(--space-6)",
        background: "var(--bg-surface)",
        borderRadius: "var(--radius-lg)",
        border: "1px solid var(--border-subtle)",
        marginBottom: "var(--space-5)",
      }}>
        <h2 style={{ fontSize: "var(--text-xl)", fontWeight: 700, color: "var(--text-primary)", marginBottom: 8 }}>
          Tìm kiếm toàn cục
        </h2>
        <p style={{ color: "var(--text-secondary)", fontSize: "var(--text-sm)", marginBottom: 16 }}>
          Tìm nhanh nhiệm vụ, thói quen và ghi chú nhật ký
        </p>

        <div style={{ position: "relative" }}>
          <Input
            placeholder="Nhập từ khóa tìm kiếm..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            style={{ paddingLeft: 38, fontSize: "var(--text-md)" }}
            autoFocus
          />
          <SearchIcon
            size={18}
            style={{
              position: "absolute",
              left: 12,
              top: "50%",
              transform: "translateY(-50%)",
              color: "var(--text-muted)",
            }}
          />
        </div>
      </div>

      {!hasSearched ? (
        <div style={{
          padding: "var(--space-12)",
          textAlign: "center",
          color: "var(--text-muted)",
          border: "1px dashed var(--border-subtle)",
          borderRadius: "var(--radius-lg)",
          background: "var(--bg-surface)",
        }}>
          <SearchIcon size={36} style={{ margin: "0 auto 12px", opacity: 0.5 }} />
          <p style={{ fontSize: "var(--text-md)", fontWeight: 500, color: "var(--text-secondary)" }}>
            Bắt đầu gõ để tìm kiếm trên toàn hệ thống
          </p>
          <p style={{ fontSize: "var(--text-xs)", marginTop: 4 }}>
            Hỗ trợ tìm kiếm tiêu đề công việc, mô tả thói quen và nội dung ghi chú
          </p>
        </div>
      ) : isLoading ? (
        <div style={{ padding: 40, textAlign: "center", color: "var(--text-muted)" }}>
          Đang tìm kiếm...
        </div>
      ) : totalResults === 0 ? (
        <div style={{
          padding: 40,
          textAlign: "center",
          color: "var(--text-muted)",
          border: "1px dashed var(--border-subtle)",
          borderRadius: "var(--radius-lg)",
        }}>
          Không tìm thấy kết quả nào phù hợp với "{query}"
        </div>
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
          {/* Tasks Results */}
          {tasks.length > 0 && (
            <div style={{
              background: "var(--bg-surface)",
              borderRadius: "var(--radius-lg)",
              border: "1px solid var(--border-subtle)",
              padding: 20,
            }}>
              <h3 style={{
                fontSize: "var(--text-md)",
                fontWeight: 600,
                color: "var(--text-primary)",
                display: "flex",
                alignItems: "center",
                gap: 8,
                marginBottom: 14,
              }}>
                <CheckSquare size={18} color="var(--primary)" />
                Nhiệm vụ ({tasks.length})
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
                {tasks.map((task) => (
                  <div
                    key={task.id}
                    onClick={() => navigate("/tasks")}
                    style={{
                      padding: "10px 14px",
                      borderRadius: "var(--radius-md)",
                      border: "1px solid var(--border-subtle)",
                      background: "var(--bg-elevated)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "space-between",
                      cursor: "pointer",
                      transition: "all 0.15s ease",
                    }}
                  >
                    <div>
                      <div style={{
                        fontSize: "var(--text-sm)",
                        fontWeight: 500,
                        color: task.status === "done" ? "var(--text-muted)" : "var(--text-primary)",
                        textDecoration: task.status === "done" ? "line-through" : "none",
                      }}>
                        {task.title}
                      </div>
                      {task.description && (
                        <div style={{ fontSize: "var(--text-xs)", color: "var(--text-secondary)", marginTop: 2 }}>
                          {task.description}
                        </div>
                      )}
                    </div>
                    <ArrowRight size={14} color="var(--text-muted)" />
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Habits Results */}
          {filteredHabits.length > 0 && (
            <div style={{
              background: "var(--bg-surface)",
              borderRadius: "var(--radius-lg)",
              border: "1px solid var(--border-subtle)",
              padding: 20,
            }}>
              <h3 style={{
                fontSize: "var(--text-md)",
                fontWeight: 600,
                color: "var(--text-primary)",
                display: "flex",
                alignItems: "center",
                gap: 8,
                marginBottom: 14,
              }}>
                <Sparkles size={18} color="#10B981" />
                Thói quen ({filteredHabits.length})
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
                {filteredHabits.map((habit) => (
                  <div
                    key={habit.id}
                    onClick={() => navigate("/habits")}
                    style={{
                      padding: "10px 14px",
                      borderRadius: "var(--radius-md)",
                      border: "1px solid var(--border-subtle)",
                      background: "var(--bg-elevated)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "space-between",
                      cursor: "pointer",
                    }}
                  >
                    <div>
                      <div style={{ fontSize: "var(--text-sm)", fontWeight: 500, color: "var(--text-primary)" }}>
                        {habit.name}
                      </div>
                      <div style={{ fontSize: "var(--text-xs)", color: "var(--text-secondary)", marginTop: 2 }}>
                        Streak: {habit.streak} ngày • Chu kỳ: {habit.frequency}
                      </div>
                    </div>
                    <ArrowRight size={14} color="var(--text-muted)" />
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Notes Results */}
          {filteredNotes.length > 0 && (
            <div style={{
              background: "var(--bg-surface)",
              borderRadius: "var(--radius-lg)",
              border: "1px solid var(--border-subtle)",
              padding: 20,
            }}>
              <h3 style={{
                fontSize: "var(--text-md)",
                fontWeight: 600,
                color: "var(--text-primary)",
                display: "flex",
                alignItems: "center",
                gap: 8,
                marginBottom: 14,
              }}>
                <FileText size={18} color="#F59E0B" />
                Ghi chú nhật ký ({filteredNotes.length})
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
                {filteredNotes.map((note) => (
                  <div
                    key={note.id}
                    onClick={() => navigate(`/calendar/${note.noteDate}`)}
                    style={{
                      padding: "10px 14px",
                      borderRadius: "var(--radius-md)",
                      border: "1px solid var(--border-subtle)",
                      background: "var(--bg-elevated)",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "space-between",
                      cursor: "pointer",
                    }}
                  >
                    <div>
                      <div style={{ fontSize: "var(--text-xs)", fontWeight: 600, color: "var(--primary)" }}>
                        {note.noteDate}
                      </div>
                      <div style={{
                        fontSize: "var(--text-sm)",
                        color: "var(--text-primary)",
                        marginTop: 2,
                        maxHeight: 40,
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                      }}>
                        {note.content || "(Không có nội dung)"}
                      </div>
                    </div>
                    <ArrowRight size={14} color="var(--text-muted)" />
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
