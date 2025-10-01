import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { DeleteScope } from "$lib/types/api/calendar";
import { parseFormData } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const { id } = params;
    const rawData = parseFormData(formData);

    const data = z.updateEventBody.parse(rawData);

    if (
      data.dtendTime == null ||
      data.dtstartTime == null ||
      data.dtendTime < data.dtstartTime
    ) {
      return fail(400, { dtend: true });
    }

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

  delete: async ({ params, fetch, request }) => {
    const { id } = params;

    const formData = await request.formData();
    const scope = formData.get("scope") ?? ("this-only" as DeleteScope);

    const response = await fetch(routes.calendars.event(id), {
      method: "DELETE",
      body: JSON.stringify({ scope }),
    });

    if (!response.ok) {
      const text = await response.text();
      console.log(text);
      return fail(500);
    }

    return redirect(301, "..");
  },
} satisfies Actions;
