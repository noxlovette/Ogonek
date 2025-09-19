import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { CalendarEvent } from "$lib/types/api/calendar";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const response = await fetch(
    routes.calendars.events_by_month(params.year, params.month),
  );

  if (!response.ok) {
    const text = await response.text();
    logger.error({ text });
    return redirect(303, `/${params.role}/dashboard`);
  }

  const events: CalendarEvent[] = await response.json();
  return { events };
}) satisfies LayoutServerLoad;
