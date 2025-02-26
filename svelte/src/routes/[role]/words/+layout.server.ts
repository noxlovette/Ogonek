import type { CardProgress } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, depends }) => {
  depends("learn:complete");
  const response = await fetch("/axum/deck/learn");
  const cards: CardProgress[] = await response.json();

  return {
    cards,
  };
};
