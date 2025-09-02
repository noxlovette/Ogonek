import type {
  ActivityLog,
  AppContext,
  DashboardData,
  NotificationBadges,
} from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";
import { createDeckSmall } from "./decks";
import { createLessonSmall } from "./lessons";
import { createTaskSmall } from "./tasks";
import { createProfile, createStudent, createUser } from "./user";

export function createAppContext(
  overrides: Partial<AppContext> = {},
): AppContext {
  return {
    user: createUser(),
    profile: createProfile(),
    callUrl: null,
    students: faker.helpers.multiple(createStudent, { count: 5 }),
    preferences: {
      autoSubscribe: false,
      emailNotifications: false,
      pushNotifications: false,
      theme: "light",
      language: "ru",
      ...overrides,
    },
  };
}

export function createDashboardData(
  overrides: Partial<DashboardData> = {},
): DashboardData {
  return {
    tasks: faker.helpers.multiple(createTaskSmall),
    lessons: faker.helpers.multiple(createLessonSmall),
    decks: faker.helpers.multiple(createDeckSmall),
    activity: faker.helpers.multiple(createActivity),
    learnData: {
      cardsStudiedToday: faker.number.int({ min: 3, max: 100 }),
      currentStreak: faker.number.int({ min: 1, max: 30 }),
    },
    ...overrides,
  };
}

function createActivity(): ActivityLog {
  return {
    action: faker.helpers.arrayElement(["updated", "new", "deleted"]),
    createdAt: faker.date.recent().toISOString(),
    modelId: nanoid(),
    modelType: faker.helpers.arrayElement(["lesson", "deck", "task"]),
  };
}

export function createBadges(): NotificationBadges {
  return {
    unseenDecks: faker.number.int({ min: 0, max: 15 }),
    unseenLessons: faker.number.int({ min: 0, max: 15 }),
    unseenTasks: faker.number.int({ min: 0, max: 15 }),
    dueCards: faker.number.int({ min: 0, max: 15 }),
  };
}
