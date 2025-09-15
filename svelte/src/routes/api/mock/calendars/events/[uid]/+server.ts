import { createCalendarEvent } from "$lib/server/mock/calendars";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  // Path params: id
  // Get a single event by UID

  const event = createCalendarEvent();

  return json(event);
};
