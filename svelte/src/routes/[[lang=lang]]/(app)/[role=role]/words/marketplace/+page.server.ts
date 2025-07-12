import type { DeckSmall } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch("/axum/deck/public");
  const decks: DeckSmall[] = await response.json();

  return {
    decks,
  };
};
