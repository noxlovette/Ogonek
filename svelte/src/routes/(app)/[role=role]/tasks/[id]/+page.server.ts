import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse, URLResponse } from "$lib/types";
import type { components } from "$lib/types/api/gen/openapi";
import type { Actions } from "@sveltejs/kit";
import { fail } from "@sveltejs/kit";

export const actions = {
  complete: async ({ fetch, params }) => {
    const { id } = params;

    if (!id) {
      return fail(500);
    }
    const response = await fetch(routes.tasks.task(id), {
      method: "PUT",
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(editResult)) {
      logger.error({ editResult, id }, "completing task failed");
      return fail(500);
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

    const response = await fetch(routes.files.presigned_url(encodedKey));

    if (!response.ok) {
      return fail(response.status, {
        error: "Failed to generate download URL",
      });
    }

    const { url } = (await response.json()) as URLResponse;

    return { url, success: true };
  },
  downloadAll: async ({ fetch, params }) => {
    if (!params.id) {
      return fail(400);
    }

    const files = fetch(routes.files.presigned_urls_batch(params.id), {
      method: "POST",
    });
    const pdf = fetch(routes.files.pdf(params.id, "task"), { method: "POST" });
    const [response1, response2] = await Promise.all([files, pdf]);

    const { urls } =
      (await response1.json()) as components["schemas"]["BatchPresignedUrlResponse"];

    const pdfBlob = (await response2.json()) as Blob;
    return { urls, pdfBlob, success: true };
  },
  downloadPdf: async ({ fetch, params }) => {
    if (!params.id) {
      return fail(400);
    }

    const response = await fetch(routes.files.pdf(params.id, "task"), {
      method: "POST",
    });

    if (!response.ok) {
      const error = await response.text();
      logger.error({ error });
      return fail(400);
    }

    const pdfBlob = await response.blob();

    return { pdfBlob, success: true };
  },
  deleteFile: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("fileId") as string;
    if (!id) {
      return fail(400);
    }
    logger.info("Deleting file");

    const response = await fetch(routes.files.delete_file(id), {
      method: "DELETE",
    });
    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      logger.error("File deletion failed");
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    logger.info("File deleted successfully");

    return { success: true, message: "File deleted" };
  },
} satisfies Actions;
