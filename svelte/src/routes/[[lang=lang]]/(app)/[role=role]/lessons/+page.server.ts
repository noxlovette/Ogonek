import { dev } from "$app/environment";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import { delay } from "$lib/utils";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const page = url.searchParams.get("page") || "1";
  const per_page = url.searchParams.get("per_page") || "50";
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";

  return {
    lessonsPaginated: (dev ? delay(2500) : Promise.resolve())
      .then(() => fetch(routes.lessons.all(page, per_page, search, assignee)))
      .then((res) => res.json()) as Promise<PaginatedResponse<LessonSmall>>,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.lessons.new(), {
      method: "POST",
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData }, "ERROR SVELTE SIDE LESSON CREATION");
      return fail(500);
    }

    const newResult = await handleApiResponse<string>(response);

    if (!isSuccessResponse(newResult)) {
      logger.error({ newResult }, "ERROR AXUM SIDE LESSON CREATION");
      return fail(newResult.status, { message: newResult.message });
    }

    const id = newResult.data;

    return redirect(301, `/t/lessons/${id}/edit`);
  },
};
