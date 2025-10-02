import { routes } from "$lib/routes";
import type { EventSmall } from "$lib/types/api/calendar";
import { createDaySpan } from "$lib/utils";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const [year, month, day] = [params.year, params.month, params.day].map(
    (item) => Number(item),
  );

  if (isNaN(day) || day < 1 || year > 31) {
    throw error(404);
  }
  const { start, end } = createDaySpan(year, month, day);
  const date = new Date(year, month - 1, day);
  const dayEvents: EventSmall[] = await fetch(
    routes.calendars.events(start, end),
  ).then((res) => res.json());
  return {
    dayEvents,
    date,
  };
}) satisfies LayoutServerLoad;
