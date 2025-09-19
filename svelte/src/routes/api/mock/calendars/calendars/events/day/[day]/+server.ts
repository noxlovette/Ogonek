import { createCalendarEvent } from "$lib/server/mock/calendars";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  // Path params: day
  // Get all events for a given day
  const events = faker.helpers.multiple(() => createCalendarEvent(), {
    count: { min: 3, max: 10 },
  });
  return json(events);
};
