import { invokeCommand } from "@/lib/commands";
import { Tag } from '../../tags/api/tagApi';

export interface Task {
  id: number;
  title: string;
  description?: string;
  priority: 'low' | 'medium' | 'high';
  status: 'pending' | 'in_progress' | 'done' | 'done_late' | 'archived';
  deadline?: string;
  isRecurring: boolean;
  recurrenceType?: string;
  recurrenceEndDate?: string;
  plannedDate?: string;
  createdAt: string;
  completedAt?: string;
  updatedAt: string;
  tags: Tag[];
}

export interface TaskListResponse {
  data: Task[];
  total: number;
}

export interface GetTasksParams {
  status?: string;
  priority?: string;
  plannedDate?: string;
  search?: string;
  page?: number;
  pageSize?: number;
}

export interface CreateTaskPayload {
  title: string;
  description?: string;
  priority: 'low' | 'medium' | 'high';
  deadline?: string;
  plannedDate?: string;
  isRecurring: boolean;
  recurrenceType?: string;
  recurrenceEndDate?: string;
  tagIds: number[];
}

export type UpdateTaskPayload = CreateTaskPayload;

export const taskApi = {
  getAll: async (params?: GetTasksParams): Promise<TaskListResponse> => {
    return invokeCommand<TaskListResponse>("get_tasks", { params });
  },
  getById: async (id: number): Promise<Task> => {
    return invokeCommand<Task>("get_task_by_id", { id });
  },
  create: async (data: CreateTaskPayload): Promise<Task> => {
    return invokeCommand<Task>("create_task", { payload: data });
  },
  update: async (id: number, data: UpdateTaskPayload): Promise<Task> => {
    return invokeCommand<Task>("update_task", { id, payload: data });
  },
  complete: async (id: number): Promise<Task> => {
    return invokeCommand<Task>("complete_task", { id });
  },
  uncomplete: async (id: number): Promise<Task> => {
    return invokeCommand<Task>("uncomplete_task", { id });
  },
  archive: async (id: number): Promise<Task> => {
    return invokeCommand<Task>("archive_task", { id });
  },
  delete: async (id: number): Promise<void> => {
    return invokeCommand<void>("delete_task", { id });
  }
};
