import { handleApiResponse, isSuccessResponse } from "$lib/server";
import { notifyTelegram } from "$lib/server/telegram";
import type { UploadResponse } from "$lib/types";
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
    const id = params.id;

    const formData = await request.formData();
    const markdown = formData.get("markdown")?.toString() || "";
    const title = formData.get("title")?.toString() || "";
    const dueDate = formData.get("dueDate")?.toString() || "";
    const completed = formData.has("completed");
    const filePath = formData.get("filePath")?.toString() || "";

    const assigneeData = formData.get("student")?.toString() || "{}";
    const { assignee = "", telegramId = "" } = JSON.parse(assigneeData);
    const initialAssignee = formData.get("initialAssignee")?.toString() || "";

    const dueDateWithTime =
      dueDate && dueDate !== ""
        ? new Date(`${dueDate}T23:59:59`).toISOString()
        : null;

    let body = {
      id,
      title,
      markdown,
      assignee,
      dueDate: dueDateWithTime,
      completed,
      filePath,
    };

    console.log(filePath);

    const response = await fetch(`/axum/task/t/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData: App.Error = await response.json();
      const { code, message } = errorData;
      return error(code || 400, message);
    }

    const message = `You have a new task: "${title}"\\. You can view it on [Firelight](https://firelight\\.noxlovette\\.com/s/tasks)\\.`;

    if (telegramId && initialAssignee !== assignee) {
      const telegramResponse = await notifyTelegram(message, telegramId);
      if (telegramResponse.status !== 404 && telegramResponse.status !== 200) {
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
  upload: async ({ request, fetch }) => {
    const formData = await request.formData();
    const file = formData.get("file") as File;

    if (!file) throw new Error("yikes, no file");

    const response = await fetch("/file-server/upload", {
      method: "POST",
      body: formData,
    });

    const uploadResult = await handleApiResponse<UploadResponse>(response);

    if (!isSuccessResponse(uploadResult)) {
      return fail(uploadResult.status, { message: uploadResult.message });
    }

    console.log(uploadResult);

    let { filePath } = uploadResult.data;

    return {
      success: true,
      filePath,
      message: "Uploaded successfully",
    };
  },
} satisfies Actions;
