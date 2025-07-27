import { dev } from "$app/environment";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { DeckSmall, NewResponse } from "$lib/types";
import { delay } from "$lib/utils";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

// Loads all decks
export const load: PageServerLoad = async ({ fetch, url }) => {
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";

  return {
    decksResponse: (dev ? delay(2500) : Promise.resolve())
      .then(() => fetch(routes.decks.all(search, assignee)))
      .then((res) => res.json()) as Promise<DeckSmall[]>,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.decks.new(), {
      method: "POST",
    });

    const newResult = await handleApiResponse<NewResponse>(response);

    if (!isSuccessResponse(newResult)) {
      logger.error({ newResult }, "Deck creation failed");
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    return redirect(301, `flashcards/${id}/edit`);
  },
};
