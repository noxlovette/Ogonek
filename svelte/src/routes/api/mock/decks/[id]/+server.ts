import logger from "$lib/logger";
import { createDeckWithCards } from "$lib/server/mock/decks";
import { mockResponder } from "$lib/server/mock/helpers";
import type { DeckUpdate } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ url }) => {
  return mockResponder(url, {
    200: () => createDeckWithCards(),
    401: () => null,
    404: () => null,
  });
};

export const DELETE: RequestHandler = async ({ url }) => {
  return mockResponder(url, {
    204: () => 204,
    401: () => null,
    404: () => null,
  });
};

export const PATCH: RequestHandler = async ({ request, url }) => {
  const body: DeckUpdate = await request.json();

  logger.debug(body);

  return mockResponder(url, {
    200: () => json(null, { status: 200 }),
    401: () => null,
    404: () => null,
  });
};
