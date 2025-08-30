import { createDecksSmall } from "$lib/server/mock/decks";
import type { DeckSmall, PaginatedResponse } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // One deck
  const paginatedResponse: PaginatedResponse<DeckSmall> = {
    data: createDecksSmall(4),
    perPage: 4,
    page: 1,
  };
  return json(paginatedResponse);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Deletes a deck

  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  // Path params: id
  // Updates a deck

  return json(null);
};
