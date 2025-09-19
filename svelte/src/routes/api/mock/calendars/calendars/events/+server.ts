import { createCalendarEvent } from "$lib/server/mock/calendars";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const events = faker.helpers.multiple(() => createCalendarEvent(), {
    count: { min: 3, max: 10 },
  });
  return json(events);
};

export const POST: RequestHandler = async () => {
  return json({ id: nanoid() });
};
