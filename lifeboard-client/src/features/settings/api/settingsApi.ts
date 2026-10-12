import { invokeCommand } from "@/lib/commands";

export interface Settings {
  theme: 'light' | 'dark';
  pomodoroFocusMinutes: number;
  pomodoroBreakMinutes: number;
  pomodoroRounds: number;
  habitGracePeriodDays: number;
  language: string;
  timezone: string;
}

export const settingsApi = {
  get: async (): Promise<Settings> => {
    return invokeCommand<Settings>("get_settings");
  },
  update: async (data: Settings): Promise<Settings> => {
    return invokeCommand<Settings>("update_settings", { payload: data });
  },
  exportBackup: async (): Promise<string> => {
    return invokeCommand<string>("export_backup_json");
  },
  importBackup: async (jsonContent: string): Promise<void> => {
    return invokeCommand<void>("import_backup_json", { jsonContent });
  }
};
