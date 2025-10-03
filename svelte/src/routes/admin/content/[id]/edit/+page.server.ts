import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
  update: async ({ params, request, fetch }) => {
    const { id } = params;
    const formData = await request.formData();
    const data = Object.fromEntries(formData);

    const response = await fetch(routes.admin.update_content({ id }), {
      method: "PATCH",
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const error = await response.text();
      logger.error({ error, id }, "Error updating content");
      return fail(500);
    }

    return redirect(302, ".");
  },

  delete: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.admin.delete_content({ id }), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true, deleted: true };
  },
  publish: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.admin.publish_content({ id }), {
      method: "PUT",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true };
  },
  unpublish: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.admin.unpublish_content({ id }), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true };
  },
};
