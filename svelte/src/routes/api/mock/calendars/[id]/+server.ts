import { createCalendar } from "$lib/server/mock/calendars";
import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Get a single calendar by ID
  const calendar = createCalendar();
  calendar.id = params.id;
  return json(calendar);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Delete a calendar
  
  
  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/calendars/{id} with body:", body);
  // Path params: id
  // Update a calendar
  const updatedCalendar = createCalendar();
  updatedCalendar.id = params.id;
  return json(updatedCalendar);
};