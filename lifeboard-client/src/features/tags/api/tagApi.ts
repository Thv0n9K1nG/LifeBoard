import { invokeCommand } from "@/lib/commands";

export interface Tag {
  id: number;
  name: string;
  color: string;
}

export type CreateTagPayload = Omit<Tag, 'id'>;
export type UpdateTagPayload = Omit<Tag, 'id'>;

export const tagApi = {
  getAll: async (): Promise<Tag[]> => {
    return invokeCommand<Tag[]>("get_tags");
  },
  create: async (data: CreateTagPayload): Promise<Tag> => {
    return invokeCommand<Tag>("create_tag", { payload: data });
  },
  update: async (id: number, data: UpdateTagPayload): Promise<Tag> => {
    return invokeCommand<Tag>("update_tag", { id, payload: data });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_tag", { id });
  }
};
