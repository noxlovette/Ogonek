import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse, messages } from "$lib/server";
import { notifyTelegram } from "$lib/server/telegram";
import type { EmptyResponse } from "$lib/types";
import { formatDate } from "@noxlovette/svarog";
import type { Actions } from "@sveltejs/kit";
import { error, fail, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const id = params.id || "";
    const startTime = performance.now();

    const formData = await request.formData();
    const markdown = formData.get("markdown")?.toString() || "";
    const title = formData.get("title")?.toString() || "";
    const dueDate = formData.get("dueDate")?.toString();
    if (!dueDate) {
      logger.error({ dueDate }, "No due date. bailing out");
      return fail(400, { message: "Add due date" });
    }
    const completed = formData.has("completed");
    const priority = Number(formData.get("priority"));
    const filePath = formData.get("filePath")?.toString() || "";

    const assigneeData = formData.get("student")?.toString() || "{}";
    const { assignee = "", studentTelegramId = "" } = JSON.parse(assigneeData);
    const initialAssignee = formData.get("initialAssignee")?.toString() || "";

    const dueDateWithTime =
      dueDate && dueDate !== ""
        ? new Date(`${dueDate}T23:59:59`).toISOString()
        : null;

    const body = {
      id,
      title,
      markdown,
      priority,
      assignee,
      dueDate: dueDateWithTime,
      completed,
      filePath,
    };
    const response = await fetch(routes.tasks.single(id), {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData: App.Error = await response.json();
      logger.error({ errorData }, "Error updating task");
      return error(500);
    }

    if (studentTelegramId && initialAssignee !== assignee) {
      const telegramResponse = await notifyTelegram(
        messages.taskCreated({ title, id, date: formatDate(dueDate) }),
        studentTelegramId,
      );
      if (telegramResponse.status !== 404 && telegramResponse.status !== 200) {
        logger.error({ id }, "No notification sent for task");
        return fail(400);
      }
    }

    logger.info(
      { task_id: id, duration: performance.now() - startTime },
      "Successful task update",
    );

    return redirect(303, `/t/tasks/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const id = params.id;
    if (!id) {
      return fail(400);
    }
    const response = await fetch(routes.tasks.single(id), {
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

    const response = await fetch(routes.s3.single(id), { method: "DELETE" });

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
