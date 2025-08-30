import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/notifications/register with body:", body);
  // register_device_token
  
  
  return json(null);
};