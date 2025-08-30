import { createAppContext } from "$lib/server/mock/state";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({}) => {
  // fetch_context

  return json(createAppContext());
};
