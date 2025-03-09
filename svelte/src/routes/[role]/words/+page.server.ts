import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { Deck, NewResponse } from "$lib/types";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";

  const params = new URLSearchParams();
  if (search) params.append("search", search);
  if (assignee) params.append("assignee", assignee);

  const response = await fetch(`/axum/deck?${params.toString()}`);

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
