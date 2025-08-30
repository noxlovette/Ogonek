
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Query params: invite
  // Generates the invite link for the teacher
  
  
  return json(null);
};