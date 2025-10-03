import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { DeckWithCards } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, params }) => {
  const { id } = params;

  const response = await fetch(routes.decks.deck({ id }));

  if (!response.ok) {
    return error(response.status);
  }

  logger.debug(response);
  const deckComposite: DeckWithCards = await response.json();
  logger.debug(deckComposite);
  const { deck, cards } = deckComposite;
  logger.debug(deck);
  return { deck, cards };
};
