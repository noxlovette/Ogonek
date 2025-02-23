import type { CardProgress } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch }) => {
  const response = await fetch("/axum/deck/learn");
  const cards: CardProgress[] = await response.json();

  return {
    cards,
  };
};
