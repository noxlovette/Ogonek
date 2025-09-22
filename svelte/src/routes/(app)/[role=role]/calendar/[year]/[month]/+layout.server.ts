import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { EventSmall } from "$lib/types/api/calendar";
import { createMonthSpan } from "$lib/utils";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const span = createMonthSpan(Number(params.year), Number(params.month));
  const response = await fetch(routes.calendars.events(span.start, span.end));

  if (!response.ok) {
    const text = await response.text();
    logger.error({ text });
    return redirect(303, `/${params.role}/dashboard`);
  }

  const events: EventSmall[] = await response.json();
  return { events };
}) satisfies LayoutServerLoad;
