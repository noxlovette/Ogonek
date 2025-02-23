import { error, fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();

    const { id } = params;
    const name = formData.get("name");
    const description = formData.get("description");
    const shared = formData.get("shared") === "on";

    const cards = [];
    let index = 0;
    while (formData.has(`cards[${index}][front]`)) {
      cards.push({
        id: formData.get(`cards[${index}][id]`) || null,
        front: formData.get(`cards[${index}][front]`),
        back: formData.get(`cards[${index}][back]`),
        media_url: formData.get(`cards[${index}][media_url]`) || null,
      });
      index++;
    }

    if (!name || typeof name !== "string") {
      return fail(400, {
        message: "Deck name is required",
      });
    }

    for (const card of cards) {
      if (!card.front || !card.back) {
        return fail(400, {
          message: "All cards must have both front and back content",
        });
      }
    }

    const body = {
      deck: {
        name,
        description,
        shared,
      },
      cards,
    };

    const response = await fetch(`/axum/deck/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => null);
      return fail(response.status, {
        message: errorData?.message || "Failed to update deck",
      });
    }

    return redirect(301, ".");
  },

  delete: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(`/axum/deck/${id}`, {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => null);
      throw error(
        response.status,
        errorData?.message || "Failed to delete the deck",
      );
    }

    return redirect(301, ".");
  },
} satisfies Actions;
