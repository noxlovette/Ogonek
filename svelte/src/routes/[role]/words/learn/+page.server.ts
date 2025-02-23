import type { CardProgress } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";
export const load: PageServerLoad = async ({ fetch, url }) => {
  const response = await fetch("/axum/deck/learn");
  const cards: CardProgress[] = await response.json();

  if (cards.length > 0 && !url.searchParams.has("cardId")) {
    throw redirect(302, `?cardId=${cards[0].id}`);
  }

  return {
    cards,
  };
};

export const actions: Actions = {
  default: async ({ fetch, request, url }) => {
    const formData = await request.formData();
    const cardId = url.searchParams.get("cardId");

    if (!cardId) {
      return fail(422, { message: "No Id Provided" });
    }

    console.log(formData);

    const response = await fetch(`/axum/deck/learn/${cardId}`, {
      method: "PATCH",
      body: JSON.stringify({
        quality: Number(formData.get("quality")),
      }),
    });

    console.log(response);

    if (!response.ok) {
      return fail(500);
    }
  },
};
