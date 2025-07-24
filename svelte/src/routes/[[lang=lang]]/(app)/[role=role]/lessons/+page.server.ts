import { dev } from "$app/environment";
import logger from "$lib/logger";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { LessonSmall, NewResponse, PaginatedResponse } from "$lib/types";
import { delay } from "$lib/utils";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const page = url.searchParams.get("page") || "1";
  const per_page = url.searchParams.get("per_page") || "50";
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";

  const params = new URLSearchParams();
  params.append("page", page);
  params.append("per_page", per_page);

  if (search) params.append("search", search);
  if (assignee) params.append("assignee", assignee);

  const apiUrl = `/axum/lesson?${params.toString()}`;

  return {
    lessonsPaginated: (dev ? delay(2500) : Promise.resolve())
      .then(() => fetch(apiUrl))
      .then((res) => res.json()) as Promise<PaginatedResponse<LessonSmall>>,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const body = {
      title: "New Lesson",
      markdown: "## Try adding some content here",
      topic: "General",
    };

    const response = await fetch(`/axum/lesson`, {
      method: "POST",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData }, "ERROR SVELTE SIDE LESSON CREATION");
      return fail(500);
    }

    const newResult = await handleApiResponse<NewResponse>(response);

    if (!isSuccessResponse(newResult)) {
      logger.error({ newResult }, "ERROR AXUM SIDE LESSON CREATION");
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    if (response.ok) {
      return redirect(301, `/t/lessons/l/${id}/edit`);
    }
  },
};
