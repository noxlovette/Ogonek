import { routes } from "$lib/routes";
import type { Calendar, CalendarEvent } from "$lib/types/api/calendar";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch }) => {
  const calendars: Calendar[] = await fetch(routes.calendars.calendars()).then(
    (res) => res.json(),
  );

  const events: CalendarEvent[] = await fetch(
    routes.calendars.events(calendars[0].id),
  ).then((res) => res.json());

  return { calendars, events };
}) satisfies LayoutServerLoad;
