import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // fetch_student

  return json(null);
};

export const POST: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // upsert_student

  return json(null);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // remove_student

  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/users/student/{id} with body:", body);
  // Path params: id
  // update_student

  return json(null);
};
