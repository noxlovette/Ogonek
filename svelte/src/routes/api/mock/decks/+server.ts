import { createDeckSmall } from "$lib/server/mock/decks";
import type { DeckSmall, PaginatedResponse } from "$lib/types";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<DeckSmall> = {
    data: faker.helpers.multiple(createDeckSmall),
    perPage: 3,
    page: 1,
  };
  return json(paginatedResponse);
};

export const POST: RequestHandler = async () => {
  const id = nanoid();
  return json(id);
};
