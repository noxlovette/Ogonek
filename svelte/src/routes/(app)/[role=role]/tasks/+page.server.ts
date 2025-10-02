import logger from "$lib/logger";

import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url, depends }) => {
  depends("tasks:completed");

  try {
    const page = url.searchParams.get("page") || "1";
    const per_page = url.searchParams.get("per_page") || "50";
    const search = url.searchParams.get("search") || "";
    const assignee = url.searchParams.get("assignee") || "";
    const completed = url.searchParams.get("completed") || "false";
    const tasksPaginated = (await fetch(
      routes.tasks.all(page, per_page, search, assignee, completed),
    ).then((res) => res.json())) as PaginatedResponse<TaskSmall>;
    return {
      tasksPaginated,
    };
  } catch (err) {
    logger.error({ err });
    return error(404);
  }
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.tasks.new(), {
      method: "POST",
    });

    const newResult = await handleApiResponse<string>(response);

    if (!isSuccessResponse(newResult)) {
      logger.info("Task creation failed axum-side");
      return fail(newResult.status, { message: newResult.message });
    }

    const id = newResult.data;

    if (response.ok) {
      logger.info("Task creation completed");
      return redirect(301, `/t/tasks/${id}/edit`);
    }
  },
  requestHW: async ({ fetch }) => {
    const response = await fetch(routes.notifications.request_hw(), {
      method: "POST",
    });

    if (!response.ok) {
      return fail(500);
    }
  },
};
