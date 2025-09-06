import type { Card } from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";

/**
 * Creates a single card
 */
export function createCard(): Card {
  return {
    id: nanoid(),
    front: faker.animal.cetacean(),
    back: faker.animal.petName(),
    mediaUrl: null,
  };
}
