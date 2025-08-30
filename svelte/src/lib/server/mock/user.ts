import type { Profile, Student, User } from "$lib/types";
import { nanoid } from "nanoid";

export function createMockTeacher(overrides: Partial<User> = {}): User {
  return {
    id: "teacher1",
    name: "Teacher One",
    username: "teacher1",
    role: "teacher",
    email: "",
    ...overrides,
  };
}
export function createMockStudent(overrides: Partial<Student> = {}): Student {
  return {
    id: nanoid(),
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
