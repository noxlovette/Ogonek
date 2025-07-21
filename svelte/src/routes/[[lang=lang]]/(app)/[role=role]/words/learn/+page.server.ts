import logger from "$lib/logger";
import { fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
  // Update the progress of the card
  default: async ({ fetch, request }) => {
    const formData = await request.formData();
    const cardId = formData.get("cardId");

    if (!cardId) {
      return fail(422, { message: "No Id Provided" });
    }

    const response = await fetch(`/axum/deck/learn/${cardId}`, {
      method: "PATCH",
      body: JSON.stringify({
        quality: Number(formData.get("quality")),
      }),
    });

    if (!response.ok) {
      logger.error(
        { errorData: response.json() },
        "Error updating learn status on axum side",
      );
      return fail(500);
    }
  },
};
