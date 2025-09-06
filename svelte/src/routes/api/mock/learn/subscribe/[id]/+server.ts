import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Subscribes the user to the deck

  return json(null);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Unsubscribes the user from the deck

  return json(null);
};
