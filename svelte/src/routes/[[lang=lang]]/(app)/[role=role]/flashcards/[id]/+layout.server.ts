import { routes } from "$lib/routes";
import type { DeckWithCards } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, params }) => {
  const { id } = params;

  const response = await fetch(routes.decks.single(id));

  if (!response.ok) {
    return error(response.status);
  }

  const deckComposite: DeckWithCards = await response.json();
  const { deck, cards, isSubscribed } = deckComposite;
  return { deck, cards, isSubscribed };
};
