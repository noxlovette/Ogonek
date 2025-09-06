/**
 * This file was auto-generated from OpenAPI spec.
 * Do not make direct changes to the file.
 */

const API_BASE = "/axum";

export const routes = {
  auth: {
    bind_student_to_teacher: () => `${API_BASE}/auth/bind`,
    generate_invite_link: (
    invite: string
  ) => {
      const params = new URLSearchParams();
      if (invite) params.set("invite", invite);
      const query = params.toString();
      return `${API_BASE}/auth/invite${query ? `?${query}` : ""}`;
    },
    refresh: () => `${API_BASE}/auth/refresh`,
    signin: () => `${API_BASE}/auth/signin`,
    signup: () => `${API_BASE}/auth/signup`,
  },
  decks: {
    all: (
    page?: string,
    per_page?: string,
    search?: string,
    assignee?: string
  ) => {
      const params = new URLSearchParams();
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);
      if (search) params.set("search", search);
      if (assignee) params.set("assignee", assignee);
      const query = params.toString();
      return `${API_BASE}/decks${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/decks`,
    decks_public: () => `${API_BASE}/decks/public`,
    deck: (id: string) => `${API_BASE}/decks/${id}`,
    delete: (id: string) => `${API_BASE}/decks/${id}`,
    update: (id: string) => `${API_BASE}/decks/${id}`,
    duplicate: (id: string) => `${API_BASE}/decks${id}/duplicate`,
  },
  files: {
    abort_multipart_upload: () => `${API_BASE}/files/abort`,
    complete_multipart_upload: () => `${API_BASE}/files/complete`,
    init_multipart_upload: () => `${API_BASE}/files/init`,
    presigned_urls_batch: (file_id: string) => `${API_BASE}/files/presigned/batch/${file_id}`,
    presigned_url: (encoded_key: string) => `${API_BASE}/files/presigned/${encoded_key}`,
    delete_file: (id: string) => `${API_BASE}/files/${id}`,
  },
  learn: {
    due_cards: () => `${API_BASE}/learn`,
    subscribe_to: (id: string) => `${API_BASE}/learn/subscribe/${id}`,
    unsubscribe_from: (id: string) => `${API_BASE}/learn/subscribe/${id}`,
    update_card_progress: (id: string) => `${API_BASE}/learn/${id}`,
    reset_deck_progress: (id: string) => `${API_BASE}/learn/${id}`,
  },
  lessons: {
    all: (
    page?: string,
    per_page?: string,
    search?: string,
    assignee?: string
  ) => {
      const params = new URLSearchParams();
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);
      if (search) params.set("search", search);
      if (assignee) params.set("assignee", assignee);
      const query = params.toString();
      return `${API_BASE}/lessons${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/lessons`,
    lesson: (id: string) => `${API_BASE}/lessons/${id}`,
    delete: (id: string) => `${API_BASE}/lessons/${id}`,
    update: (id: string) => `${API_BASE}/lessons/${id}`,
    upsert_photo: (id: string) => `${API_BASE}/lessons/${id}/photo`,
  },
  notifications: {
    register_device_token: () => `${API_BASE}/notifications/register`,
    request_hw: () => `${API_BASE}/notifications/request`,
  },
  state: {
    badges: () => `${API_BASE}/state/badges`,
    context: () => `${API_BASE}/state/context`,
    dashboard: () => `${API_BASE}/state/dashboard`,
  },
  tasks: {
    all: (
    page?: string,
    per_page?: string,
    search?: string,
    assignee?: string,
    completed?: string,
    priority?: string
  ) => {
      const params = new URLSearchParams();
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);
      if (search) params.set("search", search);
      if (assignee) params.set("assignee", assignee);
      if (completed) params.set("completed", completed);
      if (priority) params.set("priority", priority);
      const query = params.toString();
      return `${API_BASE}/tasks${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/tasks`,
    task: (id: string) => `${API_BASE}/tasks/${id}`,
    toggle: (id: string) => `${API_BASE}/tasks/${id}`,
    delete: (id: string) => `${API_BASE}/tasks/${id}`,
    update: (id: string) => `${API_BASE}/tasks/${id}`,
  },
  users: {
    me: () => `${API_BASE}/users`,
    delete_user: () => `${API_BASE}/users`,
    update_user: () => `${API_BASE}/users`,
    inviter: (
    invite?: string
  ) => {
      const params = new URLSearchParams();
      if (invite) params.set("invite", invite);
      const query = params.toString();
      return `${API_BASE}/users/inviter${query ? `?${query}` : ""}`;
    },
    profile: () => `${API_BASE}/users/profile`,
    upsert_profile: () => `${API_BASE}/users/profile`,
    students: () => `${API_BASE}/users/student`,
    student: (id: string) => `${API_BASE}/users/student/${id}`,
    upsert_student: (id: string) => `${API_BASE}/users/student/${id}`,
    remove_student: (id: string) => `${API_BASE}/users/student/${id}`,
    update_student: (id: string) => `${API_BASE}/users/student/${id}`,
  }
} as const;

// Type helpers
export type Routes = typeof routes;
export type RouteGroup = keyof Routes;

// Helper to get all route functions from a group
export type RouteFunctions<T extends RouteGroup> = Routes[T];
