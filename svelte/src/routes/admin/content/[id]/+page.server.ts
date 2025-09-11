import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { Content } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
  const { id } = params;

  const response = await fetch(routes.admin.delete_content(id));

  const content: Content = await response.json();

  return {
    content,
  };
}) satisfies PageServerLoad;

export const actions: Actions = {
  update: async ({ params, request, fetch }) => {
    const { id } = params;
    const formData = await request.formData();
    const data = Object.fromEntries(formData);
    const body = z.updateContentBody.safeParse(data).data;

    console.log(body);
    const response = await fetch(routes.admin.update_content(id), {
      method: "PATCH",
      body: JSON.stringify(body),
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

    const response = await fetch(routes.admin.delete_content(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true, deleted: true };
  },
  publish: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.admin.publish_content(id), {
      method: "PUT",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true };
  },
  unpublish: async ({ params, fetch }) => {
    const { id } = params;

    const response = await fetch(routes.admin.unpublish_content(id), {
      method: "DELETE",
    });

    if (!response.ok) {
      return fail(500);
    }

    return { success: true };
  },
};
