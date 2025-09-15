import { routes } from "$lib/routes";
import type { CalendarEvent } from "$lib/types/api/calendar";
import type { PageServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const dayEvents: CalendarEvent[] = await fetch(
    routes.calendars.events_day(params.iso),
  ).then((res) => res.json());
  return {
    dayEvents,
  };
}) satisfies PageServerLoad;
