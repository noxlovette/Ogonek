import { routes } from "$lib/routes";
import type { CalendarEvent } from "$lib/types/api/calendar";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const date = new Date(
    Number(params.year),
    Number(params.month) - 1,
    Number(params.day),
  );

  const dayEvents: CalendarEvent[] = await fetch(
    routes.calendars.events_day(date.toISOString()),
  ).then((res) => res.json());
  return {
    dayEvents,
    date,
  };
}) satisfies LayoutServerLoad;
