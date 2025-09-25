import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import type { EmptyResponse } from "$lib/types";
import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const { id } = params;
    const formData = await request.formData();
    const data = {
      title: formData.get("title")?.toString(),
      dueDate: new Date(formData.get("dueDate") + "T23:59:59").toISOString(),
      completed: formData.has("completed"),
      assignee: formData.get("assignee")?.toString(),
      markdown: formData.get("markdown")?.toString(),
    };

    const validation = z.updateTaskBody.safeParse(data);
    if (!validation.success) {
      return fail(400, {
        message: "Validation failed",
      });
    }
    const response = await fetch(routes.tasks.task(id || ""), {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(validation.data),
    });
    if (!response.ok) {
      const errorData: App.Error = await response.json();
      logger.error({ errorData, id }, "Error updating task");
      return fail(response.status, {
        message: errorData.message || "Update failed",
      });
    }
    return redirect(303, `/t/tasks/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const id = params.id;
    if (!id) {
      return fail(400);
    }
    const response = await fetch(routes.tasks.task(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json();

      logger.error({ errorData, task_id: id }, "Error deleting backend-side");

      return {
        success: false,
        error: errorData,
      };
    }

    redirect(303, "/t/tasks");
  },
  deleteFile: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("fileId") as string;

    const response = await fetch(routes.files.delete_file(id), {
      method: "DELETE",
    });

    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      logger.error({ deleteResult }, "Error deleting file");
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    return {
      success: true,
      message: "Deleted File",
    };
  },
} satisfies Actions;
