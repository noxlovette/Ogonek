import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();

    const { id } = params;
    const name = formData.get("name");
    const description = formData.get("description");
    const visibility = formData.get("visibility");

    const assigneeData = formData.get("student")?.toString() || "{}";
    const { assignee = undefined } = JSON.parse(assigneeData);

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
        visibility,
        assignee,
      },
      cards,
    };

    const response = await fetch(`/axum/deck/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(editResult)) {
      return fail(editResult.status, { message: editResult.message });
    }

    return redirect(301, ".");
  },

  delete: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(`/axum/deck/${id}`, {
      method: "DELETE",
    });

    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    return redirect(301, "../../");
  },
} satisfies Actions;
