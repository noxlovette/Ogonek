import { routes } from "$lib/routes";
import type { CardProgress } from "$lib/types";
import { shuffleArray } from "$lib/utils";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, depends }) => {
  depends("learn:subscribe", "learn:complete");
  const response = await fetch(routes.learn.due_cards());
  const cards: CardProgress[] = await response.json();

  const shuffled = shuffleArray(cards);

  return {
    cards: shuffled,
  };
};
