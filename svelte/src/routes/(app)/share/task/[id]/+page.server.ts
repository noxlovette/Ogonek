import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { TaskPublicWithFiles, URLResponse } from "$lib/types";
import type { components } from "$lib/types/api/gen/openapi";
import { parseMarkdown } from "$lib/utils/markdown";
import type { Actions } from "@sveltejs/kit";
import { error, fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
  const { id } = params;
  if (!id) {
    return fail(400);
  }

  const response = await fetch(routes.public.task_public({ id }));

  if (!response.ok) {
    const err = await response.text();
    logger.error({ err }, "failed to fetch public task");

    return error(404);
  }
  const task: TaskPublicWithFiles = await response.json();

  const rendered = await parseMarkdown(task.markdown);
  return {
    task,
    files: task.files,
    rendered,
  };
}) satisfies PageServerLoad;

export const actions = {
  complete: async ({ fetch, params }) => {
    const { id } = params;

    if (!id) {
      return fail(500);
    }
    const response = await fetch(routes.public.task_public({ id }), {
      method: "PUT",
    });

    if (!response.ok) {
      logger.error({ id }, "error making task as completed");
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

    const response = await fetch(
      routes.files.presigned_url({ encoded_key: encodedKey }),
    );

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

    const files = fetch(
      routes.files.presigned_urls_batch({ task_id: params.id }),
      {
        method: "POST",
      },
    );
    const pdf = fetch(routes.files.pdf({ id: params.id, pdfType: "task" }), {
      method: "POST",
    });
    const [response1, response2] = await Promise.all([files, pdf]);

    const { urls } =
      (await response1.json()) as components["schemas"]["BatchPresignedUrlResponse"];

    const pdfUrl = (await response2.json()) as URLResponse;

    return { urls: [pdfUrl, ...urls], success: true };
  },
} satisfies Actions;
