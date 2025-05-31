import type { Deck } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch("/axum/deck/public");
  const decks: Deck[] = await response.json();

  return {
    decks,
  };
};
