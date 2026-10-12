import { invokeCommand } from "@/lib/commands";

export interface Habit {
  id: number;
  name: string;
  description?: string;
  frequency: 'daily' | 'weekly' | 'monthly';
  icon: string;
  color: string;
  isActive: boolean;
  streak: number;
  checkedInToday: boolean;
  createdAt: string;
}

export type CreateHabitPayload = Omit<Habit, 'id' | 'isActive' | 'streak' | 'checkedInToday' | 'createdAt'>;
export type UpdateHabitPayload = CreateHabitPayload;

export const habitApi = {
  getAll: async (): Promise<Habit[]> => {
    return invokeCommand<Habit[]>("get_habits");
  },
  create: async (data: CreateHabitPayload): Promise<Habit> => {
    return invokeCommand<Habit>("create_habit", { payload: data });
  },
  update: async (id: number, data: UpdateHabitPayload): Promise<Habit> => {
    return invokeCommand<Habit>("update_habit", { id, payload: data });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_habit", { id });
  },
  checkin: async (id: number, date: string, isCompleted: boolean): Promise<void> => {
    return invokeCommand<void>("checkin_habit", { id, date, isCompleted });
  }
};
