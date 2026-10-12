import { invokeCommand } from "@/lib/commands";

export interface Countdown {
  id: number;
  title: string;
  targetDate: string;
  daysRemaining: number;
  icon?: string;
  color?: string;
  createdAt: string;
  updatedAt: string;
}

export type CreateCountdownPayload = Omit<Countdown, 'id' | 'daysRemaining' | 'createdAt' | 'updatedAt'>;
export type UpdateCountdownPayload = CreateCountdownPayload;

function withDaysRemaining(c: Countdown): Countdown {
  if (typeof c.daysRemaining === "number") return c;
  const parts = (c.targetDate || "").split("T")[0].split("-").map(Number);
  if (parts.length === 3) {
    const target = new Date(parts[0], parts[1] - 1, parts[2]);
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const days = Math.round((target.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));
    return { ...c, daysRemaining: days };
  }
  return { ...c, daysRemaining: 0 };
}

export const countdownApi = {
  getAll: async (): Promise<Countdown[]> => {
    const list = await invokeCommand<Countdown[]>("get_countdowns");
    return list.map(withDaysRemaining);
  },
  create: async (data: CreateCountdownPayload): Promise<Countdown> => {
    const c = await invokeCommand<Countdown>("create_countdown", { payload: data });
    return withDaysRemaining(c);
  },
  update: async (id: number, data: UpdateCountdownPayload): Promise<Countdown> => {
    const c = await invokeCommand<Countdown>("update_countdown", { id, payload: data });
    return withDaysRemaining(c);
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_countdown", { id });
  }
};
