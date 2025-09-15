import { routes } from "$lib/routes";
import type { EventWithAttendees } from "$lib/types/api/calendar";
import type { PageServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const event: EventWithAttendees = await fetch(
    routes.calendars.event(params.uid),
  ).then((res) => res.json());
  return {
    event,
  };
}) satisfies PageServerLoad;
