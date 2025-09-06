import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: encoded_key
  // fetch_presigned_url

  return json(null);
};
