import {
  createCalendarEvent,
  createEventAttendee,
} from "$lib/server/mock/calendars";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  // Path params: id
  // Get a single event by UID

  const event = createCalendarEvent();
  const attendees = faker.helpers.multiple(
    () => createEventAttendee(nanoid()),
    {
      count: { min: 1, max: 8 },
    },
  );

  const complex = {
    ...event,
    attendees,
  };
  return json(complex);
};
