import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({}) => {
  // Fetches self for the user
  
  
  return json(null);
};

export const DELETE: RequestHandler = async ({}) => {
  // Deletes user by their claims
  
  
  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/users with body:", body);
  // Updates the user by their claims
  
  
  return json(null);
};