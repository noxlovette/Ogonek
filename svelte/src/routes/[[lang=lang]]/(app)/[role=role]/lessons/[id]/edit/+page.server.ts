import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { Actions } from "@sveltejs/kit";
import { fail, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    throw redirect(301, "/unauthorised");
  }
};

export const actions = {
  update: async ({ request, fetch, params }) => {
    const { id } = params;

    if (!id) {
      return fail(500);
    }
    const formData = await request.formData();
    const data = Object.fromEntries(formData);
    const body = z.updateLessonBody.safeParse(data).data;

    console.log(body);
    const response = await fetch(routes.lessons.lesson(id), {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.json();
      logger.error({ errorData, id }, "Error updating lesson axum-side");
      return {
        success: false,
        error: errorData,
      };
    }

    throw redirect(303, `/t/lessons/${id}`);
  },
  delete: async ({ fetch, params }) => {
    const { id } = params;
    if (!id) {
      return fail(500);
    }
    const response = await fetch(routes.lessons.lesson(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      const errorData = await response.json();
      logger.error("Error deleting lesson axum-side", errorData);
      return {
        success: false,
        error: errorData,
      };
    }

    return redirect(303, "/t/lessons");
  },
} satisfies Actions;
