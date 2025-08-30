import type { DeckFull, DeckSmall } from "$lib/types";
import { daysAgo } from "$lib/utils";
import { nanoid } from "nanoid";

export function createDeckSmall(overrides: Partial<DeckSmall> = {}): DeckSmall {
  return {
    id: nanoid(),
    title: "Sample Lesson",
    assigneeName: "John Doe",
    cardCount: 10,
    description: "test;miracle",
    isSubscribed: true,
    visibility: "assigned",
    seen: false,
    ...overrides,
  };
}

export function createDeckFull(overrides: Partial<DeckFull> = {}): DeckFull {
  const base = createDeckSmall(overrides);

  return {
    ...base,
    createdBy: "teacher1",
    createdAt: daysAgo(30),
    assignee: "student_" + nanoid(8),
    ...overrides,
  };
}

export function createDecksSmall(
  count: number = 4,
  overrides: Partial<DeckSmall> = {},
): DeckSmall[] {
  return Array.from({ length: count }, (_, i) =>
    createDeckSmall({
      title: `Deck ${i + 1}`,
      ...overrides,
    }),
  );
}
