import { invokeCommand } from "@/lib/commands";

export interface DayAnalytics {
  date: string;
  completedTasks: number;
  focusMinutes: number;
  habitsCompleted: number;
  habitsTotal: number;
  moodScore?: number;
}

export interface WeeklyAnalytics {
  weekStart: string;
  weekEnd: string;
  days: DayAnalytics[];
  summary: {
    totalCompletedTasks: number;
    totalFocusMinutes: number;
    completionRate: number;
    avgMoodScore: number;
  };
}

export interface MonthlyAnalytics {
  year: number;
  month: number;
  days: DayAnalytics[];
  summary: {
    totalCompletedTasks: number;
    totalFocusHours: number;
    completionRate: number;
  };
}

export interface HeatmapDay {
  date: string;
  taskCount: number;
  focusMinutes: number;
  habitDone: number;
  intensityLevel: number;
}

export interface YearlyAnalytics {
  year: number;
  heatmap: HeatmapDay[];
}

export interface Statistics {
  period: { from: string; to: string };
  tasks: {
    total: number;
    completed: number;
    completionRate: number;
    byPriority: Record<string, number>;
  };
  focus: {
    totalSessions: number;
    totalMinutes: number;
    avgMinutesPerDay: number;
    byType: Record<string, number>;
  };
  habits: {
    longestStreak?: { habitName: string; streak: number } | null;
    avgCompletionRate: number;
  };
  mood: {
    avgScore: number;
    distribution: Record<string, number>;
  };
}

export const analyticsApi = {
  getWeekly: async (weekStart?: string): Promise<WeeklyAnalytics> => {
    const ws = weekStart || new Date().toISOString().split("T")[0];
    return invokeCommand<WeeklyAnalytics>("get_weekly_analytics", { weekStart: ws });
  },
  getMonthly: async (year: number, month: number): Promise<MonthlyAnalytics> => {
    return invokeCommand<MonthlyAnalytics>("get_monthly_analytics", { year, month });
  },
  getYearly: async (year: number): Promise<YearlyAnalytics> => {
    return invokeCommand<YearlyAnalytics>("get_yearly_analytics", { year });
  },
  getStatistics: async (from: string, to: string): Promise<Statistics> => {
    return invokeCommand<Statistics>("get_statistics", { from, to });
  },
};
