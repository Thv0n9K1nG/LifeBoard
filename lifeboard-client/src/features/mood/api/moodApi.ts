import { invokeCommand } from "@/lib/commands";

export interface MoodEntry {
  id: number;
  entryDate: string;
  score: number;
  note?: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpsertMoodPayload {
  entryDate: string;
  score: number;
  note?: string;
}

export const moodApi = {
  getByDate: async (date: string): Promise<MoodEntry | null> => {
    return invokeCommand<MoodEntry | null>("get_mood_entry_by_date", { date });
  },
  getAll: async (): Promise<MoodEntry[]> => {
    return invokeCommand<MoodEntry[]>("get_mood_entries");
  },
  upsert: async (data: UpsertMoodPayload): Promise<MoodEntry> => {
    return invokeCommand<MoodEntry>("upsert_mood_entry", { payload: data });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_mood_entry", { id });
  }
};
