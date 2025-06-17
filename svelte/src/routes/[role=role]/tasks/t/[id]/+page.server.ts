import logger from "$lib/logger";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse, URLResponse } from "$lib/types";
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
    logger.debug(key);
    logger.debug(encodedKey);
    const response = await fetch(`/axum/file/presigned/${encodedKey}`);

    if (!response.ok) {
      return fail(400, { error: "Failed to fetch file url" });
    }

    const { url } = (await response.json()) as URLResponse;
    return {
      url,
      success: true,
    };
  },
  deleteFile: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("fileId");

    const response = await fetch(`/axum/file/${id}`, { method: "DELETE" });

    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    return {
      success: true,
      message: "Deleted File",
    };
  },
} satisfies Actions;
