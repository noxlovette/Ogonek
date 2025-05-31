import { env } from "$env/dynamic/private";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse } from "$lib/types";
import { fail } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  subscribe: async ({ fetch, params, request }) => {
    const { id } = params;

    const formData = await request.formData();

    const isSubscribed = formData.get("isSubscribed") === "true";
    const response = await fetch(`/axum/deck/learn/subscribe/${id}`, {
      method: isSubscribed ? "DELETE" : "POST",
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(editResult)) {
      return fail(editResult.status, { message: editResult.message });
    }

    return { success: true };
  },
  share: async ({ params }) => {
    const { id } = params;

    const link = `${env.ORIGIN}/s/words/w/${id}`;

    return { link };
  },
} satisfies Actions;
