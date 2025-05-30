import {
  handleApiResponse,
  isSuccessResponse,
  notifyTelegram,
} from "$lib/server";
import type {
  EmptyResponse,
  NewResponse,
  PaginatedResponse,
  Task,
} from "$lib/types";
import { fail, redirect, type Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url, depends }) => {
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
    (res) => res.json() as Promise<PaginatedResponse<Task>>,
  );

  depends("tasks:completed");
  return {
    tasksPaginated,
  };
};

export const actions: Actions = {
  new: async ({ fetch }) => {
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
      return fail(newResult.status, { message: newResult.message });
    }

    const { id } = newResult.data;

    if (response.ok) {
      return redirect(301, `/t/tasks/t/${id}/edit`);
    }
  },
  requestHW: async ({ request }) => {
    const formData = await request.formData();
    const username = formData.get("username");
    const teacherTelegramId = formData.get("teacherTelegramId") as string;

    if (teacherTelegramId) {
      const message = `${username} needs homework`;

      const telegramResponse = await notifyTelegram(message, teacherTelegramId);
      if (telegramResponse.status !== 200) {
        return fail(400);
      }
    }
  },
  completed: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");
    const completed = formData.get("completed") === "true";
    const body = {
      completed,
      id,
    };

    const response = await fetch(`/axum/task/t/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    const editResult = await handleApiResponse<EmptyResponse>(response);
    if (!isSuccessResponse(editResult)) {
      return fail(editResult.status, { message: editResult.message });
    }

    return {
      success: true,
    };
  },
};
