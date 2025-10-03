import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type DELETERequestBody = string[];

export const DELETE: RequestHandler = async ({}) => {
  // Deletes many lessons

  return json(null);
};
