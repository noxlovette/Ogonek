import logger from "$lib/logger";
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
    logger.info({ id }, "Starting task update");

    const formData = await request.formData();
    const markdown = formData.get("markdown")?.toString() || "";
    const title = formData.get("title")?.toString() || "";
    const dueDate = formData.get("dueDate")?.toString();
    if (!dueDate) {
      logger.error({ dueDate }, "No due date. bailing out");
      return fail(400, { message: "Add due date" });
    }
    const completed = formData.has("completed");
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
      assignee,
      dueDate: dueDateWithTime,
      completed,
      filePath,
    };

    const response = await fetch(`/axum/task/t/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData: App.Error = await response.json();
      const { code, message } = errorData;
      return error(code || 400, message);
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

    return redirect(303, `/t/tasks/t/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const id = params.id;
    const response = await fetch(`/axum/task/t/${id}`, {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json();

      return {
        success: false,
        error: errorData,
      };
    }

    redirect(303, "/t/tasks");
  },
  upload: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const file = formData.get("file") as File;

    if (!file) throw new Error("yikes, no file");

    const response = await fetch(`/axum/file?task_id=${params.id}`, {
      method: "POST",
      body: formData,
    });

    const uploadResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(uploadResult)) {
      return fail(uploadResult.status, { message: uploadResult.message });
    }

    return {
      success: true,
      message: "Uploaded successfully",
    };
  },
  deleteFile: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("fileId");

    const response = await fetch(`/axum/file/${id}`, { method: "DELETE" });

    const deleteResult = await handleApiResponse<EmptyResponse>(response);

    if (!isSuccessResponse(deleteResult)) {
      return fail(deleteResult.status, { message: deleteResult.message });
    }

    return {
      success: true,
      message: "Deleted File",
    };
  },
} satisfies Actions;
