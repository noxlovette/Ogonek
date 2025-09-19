import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { parseFormData } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const { id } = params;

    const rawData = parseFormData(formData);

    console.log(rawData);

    const data = z.updateEventBody.parse(rawData);

    console.log(data);

    const response = await fetch(routes.calendars.event(id), {
      body: JSON.stringify(data),
      method: "PATCH",
    });

    if (!response.ok) {
      const data = await response.text();
      logger.error({ data });
      return fail(500);
    }
    return redirect(301, ".");
  },

  delete: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.calendars.event(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return redirect(301, "..");
  },
} satisfies Actions;
