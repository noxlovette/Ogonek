
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: year, month
  // Get events for a specific month
  
  
  return json([
  null,
  null
]);
};