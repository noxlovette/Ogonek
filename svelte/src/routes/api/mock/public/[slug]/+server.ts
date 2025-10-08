import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: slug
  // Fetches content by slug (public endpoint)

  return json(null);
};
