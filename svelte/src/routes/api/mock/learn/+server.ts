import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // Returns the list of all cards due for review

  return json([null, null]);
};
