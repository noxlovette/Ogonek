import logger from "$lib/logger";
import { routes } from "$lib/routes";
import {
  handleApiResponse,
  isSuccessResponse,
  messages,
  notifyTelegram,
} from "$lib/server";
import type { EmptyResponse, URLResponse } from "$lib/types";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";

export const actions = {
  complete: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const { id } = params;
    const username = formData.get("username") as string;
    const task = formData.get("task") as string;
    const teacherTelegramId = formData.get("teacherTelegramId") as string;
    if (!id) {
      logger.warn("Task completion attempted without ID");
      return fail(400, { message: "Task ID is required" });
    }

    if (teacherTelegramId) {
      const telegramResponse = await notifyTelegram(
        messages.completed({ task, username, id }),
        teacherTelegramId,
      );
      if (!telegramResponse.ok) {
        logger.error(
          { telegramResponse },
          "Error in notifying teacher task completed",
        );
      }
    }

    const response = await fetch(routes.tasks.single(id), {
      method: "PUT",
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(editResult)) {
      logger.error({ id }, "completing task failed");
      return fail(editResult.status, { message: editResult.message });
    }

    return { success: true };
  },

  download: async ({ fetch, request }) => {
    const formData = await request.formData();
    const key = formData.get("key") as string;

    if (!key) {
      logger.warn("File download attempted without key");
      return fail(400, { error: "File key is required" });
    }

    const encodedKey = btoa(key);

    const response = await fetch(routes.s3.presign(encodedKey));

    if (!response.ok) {
      return fail(response.status, {
        error: "Failed to generate download URL",
      });
    }

    const { url } = (await response.json()) as URLResponse;

    return { url, success: true };
  },
  deleteFile: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("fileId") as string;
    if (!id) {
      return fail(400);
    }
    logger.info("Deleting file");

    const response = await fetch(routes.s3.single(id), { method: "DELETE" });
    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      logger.error("File deletion failed");
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    logger.info("File deleted successfully");

    return { success: true, message: "File deleted" };
  },
} satisfies Actions;
