import logger from "$lib/logger";
import type { Actions } from "@sveltejs/kit";
import { error, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    throw redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");
    try {
      const markdown = formData.get("markdown");
      const title = formData.get("title");
      const topic = formData.get("topic");
      const assigneeData = formData.get("student")?.toString() || "{}";
      const { assignee = "" } = JSON.parse(assigneeData);

      const body = {
        id,
        title,
        markdown,
        topic,
        assignee,
      };

      const response = await fetch(`/axum/lesson/l/${formData.get("id")}`, {
        method: "PATCH",
        body: JSON.stringify(body),
      });

      if (!response.ok) {
        const errorData = await response.json(); // Parse error details
        logger.error({ errorData, id }, "Error updating lesson axum-side");
        return {
          success: false,
          error: errorData,
        };
      }

      redirect(303, `/t/lessons/l/${id}`);
    } catch (err: any) {
      logger.error({ err, id }, "Error updating lesson svelte-side");
      return error(500);
    }
  },
  delete: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");

    try {
      const response = await fetch(`/axum/lesson/l/${id}`, {
        method: "DELETE",
      });

      if (!response.ok) {
        const errorData = await response.json(); // Parse error details
        logger.error("Error deleting lesson axum-side", errorData);
        return {
          success: false,
          error: errorData,
        };
      }

      redirect(303, "/t/lessons");
    } catch (err: any) {
      logger.error({ err, id }, "Error deleting lesson svelte-side");
      return error(500);
    }
  },
} satisfies Actions;
