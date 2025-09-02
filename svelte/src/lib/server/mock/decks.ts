import type { DeckFull, DeckSmall, DeckWithCards } from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";
import { createCards } from "./cards";

export function createDeckSmall(): DeckSmall {
  return {
    id: nanoid(),
    title: faker.book.title(),
    assigneeName: faker.person.fullName(),
    cardCount: faker.number.int(),
    description: Array.from(
      { length: faker.number.int({ min: 1, max: 4 }) },
      () => faker.word.adjective(),
    ).join(";"),
    isSubscribed: faker.datatype.boolean(),
    visibility: faker.helpers.arrayElement(["assigned", "private", "public"]),
    seen: faker.datatype.boolean(),
  };
}

export function createDeckFull(): DeckFull {
  const base = createDeckSmall();

  return {
    ...base,
    createdBy: nanoid(),
    createdAt: faker.date.past().toISOString(),
    assignee: nanoid(),
  };
}

export function createDeckWithCards(): DeckWithCards {
  return {
    deck: createDeckFull(),
    cards: createCards(),
  };
}
