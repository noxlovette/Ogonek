import type { LessonFull, LessonSmall } from "$lib/types";
import { daysAgo, yesterday } from "$lib/utils";
import { nanoid } from "nanoid";

export function createLessonSmall(
  overrides: Partial<LessonSmall> = {},
): LessonSmall {
  return {
    id: nanoid(),
    title: "Sample Lesson",
    topic: "Sample Topic",
    createdAt: yesterday(),
    assigneeName: "John Doe",
    seen: false,
    ...overrides,
  };
}

export function createLessonsSmall(
  count: number = 4,
  overrides: Partial<LessonSmall> = {},
): LessonSmall[] {
  return Array.from({ length: count }, (_, i) =>
    createLessonSmall({
      title: `Lesson ${i + 1}`,
      ...overrides,
    }),
  );
}
export function createLessonFull(
  overrides: Partial<LessonFull> = {},
): LessonFull {
  const base = createLessonSmall(overrides);

  return {
    ...base,
    markdown:
      "## Task Description\n\nThis is a sample task with markdown content.",
    createdBy: "teacher1",
    updatedAt: daysAgo(1),
    assignee: "student_" + nanoid(8),
    ...overrides,
  };
}
