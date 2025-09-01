import { createDeckFull } from "$lib/server/mock/decks";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Query params: page, per_page, search, assignee
  // Decks the user has access to

  return json(createDeckFull());
};

export const POST: RequestHandler = async ({}) => {
  // Creates a new Deck using user defaults
  const id = nanoid();
  return json(id);
};
