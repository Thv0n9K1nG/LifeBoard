import { invokeCommand } from "@/lib/commands";

export interface DailyNote {
  id: number;
  noteDate: string;
  content?: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpsertDailyNotePayload {
  noteDate: string;
  content: string;
}

export const noteApi = {
  getByDate: async (date: string): Promise<DailyNote | null> => {
    return invokeCommand<DailyNote | null>("get_daily_note_by_date", { date });
  },
  getAll: async (): Promise<DailyNote[]> => {
    return invokeCommand<DailyNote[]>("get_daily_notes");
  },
  upsert: async (data: UpsertDailyNotePayload): Promise<DailyNote> => {
    return invokeCommand<DailyNote>("upsert_daily_note", { payload: data });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_daily_note", { id });
  }
};
