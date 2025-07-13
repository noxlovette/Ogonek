import logger from "$lib/logger";
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
  complete: async ({ request, fetch }) => {
    const startTime = performance.now();

    try {
      const formData = await request.formData();
      const id = formData.get("id") as string;
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

      const response = await fetch(`/axum/task/t/${id}`, {
        method: "PUT",
      });

      const editResult = await handleApiResponse<EmptyResponse>(response);

      if (!isSuccessResponse(editResult)) {
        logger.error(
          {
            taskId: id,
            status: editResult.status,
            error: editResult.message,
            duration: performance.now() - startTime,
          },
          "Task completion update failed",
        );
        return fail(editResult.status, { message: editResult.message });
      }

      logger.info(
        {
          taskId: id,
          duration: performance.now() - startTime,
        },
        "Task completion updated successfully",
      );

      return { success: true };
    } catch (err: any) {
      logger.error("Unexpected error in task completion", {
        error: err.message,
        stack: err.stack,
        duration: performance.now() - startTime,
      });
      return fail(500, { message: "Internal server error" });
    }
  },

  download: async ({ fetch, request }) => {
    const startTime = performance.now();

    try {
      const formData = await request.formData();
      const key = formData.get("key") as string;

      // Validate the key exists
      if (!key) {
        logger.warn("File download attempted without key");
        return fail(400, { error: "File key is required" });
      }

      const encodedKey = btoa(key);

      logger.info(
        {
          key: key.substring(0, 20) + "...", // Truncate for security
          encodedKey: encodedKey.substring(0, 20) + "...",
          timestamp: new Date().toISOString(),
        },
        "Generating presigned URL for file download",
      );

      const response = await fetch(`/axum/file/presigned/${encodedKey}`);

      if (!response.ok) {
        logger.error(
          {
            key: key.substring(0, 20) + "...",
            status: response.status,
            statusText: response.statusText,
            duration: performance.now() - startTime,
          },
          "Failed to generate presigned URL",
        );
        return fail(response.status, {
          error: "Failed to generate download URL",
        });
      }

      const { url } = (await response.json()) as URLResponse;

      logger.info(
        {
          key: key.substring(0, 20) + "...",
          duration: performance.now() - startTime,
        },
        "Presigned URL generated successfully",
      );

      return { url, success: true };
    } catch (err: any) {
      logger.error(
        {
          error: err.message,
          stack: err.stack,
          duration: performance.now() - startTime,
        },
        "Unexpected error in file download",
      );
      return fail(500, { error: "Internal server error" });
    }
  },
  deleteFile: async ({ request, fetch }) => {
    const startTime = performance.now();

    try {
      const formData = await request.formData();
      const id = formData.get("fileId");

      // More context = easier debugging
      logger.info(
        {
          fileId: id,
          userAgent: request.headers.get("user-agent"),
          timestamp: new Date().toISOString(),
        },
        "Deleting file",
      );

      const response = await fetch(`/axum/file/${id}`, { method: "DELETE" });
      const deleteResult = await handleApiResponse<EmptyResponse>(response);

      if (!isSuccessResponse(deleteResult)) {
        // Log the actual error details - future you will thank you
        logger.error(
          {
            fileId: id,
            status: deleteResult.status,
            error: deleteResult.message,
            duration: performance.now() - startTime,
          },
          "File deletion failed",
        );
        return fail(deleteResult.status, { message: deleteResult.message });
      }

      logger.info(
        {
          fileId: id,
          duration: performance.now() - startTime,
        },
        "File deleted successfully",
      );

      return { success: true, message: "File deleted" };
    } catch (err: any) {
      // Catch any unexpected errors
      logger.error(
        {
          error: err.message,
          stack: err.stack,
          duration: performance.now() - startTime,
        },
        "Unexpected error in deleteFile",
      );
      return fail(500, { message: "Internal server error" });
    }
  },
} satisfies Actions;
