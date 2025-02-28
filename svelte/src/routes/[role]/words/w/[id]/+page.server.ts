import { fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({ fetch, params, request }) => {
    const { id } = params;

    const formData = await request.formData();

    const isSubscribed = formData.get("isSubscribed") === "true";

    const response = await fetch(`/axum/deck/learn/subscribe/${id}`, {
      method: isSubscribed ? "DELETE" : "POST",
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
