import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Delete a calendar

  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/calendars/{id} with body:", body);
  // Path params: id
  // Update a calendar

  return json(null);
};
