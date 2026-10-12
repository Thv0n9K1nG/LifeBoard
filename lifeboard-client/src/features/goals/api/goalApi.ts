import { invokeCommand } from "@/lib/commands";

export interface Goal {
  id: number;
  title: string;
  description?: string;
  currentValue: number;
  targetValue: number;
  unit: string;
  progressPercent: number;
  deadline?: string;
  status: 'active' | 'completed' | 'paused';
  createdAt: string;
  updatedAt: string;
}

export interface CreateGoalPayload {
  title: string;
  description?: string;
  currentValue: number;
  targetValue: number;
  unit: string;
  deadline?: string;
}

export interface UpdateGoalPayload extends CreateGoalPayload {
  status: 'active' | 'completed' | 'paused';
}

function withProgress(g: Goal): Goal {
  const percent = g.targetValue > 0 ? Math.min(100, Math.round((g.currentValue / g.targetValue) * 100)) : 0;
  return { ...g, progressPercent: percent };
}

export const goalApi = {
  getAll: async (status?: string): Promise<Goal[]> => {
    const list = await invokeCommand<Goal[]>("get_goals", { status: status || null });
    return list.map(withProgress);
  },
  getById: async (id: number): Promise<Goal> => {
    const g = await invokeCommand<Goal>("get_goal_by_id", { id });
    return withProgress(g);
  },
  create: async (data: CreateGoalPayload): Promise<Goal> => {
    const g = await invokeCommand<Goal>("create_goal", { payload: data });
    return withProgress(g);
  },
  update: async (id: number, data: UpdateGoalPayload): Promise<Goal> => {
    const g = await invokeCommand<Goal>("update_goal", { id, payload: data });
    return withProgress(g);
  },
  updateProgress: async (id: number, currentValue: number): Promise<Goal> => {
    const g = await invokeCommand<Goal>("update_goal_progress", { id, currentValue });
    return withProgress(g);
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_goal", { id });
  },
};
