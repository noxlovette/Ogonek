import type {
  ActivityLog,
  AppContext,
  DashboardData,
  NotificationBadges,
} from "$lib/types";
import { nanoid } from "nanoid";
import { createDecksSmall } from "./decks";
import { createLessonsSmall } from "./lessons";
import { createTasksSmall } from "./tasks";
import { createMockProfile, createMockTeacher, createStudents } from "./user";

export function createAppContext(
  overrides: Partial<AppContext> = {},
): AppContext {
  return {
    user: createMockTeacher(),
    profile: createMockProfile(),
    callUrl: null,
    students: createStudents(),
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
    tasks: createTasksSmall(),
    lessons: createLessonsSmall(),
    decks: createDecksSmall(),
    activity: createActivities(),
    learnData: {
      cardsStudiedToday: 5,
      currentStreak: 2,
    },
  };
}

function createActivity(overrides: Partial<ActivityLog> = {}): ActivityLog {
  return {
    action: "created",
    createdAt: null,
    modelId: nanoid(),
    modelType: "lesson",
    ...overrides,
  };
}

function createActivities(count = 4): ActivityLog[] {
  return Array.from({ length: count }, (_, i) => createActivity());
}

export function createBadges(): NotificationBadges {
  return {
    unseenDecks: 2,
    unseenLessons: 4,
    unseenTasks: 4,
    dueCards: 4,
  };
}
