import logger from "$lib/logger";
import { createCalendarEvent } from "$lib/server/mock/calendars";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  // Path params: calendar_id
  // Get all events for a calendar
  const events = faker.helpers.multiple(() => createCalendarEvent(), {
    count: { min: 3, max: 10 },
  });
  return json(events);
};

export const POST: RequestHandler = async ({ request }) => {
  const body = await request.json();
  logger.info(
    "POST /api/v1/calendars/calendars/{calendar_id}/events with body:",
    body,
  );
  // Path params: calendar_id
  // Create a new event
  const newEvent = createCalendarEvent();
  return json(newEvent);
};
