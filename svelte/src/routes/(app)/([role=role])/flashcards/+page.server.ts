import { dev } from "$app/environment";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { DeckSmall, PaginatedResponse } from "$lib/types";
import { delay } from "$lib/utils";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

// Loads all decks
export const load: PageServerLoad = async ({ fetch, url }) => {
  const page = url.searchParams.get("page") || "1";
  const per_page = url.searchParams.get("per_page") || "50";
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";

  return {
    decksPaginated: (dev ? delay(100) : Promise.resolve())
      .then(() => fetch(routes.decks.all(page, per_page, search, assignee)))
      .then((res) => res.json()) as Promise<PaginatedResponse<DeckSmall>>,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.decks.new(), {
      method: "POST",
    });

    const newResult = await handleApiResponse<string>(response);
    if (!isSuccessResponse(newResult)) {
      logger.error({ newResult }, "Deck creation failed");
      return fail(newResult.status, { message: newResult.message });
    }

    const id = newResult.data;
    return redirect(301, `flashcards/${id}/edit`);
  },
};
