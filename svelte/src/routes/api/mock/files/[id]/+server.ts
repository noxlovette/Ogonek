import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Deletes file

  return json(null);
};
