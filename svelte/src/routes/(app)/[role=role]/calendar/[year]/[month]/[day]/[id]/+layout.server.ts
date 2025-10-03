import { routes } from "$lib/routes";
import type { EventWithAttendees } from "$lib/types/api/calendar";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const event: EventWithAttendees = await fetch(
    routes.calendars.event({ id: params.id }),
  ).then((res) => res.json());
  return {
    event,
  };
}) satisfies LayoutServerLoad;
