import logger from "$lib/logger";

import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url, depends }) => {
  depends("tasks:completed");

  try {
    const page = url.searchParams.get("page") || undefined;
    const per_page = url.searchParams.get("per_page") || undefined;
    const search = url.searchParams.get("search") || undefined;
    const assignee = url.searchParams.get("assignee") || undefined;
    const completed = url.searchParams.get("completed") || undefined;

    const sort_by = url.searchParams.get("sort_by") || undefined;
    const sort_order = url.searchParams.get("sort_order") || undefined;
    const tasksPaginated = (await fetch(
      routes.tasks.all({
        page,
        per_page,
        search,
        assignee,
        completed,
        sort_by,
        sort_order,
      }),
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
  delete: async ({ fetch, request }) => {
    const formData = await request.formData();

    const ids = formData.getAll("toDelete") as string[];
    if (ids.length > 0) {
      const response = await fetch(routes.tasks.delete_task_many(), {
        method: "DELETE",
        body: JSON.stringify(ids),
      });

      if (!response.ok) {
        const err = await response.text();
        logger.error({ err });
        return fail(500, { delete: true });
      }
    }
  },
};
