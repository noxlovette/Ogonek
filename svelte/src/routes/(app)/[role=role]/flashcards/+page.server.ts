import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { DeckSmall, PaginatedResponse } from "$lib/types";
import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

// Loads all decks
export const load: PageServerLoad = async ({ fetch, url }) => {
  try {
    const page = url.searchParams.get("page") || "1";
    const per_page = url.searchParams.get("per_page") || "50";
    const search = url.searchParams.get("search") || "";
    const assignee = url.searchParams.get("assignee") || "";

    const decksPaginated = (await fetch(
      routes.decks.all(page, per_page, search, assignee),
    ).then((res) => res.json())) as PaginatedResponse<DeckSmall>;
    return {
      decksPaginated,
    };
  } catch (err) {
    logger.error({ err });
    return error(404);
  }
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
