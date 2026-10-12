import { invokeCommand } from "@/lib/commands";

export interface StopwatchState {
  id: number;
  state: 'running' | 'paused' | 'stopped';
  label?: string;
  segmentStartedAt?: string;
  accumulatedSeconds: number;
  splits?: string;
  sessionDate?: string;
}

export interface StartStopwatchPayload {
  label?: string;
  sessionDate?: string;
}

export interface StopwatchSplitPayload {
  note?: string;
}

export const stopwatchApi = {
  getCurrent: async (): Promise<StopwatchState | null> => {
    return invokeCommand<StopwatchState | null>("get_stopwatch_state");
  },

  start: async (payload: StartStopwatchPayload): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("start_stopwatch", {
      label: payload.label || null,
      sessionDate: payload.sessionDate || null,
    });
  },

  pause: async (): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("pause_stopwatch");
  },

  resume: async (): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("resume_stopwatch");
  },

  stop: async (): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("stop_stopwatch");
  },

  split: async (payload: StopwatchSplitPayload = {}): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("split_stopwatch", {
      note: payload.note || null,
    });
  },

  updateLabel: async (label: string): Promise<StopwatchState> => {
    return invokeCommand<StopwatchState>("update_stopwatch_label", { label });
  },
};
