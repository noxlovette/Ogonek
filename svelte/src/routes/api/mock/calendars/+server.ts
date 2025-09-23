import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // Get the user's calendar

  return json(null);
};
