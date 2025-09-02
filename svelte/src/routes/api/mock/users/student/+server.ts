import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // list_students

  return json([null, null]);
};
