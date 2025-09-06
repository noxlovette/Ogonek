import type { LessonFull, LessonSmall } from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";

export function createLessonSmall(): LessonSmall {
  return {
    id: nanoid(),
    title: faker.book.title(),
    topic: faker.book.genre(),
    createdAt: faker.date.recent().toISOString(),
    assigneeName: faker.person.fullName(),
    seen: faker.datatype.boolean(),
  };
}

export function createLessonFull(): LessonFull {
  const base = createLessonSmall();

  return {
    ...base,
    markdown: faker.lorem.text(),
    updatedAt: faker.date.recent().toISOString(),
    assignee: nanoid(),
  };
}
