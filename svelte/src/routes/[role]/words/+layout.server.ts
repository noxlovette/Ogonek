import type { CardProgress } from "$lib/types";
import { shuffleArray } from "$lib/utils";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, depends }) => {
  depends("learn:complete");
  depends("learn:subscribe");
  const response = await fetch("/axum/deck/learn");
  const cards: CardProgress[] = await response.json();

  const shuffled = shuffleArray(cards);

  return {
    cards: shuffled,
  };
};
