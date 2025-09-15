import { createEventAttendee } from "$lib/server/mock/calendars";
import logger from "$lib/logger";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const GET: RequestHandler = async ({ request, params, url }) => {
  // Path params: event_id
  // Get all attendees for an event
  const attendees = faker.helpers.multiple(() => createEventAttendee(params.event_id), { 
    count: { min: 1, max: 8 } 
  });
  return json(attendees);
};

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/calendars/events/{event_id}/attendees with body:", body);
  // Path params: event_id
  // Create a new event attendee
  const newAttendee = createEventAttendee(params.event_id);
  return json(newAttendee);
};