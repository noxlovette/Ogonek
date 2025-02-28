import type { DeckWithCards } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, params }) => {
  try {
    const { id } = params;
    const endpoint = `/axum/deck/${id}`;

    const response = await fetch(endpoint, {
      method: "GET",
    });

    if (!response.ok) {
      return error(response.status);
    }

    const deckComposite: DeckWithCards = await response.json();
    const { deck, cards, isSubscribed } = deckComposite;
    return { deck, cards, isSubscribed };
  } catch (err: any) {
    return error(500, err);
  }
};
