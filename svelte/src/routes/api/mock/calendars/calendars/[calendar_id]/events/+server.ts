import { createCalendarEvent } from "$lib/server/mock/calendars";
import logger from "$lib/logger";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: calendar_id
  // Get all events for a calendar
  const events = faker.helpers.multiple(() => createCalendarEvent(params.calendar_id), { 
    count: { min: 3, max: 10 } 
  });
  return json(events);
};

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/calendars/calendars/{calendar_id}/events with body:", body);
  // Path params: calendar_id
  // Create a new event
  const newEvent = createCalendarEvent(params.calendar_id);
  return json(newEvent);
};