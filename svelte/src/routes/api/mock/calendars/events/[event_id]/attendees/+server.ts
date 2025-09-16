import logger from "$lib/logger";
import { createEventAttendee } from "$lib/server/mock/calendars";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: event_id
  // Get all attendees for an event
  const attendees = faker.helpers.multiple(() => createEventAttendee(), {
    count: { min: 1, max: 8 },
  });
  return json(attendees);
};

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info(
    "POST /api/v1/calendars/events/{event_id}/attendees with body:",
    body,
  );
  // Path params: event_id
  // Create a new event attendee
  const newAttendee = createEventAttendee();
  return json(newAttendee);
};
