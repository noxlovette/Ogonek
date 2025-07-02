import logger from "$lib/logger";
import type { CardProgress } from "$lib/types";
import { shuffleArray } from "$lib/utils";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ fetch, depends }) => {
  depends("learn:subscribe", "learn:complete");
  try {
    const response = await fetch("/axum/deck/learn");
    const cards: CardProgress[] = await response.json();

    const shuffled = shuffleArray(cards);

    return {
      cards: shuffled,
    };
  } catch (err: any) {
    logger.error({ err }, "Error getting learn cards");
    return error(500);
  }
};
