import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse } from "$lib/types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";

export const actions = {
  complete: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");
    const completed = formData.get("completed") === "true";
    const body = {
      completed,
      id,
    };

    const response = await fetch(`/axum/task/t/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);
    if (!isSuccessResponse(editResult)) {
      return fail(editResult.status, { message: editResult.message });
    }

    return {
      success: true,
    };
  },
  download: async ({ fetch, request }) => {
    const formData = request.formData();

    const key = (await formData).get("key") as string;
    const encodedKey = btoa(key);

    const response = await fetch(`/axum/s3/presign/${encodedKey}`);

    if (!response.ok) {
      return fail(400, { error: "Failed to generate presigned url" });
    }

    const presigned = await response.json();

    return {
      presigned,
    };
  },
} satisfies Actions;
