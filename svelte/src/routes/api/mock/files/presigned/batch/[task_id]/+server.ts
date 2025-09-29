import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({ request, params, url }) => {
  // Path params: task_id
  // Fetches all the files associated with a task and returns their presigned URLs
  
  
  return json(null);
};