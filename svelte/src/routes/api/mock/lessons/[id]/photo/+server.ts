import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/lessons/{id}/photo with body:", body);
  // Path params: id
  // Adds a photo to the lesson
  
  
  return json(null);
};