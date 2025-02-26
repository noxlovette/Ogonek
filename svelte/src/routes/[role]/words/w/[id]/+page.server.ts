import { fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({ fetch, params }) => {
    const { id } = params;

    const response = await fetch(`/axum/deck/learn/init/${id}`, {
      method: "POST",
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => null);
      return fail(response.status, {
        message: errorData?.message || "Failed to update deck",
      });
    }

    return { success: true };
  },
} satisfies Actions;
