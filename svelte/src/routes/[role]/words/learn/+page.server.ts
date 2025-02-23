import type { CardProgress } from "$lib/types";
import { fail } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch("/axum/deck/learn");
  const cards: CardProgress[] = await response.json();

  return {
    cards,
  };
};

export const actions: Actions = {
  default: async ({ fetch, request }) => {
    const formData = await request.formData();
    const cardId = formData.get("cardId");

    if (!cardId) {
      return fail(422, { message: "No Id Provided" });
    }

    const response = await fetch(`/axum/deck/learn/${cardId}`, {
      method: "PATCH",
      body: JSON.stringify({
        quality: Number(formData.get("quality")),
      }),
    });

    if (!response.ok) {
      return fail(500);
    }
  },
};
