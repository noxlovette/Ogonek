import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/auth/refresh with body:", body);
  // Receives the refresh token as json, gets it, then decodes, finds the user id, and generates a new access token

  return json(null);
};
