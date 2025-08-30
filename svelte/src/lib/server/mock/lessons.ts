import type { LessonFull, LessonSmall } from "$lib/types";
import { daysAgo, yesterday } from "$lib/utils";
import { nanoid } from "nanoid";

export function createMockLessonSmall(
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

export function createMockLessonFull(
  overrides: Partial<LessonFull> = {},
): LessonFull {
  const base = createMockLessonSmall(overrides);

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

export function generateMockLessonsSmall(
  count: number,
  overrides: Partial<LessonSmall> = {},
): LessonSmall[] {
  return Array.from({ length: count }, (_, i) =>
    createMockLessonSmall({
      title: `Lesson ${i + 1}`,
      ...overrides,
    }),
  );
}
