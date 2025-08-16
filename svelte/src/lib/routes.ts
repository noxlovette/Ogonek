const API_BASE = "/axum";

export const routes = {
  auth: {
    signup: () => `${API_BASE}/auth/signup`,
    login: () => `${API_BASE}/auth/signin`,
    refresh: () => `${API_BASE}/auth/refresh`,
    invite: (isRegistered: boolean = false) =>
      `${API_BASE}/auth/invite?isRegistered=${isRegistered}`,
    bind: () => `${API_BASE}/auth/bind`,
  },

  learn: {
    due: () => `${API_BASE}/learn`,
    update: (id: string) => `${API_BASE}/learn/${id}`,
    subscribe: (id: string) => `${API_BASE}/learn/subscribe/${id}`,
  },

  decks: {
    all: (
      search?: string,
      assignee?: string,
      page?: string,
      per_page?: string,
    ) => {
      const params = new URLSearchParams();
      if (search) params.append("search", search);
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);

      if (assignee) params.append("assignee", assignee);
      const query = params.toString();

      return `${API_BASE}/decks${query ? `?${query}` : ""}`;
    },
    single: (id: string) => `${API_BASE}/decks/${id}`,
    duplicate: (id: string) => `${API_BASE}/decks/${id}/duplicate`,
    new: () => `${API_BASE}/decks`,
  },

  lessons: {
    single: (id: string) => `${API_BASE}/lessons/${id}`,
    new: () => `${API_BASE}/lessons`,
    all: (
      page?: string,
      per_page?: string,
      search?: string,
      assignee?: string,
    ) => {
      const params = new URLSearchParams();
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);
      if (search) params.set("search", search);
      if (assignee) params.set("assignee", assignee);
      const query = params.toString();
      return `${API_BASE}/lessons${query ? `?${query}` : ""}`;
    },
  },

  tasks: {
    all: (
      page?: string,
      per_page?: string,
      search?: string,
      assignee?: string,
      completed?: string,
    ) => {
      const params = new URLSearchParams();
      if (page) params.set("page", page);
      if (per_page) params.set("per_page", per_page);
      if (search) params.set("search", search);
      if (assignee) params.set("assignee", assignee);
      if (completed) params.set("completed", completed);
      const query = params.toString();
      return `${API_BASE}/tasks${query ? `?${query}` : ""}`;
    },
    single: (id: string) => `${API_BASE}/tasks/${id}`,
    new: () => `${API_BASE}/tasks`,
    request: () => `${API_BASE}/notification/request`,
  },
  s3: {
    check: () => `${API_BASE}/s3`,
    presign: (encodedKey: string) => `${API_BASE}/s3/presigned/${encodedKey}`,
    single: (id: string) => `${API_BASE}/s3/${id}`,
    multipart: {
      init: () => `${API_BASE}/s3/init`,
      complete: () => `${API_BASE}/s3/complete`,
      abort: () => `${API_BASE}/s3/abort`,
    },
  },

  students: {
    all: () => `${API_BASE}/users/student`,
    single: (id: string) => `${API_BASE}/users/student/${id}`,
  },

  users: {
    self: () => `${API_BASE}/users`,
    profile: () => `${API_BASE}/users/profile`,
  },

  state: {
    dashboard: () => `${API_BASE}/state/dashboard`,
    badges: () => `${API_BASE}/state/badges`,
    context: () => `${API_BASE}/state/context`,
  },
  health: () => `${API_BASE}/health`,
} as const;
