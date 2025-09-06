import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/files/init with body:", body);
  // Multipart upload init endpoint

  return json(null);
};
