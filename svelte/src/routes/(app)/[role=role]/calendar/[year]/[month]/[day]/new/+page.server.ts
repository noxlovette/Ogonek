import { z } from "$lib";
import { routes } from "$lib/routes";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const { uid } = params;

    const rawData = Object.fromEntries(formData);

    const data = z.updateEventBody.parse(rawData);

    const response = await fetch(routes.calendars.event(uid), {
      body: JSON.stringify(data),
      method: "PATCH",
    }).then((res) => res.json());

    if (!response.ok) {
      return fail(500);
    }
    return redirect(301, ".");
  },
} satisfies Actions;
