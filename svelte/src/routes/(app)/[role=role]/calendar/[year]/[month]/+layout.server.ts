import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { EventSmall } from "$lib/types/api/calendar";
import { createMonthSpan } from "$lib/utils";
import { error, redirect } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const year = parseInt(params.year);
  const month = parseInt(params.month);

  if (isNaN(year) || year < 2020 || year > 2030) {
    throw error(404, { errorID: nanoid(), message: "Invalid year" });
  }

  if (isNaN(month) || month < 1 || month > 12) {
    throw error(404, { errorID: nanoid(), message: "Invalid month" });
  }

  const span = createMonthSpan(Number(params.year), Number(params.month));
  const response = await fetch(
    routes.calendars.events({ start: span.start, end: span.end }),
  );

  if (!response.ok) {
    const text = await response.text();
    logger.error({ text });
    return redirect(303, `/${params.role}/dashboard`);
  }

  const events: EventSmall[] = await response.json();
  return { events };
}) satisfies LayoutServerLoad;
