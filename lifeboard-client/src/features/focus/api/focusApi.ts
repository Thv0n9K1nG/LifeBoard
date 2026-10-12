import { invokeCommand } from "@/lib/commands";

export interface FocusSession {
  id: number;
  sessionType: 'stopwatch' | 'pomodoro';
  label?: string;
  startTime: string;
  endTime?: string;
  durationSeconds: number;
  sessionDate: string;
  splits?: string;
  createdAt: string;
  stopwatchState?: string;
  pausedDurationSeconds?: number;
}

export interface FocusSessionResponse {
  data: FocusSession[];
  totalDurationSeconds: number;
}

export type CreateFocusSessionPayload = {
  sessionType: 'stopwatch' | 'pomodoro';
  label?: string;
  startTime: string;
  endTime: string;
  sessionDate?: string;
  splits?: string;
};

export const focusApi = {
  getAll: async (from: string, to: string, type?: string): Promise<FocusSessionResponse> => {
    return invokeCommand<FocusSessionResponse>("get_focus_sessions", {
      from,
      to,
      sessionType: type || null,
    });
  },
  create: async (data: CreateFocusSessionPayload): Promise<FocusSession> => {
    return invokeCommand<FocusSession>("create_focus_session", { payload: data });
  },
  update: async (id: number, data: CreateFocusSessionPayload): Promise<FocusSession> => {
    return invokeCommand<FocusSession>("update_focus_session", { id, payload: data });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_focus_session", { id });
  }
};
