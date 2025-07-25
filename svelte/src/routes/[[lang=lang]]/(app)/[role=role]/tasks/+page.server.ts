import logger from "$lib/logger";

import { dev } from "$app/environment";
import { routes } from "$lib/routes";
import {
  handleApiResponse,
  isSuccessResponse,
  messages,
  notifyTelegram,
} from "$lib/server";
import type { NewResponse, PaginatedResponse, TaskSmall } from "$lib/types";
import { delay } from "$lib/utils";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url, depends }) => {
  depends("tasks:completed");
  const page = url.searchParams.get("page") || "1";
  const per_page = url.searchParams.get("per_page") || "50";
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";
  const completed = url.searchParams.get("completed") || "false";

  return {
    tasksPaginated: (dev ? delay(2000) : Promise.resolve())
      .then(() =>
        fetch(routes.tasks.all(page, per_page, search, assignee, completed)),
      )
      .then((res) => res.json()) as Promise<PaginatedResponse<TaskSmall>>,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.tasks.new(), {
      method: "POST",
    });

    const newResult = await handleApiResponse<NewResponse>(response);

    if (!isSuccessResponse(newResult)) {
      logger.info("Task creation failed axum-side");
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    if (response.ok) {
      logger.info("Task creation completed");
      return redirect(301, `/t/tasks/${id}/edit`);
    }
  },
  requestHW: async ({ request }) => {
    const formData = await request.formData();
    const username = formData.get("username") as string;
    const teacherTelegramId = formData.get("teacherTelegramId") as string;

    if (teacherTelegramId) {
      const telegramResponse = await notifyTelegram(
        messages.teacherNotify({ username }),
        teacherTelegramId,
      );
      if (telegramResponse.status !== 200) {
        return fail(400);
      }
    }
  },
};
