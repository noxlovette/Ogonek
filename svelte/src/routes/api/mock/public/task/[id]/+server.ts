
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Public Task. Handled in the content router
  
  
  return json([
  null,
  null
]);
};