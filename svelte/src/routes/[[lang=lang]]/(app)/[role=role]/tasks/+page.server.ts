import logger from "$lib/logger";

import {
  handleApiResponse,
  isSuccessResponse,
  messages,
  notifyTelegram,
} from "$lib/server";
import type { NewResponse, PaginatedResponse, TaskSmall } from "$lib/types";
import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url, depends }) => {
  depends("tasks:completed");
  const page = url.searchParams.get("page") || "1";
  const per_page = url.searchParams.get("per_page") || "50";
  const search = url.searchParams.get("search") || "";
  const assignee = url.searchParams.get("assignee") || "";
  const completed = url.searchParams.get("completed");

  const params = new URLSearchParams();
  params.append("page", page);
  params.append("per_page", per_page);

  if (search) params.append("search", search);
  if (assignee) params.append("assignee", assignee);
  if (completed) params.append("completed", completed);

  const apiUrl = `/axum/task?${params.toString()}`;

  const tasksPaginated = await fetch(apiUrl).then(
    (res) => res.json() as Promise<PaginatedResponse<TaskSmall>>,
  );

  const response = await fetch(apiUrl);
  const result =
    await handleApiResponse<PaginatedResponse<TaskSmall>>(response);

  if (!isSuccessResponse(result)) {
    return error(500);
  }

  return {
    tasksPaginated,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
    const startTime = performance.now();
    const body = {
      title: "New Task",
      markdown: "## Try adding some content here",
    };

    const response = await fetch(`/axum/task`, {
      method: "POST",
      body: JSON.stringify(body),
    });

    const newResult = await handleApiResponse<NewResponse>(response);

    if (!isSuccessResponse(newResult)) {
      logger.info(
        { duration: performance.now() - startTime },
        "Task creation failed axum-side",
      );
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    if (response.ok) {
      logger.info(
        { duration: performance.now() - startTime, id },
        "Task creation completed",
      );
      return redirect(301, `/t/tasks/t/${id}/edit`);
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
