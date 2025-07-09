import logger from "$lib/logger";
import type { RequestHandler } from "./$types";

export const PATCH: RequestHandler = async () => {
  logger.info("UPDATED CARD PROGRESS");
  return new Response(null, { status: 204 });
};
