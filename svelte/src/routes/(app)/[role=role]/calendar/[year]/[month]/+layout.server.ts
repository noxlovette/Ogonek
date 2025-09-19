import { routes } from "$lib/routes";
import type { CalendarEvent } from "$lib/types/api/calendar";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const events: CalendarEvent[] = await fetch(
    routes.calendars.events_by_month(params.year, params.month),
  ).then((res) => res.json());

  return { events };
}) satisfies LayoutServerLoad;
