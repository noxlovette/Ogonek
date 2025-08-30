import type { AppContext, Profile, Student, User } from "$lib/types";

export function createMockTeacher(overrides: Partial<User> = {}): User {
  return {
    id: "teacher1",
    name: "Teacher One",
    username: "teacher1",
    role: "teacher",
    email: null,
    ...overrides,
  };
}
export function createMockStudent(overrides: Partial<Student> = {}): Student {
  return {
    id: "student1",
    name: "Student One",
    username: "student1",
    email: "",
    markdown: null,
    studentTelegramId: null,
    ...overrides,
  };
}
export function createMockProfile(overrides: Partial<Profile> = {}): Profile {
  return {
    userId: "teacher1",
    videoCallUrl: "https://zoom.us/j/pwd?=teacher1",
    avatarUrl: null,
    telegramId: null,
    ...overrides,
  };
}

export function createStudents(
  overrides: Partial<Student> = {},
  count: number = 4,
): Student[] {
  return Array.from({ length: count }, (_, i) =>
    createMockStudent({
      name: `Student ${i + 1}`,
      username: `student${i + 1}`,
      ...overrides,
    }),
  );
}

export function generateAppContext(
  overrides: Partial<AppContext> = {},
): AppContext {
  return {
    user: createMockTeacher(),
    profile: createMockProfile(),
    callUrl: null,
    students: createStudents(),
    preferences: {
      auto_subscribe: false,
      email_notifications: false,
      push_notifications: false,
      theme: "light",
      language: "ru",
      ...overrides,
    },
  };
}
