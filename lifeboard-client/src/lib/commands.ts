import { invoke } from "@tauri-apps/api/core";

export interface CommandError {
  error: string;
  message: string;
  status?: number;
}

export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (err: unknown) {
    if (typeof err === "object" && err !== null && "error" in err && "message" in err) {
      const e = err as { error: string; message: string };
      throw {
        error: e.error,
        message: e.message,
        status: e.error === "NotFound" ? 404 : 400,
      };
    }
    const message = typeof err === "string" ? err : String(err);
    throw {
      error: "CommandError",
      message,
      status: 500,
    };
  }
}
