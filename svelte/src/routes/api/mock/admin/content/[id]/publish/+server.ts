import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const PUT: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Updates content

  return json(null);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Updates content

  return json(null);
};
