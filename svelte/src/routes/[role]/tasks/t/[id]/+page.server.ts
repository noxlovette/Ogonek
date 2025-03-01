import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse, Task } from "$lib/types";
import { parseMarkdown } from "$lib/utils";
import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
  const { id } = params;
  try {
    const response = await fetch(`/axum/task/t/${id}`);
    if (!response.ok) {
      throw redirect(303, "/s/tasks/");
    }
    const task: Task = await response.json();

    const rendered = await parseMarkdown(task.markdown);

    return {
      task,
      rendered,
    };
  } catch (err) {
    console.log(err);
    throw redirect(303, "/s/tasks/");
  }
};

export const actions = {
  default: async ({ request, fetch }) => {
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
} satisfies Actions;
