import logger from "$lib/logger";
import {
  createCalendarEvent,
  createEventAttendee,
} from "$lib/server/mock/calendars";
import type { EventWithAttendees } from "$lib/types/api/calendar";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const event: EventWithAttendees = {
    ...createCalendarEvent(),
    attendees: [createEventAttendee()],
  };
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

  return json(null);
};
