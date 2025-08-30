
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // Only public decks
  
  
  return json([
  null,
  null
]);
};