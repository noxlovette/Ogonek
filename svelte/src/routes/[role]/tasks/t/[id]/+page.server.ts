import { env } from "$env/dynamic/private";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import { notifyTelegram } from "$lib/server/telegram";
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
    const response = await fetch(`/axum/s3/presign/${encodedKey}`);

    if (!response.ok) {
      return fail(400, { error: "Failed to fetch file url" });
    }

    const { url } = (await response.json()) as URLResponse;
    return {
      url,
      success: true,
    };
  },
  upload: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const file = formData.get("file") as File;

    if (!file) throw new Error("yikes, no file");

    const id = params.id;

    const response = await fetch(`/axum/file?task_id=${id}`, {
      method: "POST",
      body: formData,
    });

    const uploadResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(uploadResult)) {
      return fail(uploadResult.status, { message: uploadResult.message });
    }

    const message = `A task has been completed\\. Check the student's homework on [Ogonek](https://Ogonek\\.app/t/tasks/t/${id})\\.`;

    const telegramResponse = await notifyTelegram(
      message,
      env.TELEGRAM_CHAT_ID,
    );
    if (telegramResponse.status !== 404 && telegramResponse.status !== 200) {
      return fail(400);
    }

    return {
      success: true,
      message: "Uploaded successfully",
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
