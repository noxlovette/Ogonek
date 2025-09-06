import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PUTRequestBody = any;

export const PUT: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PUT /api/v1/learn/{id} with body:", body);
  // Path params: id
  // Updates the learn progress on a card

  return json(null);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Resets the progress for a particular deck

  return json([null, null]);
};
