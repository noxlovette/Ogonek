import { createCard } from "$lib/server/mock/cards";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  // Returns the list of all cards due for review

  return json(faker.helpers.multiple(createCard, { count: 50 }));
};
