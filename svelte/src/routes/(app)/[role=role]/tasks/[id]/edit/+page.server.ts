import logger from "$lib/logger";
import { routes } from "$lib/routes";
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
    const dateString = formData.get("dueDate")?.toString();

    let dueDate: string | null;
    try {
      dueDate =
        dateString && dateString.trim() !== ""
          ? new Date(dateString + "T23:59:59").toISOString()
          : null;
    } catch {
      return fail(400, { date: true });
    }

    const assignee = formData.get("assignee")?.toString();
    const title = formData.get("title")?.toString();

    if (title?.trim() == "") {
      return fail(400, { title: true });
    }

    if (assignee?.trim() == "") {
      return fail(400, { assignee: true });
    }

    const data = {
      title: formData.get("title")?.toString(),
      assignee: assignee && assignee.trim() !== "" ? assignee : null,
      unassign: !formData.has("assigned"),
      visibility: formData.get("visibility"),
      dueDate,
      markdown: formData.get("markdown")?.toString(),
    };

    console.log(data);

    const response = await fetch(routes.tasks.task({ id }), {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData, id }, "Error updating task");
      return fail(500);
    }
    return redirect(303, `/t/tasks/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const id = params.id;
    if (!id) {
      return fail(400);
    }
    const response = await fetch(routes.tasks.task({ id }), {
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

    const response = await fetch(routes.files.delete_file({ id }), {
      method: "DELETE",
    });

    if (!response.ok) {
      logger.error({ id }, "error deleting file");
      return fail(500);
    }

    return {
      success: true,
      message: "Deleted File",
    };
  },
} satisfies Actions;
