import { routes } from "$lib/routes";
import type { CalendarEvent } from "$lib/types/api/calendar";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch }) => {
  const events: CalendarEvent[] = await fetch(routes.calendars.events()).then(
    (res) => res.json(),
  );

  return { events };
}) satisfies LayoutServerLoad;
