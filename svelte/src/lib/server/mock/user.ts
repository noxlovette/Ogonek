import type { Profile, Role, Student, User } from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";
/**
 * Creates a user instance for mock endpoints
 * @param overrides redefine the user you want to create
 * @param role t or s, creates either teacher or student. Defaults to teacher
 * @returns a user instance
 */
export function createUser(role: Role = "t"): User {
  const isTeacher = role === "t";
  return {
    id: nanoid(),
    name: faker.person.fullName(),
    username: faker.internet.username(),
    role: isTeacher ? "teacher" : "student",
    email: faker.internet.email(),
  };
}

/**
 * Creates a Student instance (NOT A STUDENT USER)
 * @param overrides student fields
 * @returns a student instance
 */
export function createStudent(): Student {
  return {
    id: nanoid(),
    name: faker.person.firstName(),
    username: faker.internet.username(),
    email: faker.internet.email(),
    markdown: null,
    studentTelegramId: null,
  };
}

/**
 * Creates a mock profile
 * @param overrides profile override
 * @returns a profile
 */
export function createProfile(overrides: Partial<Profile> = {}): Profile {
  return {
    userId: nanoid(),
    videoCallUrl: "https://zoom.us/j/pwd?=teacher1",
    avatarUrl: null,
    telegramId: null,
    ...overrides,
  };
}
