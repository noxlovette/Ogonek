import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { parseFormData } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";
export const actions = {
  default: async ({ request, fetch }) => {
    const formData = await request.formData();

    const rawData = parseFormData(formData);

    const data = z.createEventBody.safeParse(rawData);

    if (!data.success) {
      const fieldErrors: Record<string, boolean> = {};
      data.error.issues.forEach((issue) => {
        const fieldPath = issue.path[0];
        if (typeof fieldPath === "string") {
          fieldErrors[fieldPath] = true;
        }
      });
      logger.error({ fieldErrors });
      return fail(400, { ...fieldErrors });
    }

    const response = await fetch(routes.calendars.new_event(), {
      body: JSON.stringify(data.data),
      method: "POST",
    });

    if (!response.ok) {
      const errorData = await response.text();

      logger.error({ errorData });
      return fail(500, { attendee: false, dtend: false, dtstart: false });
    }

    return redirect(301, ".");
  },
} satisfies Actions;
