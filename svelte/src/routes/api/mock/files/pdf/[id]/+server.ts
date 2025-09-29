import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Generate the PDF for the requested resource
  
  
  return json(null);
};