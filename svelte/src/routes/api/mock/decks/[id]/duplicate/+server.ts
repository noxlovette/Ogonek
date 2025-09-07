import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Duplicates a deck, returns new id
  
  
  return json(null);
};