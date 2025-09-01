import type { Profile, Role, Student, User } from "$lib/types";
import { nanoid } from "nanoid";
/**
 * Creates a user instance for mock endpoints
 * @param overrides redefine the user you want to create
 * @param role t or s, creates either teacher or student. Defaults to teacher
 * @returns a user instance
 */
export function createUser(
  overrides: Partial<User> = {},
  role: Role = "t",
): User {
  const isTeacher = role === "t";
  return {
    id: isTeacher ? "teacher1" : "student",
    name: isTeacher ? "Teacher One" : "Student One",
    username: isTeacher ? "teacher1" : "student1",
    role: isTeacher ? "teacher" : "student",
    email: "",
    ...overrides,
  };
}

/**
 * Creates a Student instance (NOT A STUDENT USER)
 * @param overrides student fields
 * @returns a student instance
 */
export function createStudent(overrides: Partial<Student> = {}): Student {
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

/**
 * Creates a mock profile
 * @param overrides profile override
 * @returns a profile
 */
export function createProfile(overrides: Partial<Profile> = {}): Profile {
  return {
    userId: "teacher1",
    videoCallUrl: "https://zoom.us/j/pwd?=teacher1",
    avatarUrl: null,
    telegramId: null,
    ...overrides,
  };
}

/**
 * Creates a mock student array
 * @param overrides give all of the generated students a specific property
 * @param count how many to create
 * @returns an array of students
 */
export function createStudents(
  overrides: Partial<Student> = {},
  count: number = 4,
): Student[] {
  return Array.from({ length: count }, (_, i) =>
    createStudent({
      name: `Student ${i + 1}`,
      username: `student${i + 1}`,
      ...overrides,
    }),
  );
}

/**
 * In-memory user store for persistence inside a session
 */
export const mockTeacher: User = createUser();
