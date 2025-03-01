import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { Deck, NewResponse } from "$lib/types";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch("/axum/deck");
  const decks: Deck[] = await response.json();

  return {
    decks,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const body = {
      name: "New Deck",
      description: "Blablablabla",
      shared: false,
    };

    const response = await fetch(`/axum/deck`, {
      method: "POST",
      body: JSON.stringify(body),
    });

    const newResult = await handleApiResponse<NewResponse>(response);

    if (!isSuccessResponse(newResult)) {
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    return redirect(301, `words/w/${id}/edit`);
  },
};
