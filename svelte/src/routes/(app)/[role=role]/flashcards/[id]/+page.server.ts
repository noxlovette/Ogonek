import { env } from "$env/dynamic/private";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  subscribe: async ({ fetch, params, request }) => {
    const { id } = params;

    const formData = await request.formData();

    const isSubscribed = formData.get("isSubscribed") === "true";
    const response = await fetch(routes.learn.subscribe_to({ id }), {
      method: isSubscribed ? "DELETE" : "POST",
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error(errorData);
      return fail(400);
    }

    return { success: true };
  },
  share: async ({ params }) => {
    const { id } = params;

    const link = `${env.ORIGIN}/s/flashcards/${id}`;

    return { link };
  },
  duplicate: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.decks.duplicate({ id }), {
      method: "POST",
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData }, "ERROR SVELTE SIDE LESSON CREATION");
      return fail(500);
    }
    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData }, "ERROR SVELTE SIDE CONTENT CREATION");
      return fail(500);
    }
    const { id: new_id } = await response.json();

    return redirect(301, `../flashcards/${new_id}/edit`);
  },
} satisfies Actions;
