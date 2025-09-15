import { createCalendarEvent } from "$lib/server/mock/calendars";
import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type PATCHRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Get a single event by ID
  const event = createCalendarEvent();
  event.uid = params.id;
  return json(event);
};

export const DELETE: RequestHandler = async ({ request, params, url }) => {
  // Path params: id
  // Delete an event
  
  
  return json(null);
};

export const PATCH: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("PATCH /api/v1/calendars/events/{id} with body:", body);
  // Path params: id
  // Update an event
  const updatedEvent = createCalendarEvent();
  updatedEvent.uid = params.id;
  return json(updatedEvent);
};