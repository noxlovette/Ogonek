import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const POST: RequestHandler = async ({}) => {
  // Generates the invite link for the teacher

  return json(null);
};
