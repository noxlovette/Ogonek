import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({ request, params, url }) => {
  // Query params: token
  // Confirms email verification using a token

  return json(null);
};
