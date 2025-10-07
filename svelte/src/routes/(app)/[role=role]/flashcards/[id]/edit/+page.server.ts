import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { DeckUpdate } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const { id } = params;

    const title = formData.get("title") as string;
    const description = formData.get("description");
    const visibility = formData.get("visibility");
    const assignee = formData.get("assignee")?.toString() || "";

    if (visibility === "shared" && assignee.trim() === "") {
      return fail(400, { assignee: true });
    }

    console.log(formData);

    if (title.trim() === "") {
      return fail(400, { title: true });
    }
    const deck: DeckUpdate = {
      title: title ?? null,
      description: description?.toString() || null,
      visibility: visibility?.toString() || null,
      assignee: assignee.toString() || null,
    };

    const cards = [];
    let index = 0;
    while (formData.has(`cards[${index}][front]`)) {
      cards.push({
        id: formData.get(`cards[${index}][id]`) || null,
        front: formData.get(`cards[${index}][front]`),
        back: formData.get(`cards[${index}][back]`),
        mediaUrl: formData.get(`cards[${index}][media_url]`) || null,
      });
      index++;
    }
    const body = { deck, cards };

    const validatedBody = z.updateDeckBody.safeParse(body);

    console.log(body);
    if (validatedBody.error) {
      const error = validatedBody.error.message;
      logger.error({ error }, "error validating deck");
      return fail(400, { cards: true });
    }

    const response = await fetch(routes.decks.deck({ id }), {
      method: "PATCH",
      body: JSON.stringify(validatedBody.data),
    });

    if (!response.ok) {
      const errorData = await response.text();
      console.error("Update failed:", errorData);
      return fail(response.status, {
        message: "Failed to update deck",
      });
    }

    return redirect(301, ".");
  },

  delete: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.decks.deck({ id }), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return redirect(301, "../");
  },
} satisfies Actions;
