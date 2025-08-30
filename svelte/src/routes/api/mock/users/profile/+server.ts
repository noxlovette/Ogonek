import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({}) => {
  // Fetch the profile
  
  
  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/users/profile with body:", body);
  // Update the profile, or create a new one if already there
  
  
  return json(null);
};