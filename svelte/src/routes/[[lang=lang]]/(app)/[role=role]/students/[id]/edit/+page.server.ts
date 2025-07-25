import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from "@sveltejs/kit";

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();
    const markdown = formData.get("markdown");
    const studentTelegramId = formData.get("studentTelegramId");
    const { id } = params;
    if (!id) {
      return fail(400);
    }
    const body = {
      markdown,
      studentTelegramId,
    };

    const response = await fetch(routes.students.single(id), {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.json();

      return {
        success: false,
        error: errorData,
      };
    }
    return redirect(303, `/t/students/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const { id } = params;

    if (!id) {
      return fail(400);
    }
    const response = await fetch(routes.students.single(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json();
      logger.error({ errorData }, "Error deleting student");
      return {
        success: false,
        error: errorData,
      };
    }

    return redirect(303, "/t/students");
  },
} satisfies Actions;
