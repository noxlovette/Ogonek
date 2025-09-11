import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // All content from the website

  return json([null, null]);
};

export const POST: RequestHandler = async ({}) => {
  // Creates new content

  return json(null);
};
