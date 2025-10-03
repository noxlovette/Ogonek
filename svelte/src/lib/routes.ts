/**
 * This file was auto-generated from OpenAPI spec.
 * Do not make direct changes to the file.
 */

const API_BASE = "/axum";

export const routes = {
  admin: {
    content: () => `${API_BASE}/admin/content`,
    new_content: () => `${API_BASE}/admin/content`,
    delete_content: (params: { id: string }) => `${API_BASE}/admin/content/${params.id}`,
    update_content: (params: { id: string }) => `${API_BASE}/admin/content/${params.id}`,
    publish_content: (params: { id: string }) => `${API_BASE}/admin/content/${params.id}/publish`,
    unpublish_content: (params: { id: string }) => `${API_BASE}/admin/content/${params.id}/publish`,
  },
  auth: {
    bind_student_to_teacher: () => `${API_BASE}/auth/bind`,
    generate_invite_link: (params: { isRegistered: string }) => {
      const urlParams = new URLSearchParams();
      if (params.isRegistered) urlParams.set("isRegistered", params.isRegistered);
      const query = urlParams.toString();
      return `${API_BASE}/auth/invite${query ? `?${query}` : ""}`;
    },
    refresh: () => `${API_BASE}/auth/refresh`,
    signin: () => `${API_BASE}/auth/signin`,
    signup: () => `${API_BASE}/auth/signup`,
  },
  calendars: {
    calendar: () => `${API_BASE}/calendars`,
    delete_attendee: (params: { id: string }) => `${API_BASE}/calendars/attendees/${params.id}`,
    update_attendee: (params: { id: string }) => `${API_BASE}/calendars/attendees/${params.id}`,
    events: (params: { start: string; end: string; role?: string }) => {
      const urlParams = new URLSearchParams();
      if (params.start) urlParams.set("start", params.start);
      if (params.end) urlParams.set("end", params.end);
      if (params.role) urlParams.set("role", params.role);
      const query = urlParams.toString();
      return `${API_BASE}/calendars/events${query ? `?${query}` : ""}`;
    },
    new_event: () => `${API_BASE}/calendars/events`,
    event: (params: { id: string }) => `${API_BASE}/calendars/events/${params.id}`,
    delete_event: (params: { id: string }) => `${API_BASE}/calendars/events/${params.id}`,
    update_event: (params: { id: string }) => `${API_BASE}/calendars/events/${params.id}`,
    delete_calendar: (params: { id: string }) => `${API_BASE}/calendars/${params.id}`,
    update_calendar: (params: { id: string }) => `${API_BASE}/calendars/${params.id}`,
  },
  content: {
    content_public: (params: { slug: string }) => `${API_BASE}/content/${params.slug}`,
  },
  decks: {
    all: (params: { page?: string; per_page?: string; search?: string; assignee?: string; visibility?: string; sort_by?: string; sort_order?: string }) => {
      const urlParams = new URLSearchParams();
      if (params.page) urlParams.set("page", params.page);
      if (params.per_page) urlParams.set("per_page", params.per_page);
      if (params.search) urlParams.set("search", params.search);
      if (params.assignee) urlParams.set("assignee", params.assignee);
      if (params.visibility) urlParams.set("visibility", params.visibility);
      if (params.sort_by) urlParams.set("sort_by", params.sort_by);
      if (params.sort_order) urlParams.set("sort_order", params.sort_order);
      const query = urlParams.toString();
      return `${API_BASE}/decks${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/decks`,
    decks_public: () => `${API_BASE}/decks/public`,
    deck: (params: { id: string }) => `${API_BASE}/decks/${params.id}`,
    delete: (params: { id: string }) => `${API_BASE}/decks/${params.id}`,
    update: (params: { id: string }) => `${API_BASE}/decks/${params.id}`,
    duplicate: (params: { id: string }) => `${API_BASE}/decks/${params.id}/duplicate`,
  },
  files: {
    abort_multipart_upload: () => `${API_BASE}/files/abort`,
    complete_multipart_upload: () => `${API_BASE}/files/complete`,
    init_multipart_upload: () => `${API_BASE}/files/init`,
    pdf: (params: { id: string; pdfType: string }) => {
      const urlParams = new URLSearchParams();
      if (params.pdfType) urlParams.set("pdfType", params.pdfType);
      const query = urlParams.toString();
      return `${API_BASE}/files/pdf/${params.id}${query ? `?${query}` : ""}`;
    },
    presigned_urls_batch: (params: { task_id: string }) => `${API_BASE}/files/presigned/batch/${params.task_id}`,
    presigned_url: (params: { encoded_key: string }) => `${API_BASE}/files/presigned/${params.encoded_key}`,
    delete_file: (params: { id: string }) => `${API_BASE}/files/${params.id}`,
  },
  learn: {
    due_cards: () => `${API_BASE}/learn`,
    subscribe_to: (params: { id: string }) => `${API_BASE}/learn/subscribe/${params.id}`,
    unsubscribe_from: (params: { id: string }) => `${API_BASE}/learn/subscribe/${params.id}`,
    update_card_progress: (params: { id: string }) => `${API_BASE}/learn/${params.id}`,
    reset_deck_progress: (params: { id: string }) => `${API_BASE}/learn/${params.id}`,
  },
  lessons: {
    all: (params: { page?: string; per_page?: string; search?: string; assignee?: string; topic?: string; sort_by?: string; sort_order?: string }) => {
      const urlParams = new URLSearchParams();
      if (params.page) urlParams.set("page", params.page);
      if (params.per_page) urlParams.set("per_page", params.per_page);
      if (params.search) urlParams.set("search", params.search);
      if (params.assignee) urlParams.set("assignee", params.assignee);
      if (params.topic) urlParams.set("topic", params.topic);
      if (params.sort_by) urlParams.set("sort_by", params.sort_by);
      if (params.sort_order) urlParams.set("sort_order", params.sort_order);
      const query = urlParams.toString();
      return `${API_BASE}/lessons${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/lessons`,
    delete_lesson_many: () => `${API_BASE}/lessons/many`,
    lesson: (params: { id: string }) => `${API_BASE}/lessons/${params.id}`,
    delete: (params: { id: string }) => `${API_BASE}/lessons/${params.id}`,
    update: (params: { id: string }) => `${API_BASE}/lessons/${params.id}`,
    upsert_photo: (params: { id: string }) => `${API_BASE}/lessons/${params.id}/photo`,
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
    all: (params: { page?: string; per_page?: string; search?: string; assignee?: string; completed?: string; sort_by?: string; sort_order?: string }) => {
      const urlParams = new URLSearchParams();
      if (params.page) urlParams.set("page", params.page);
      if (params.per_page) urlParams.set("per_page", params.per_page);
      if (params.search) urlParams.set("search", params.search);
      if (params.assignee) urlParams.set("assignee", params.assignee);
      if (params.completed) urlParams.set("completed", params.completed);
      if (params.sort_by) urlParams.set("sort_by", params.sort_by);
      if (params.sort_order) urlParams.set("sort_order", params.sort_order);
      const query = urlParams.toString();
      return `${API_BASE}/tasks${query ? `?${query}` : ""}`;
    },
    new: () => `${API_BASE}/tasks`,
    task: (params: { id: string }) => `${API_BASE}/tasks/${params.id}`,
    toggle: (params: { id: string }) => `${API_BASE}/tasks/${params.id}`,
    delete: (params: { id: string }) => `${API_BASE}/tasks/${params.id}`,
    update: (params: { id: string }) => `${API_BASE}/tasks/${params.id}`,
  },
  users: {
    me: () => `${API_BASE}/users`,
    delete_user: () => `${API_BASE}/users`,
    update_user: () => `${API_BASE}/users`,
    inviter: (params: { invite?: string }) => {
      const urlParams = new URLSearchParams();
      if (params.invite) urlParams.set("invite", params.invite);
      const query = urlParams.toString();
      return `${API_BASE}/users/inviter${query ? `?${query}` : ""}`;
    },
    profile: () => `${API_BASE}/users/profile`,
    upsert_profile: () => `${API_BASE}/users/profile`,
    students: () => `${API_BASE}/users/student`,
    student: (params: { id: string }) => `${API_BASE}/users/student/${params.id}`,
    upsert_student: (params: { id: string }) => `${API_BASE}/users/student/${params.id}`,
    remove_student: (params: { id: string }) => `${API_BASE}/users/student/${params.id}`,
    update_student: (params: { id: string }) => `${API_BASE}/users/student/${params.id}`,
  }
} as const;

// Type helpers
export type Routes = typeof routes;
export type RouteGroup = keyof Routes;

// Helper to get all route functions from a group
export type RouteFunctions<T extends RouteGroup> = Routes[T];
