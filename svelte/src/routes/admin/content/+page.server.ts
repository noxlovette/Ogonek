import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { handleApiResponse, isSuccessResponse } from "$lib/server";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch(routes.admin.content());

  const content = await response.json();

  return {
    content: content || [],
  };
};
export const actions: Actions = {
  new: async ({ fetch }) => {
    const response = await fetch(routes.admin.new_content(), {
      method: "POST",
    });

    if (!response.ok) {
      const errorData = await response.text();
      logger.error({ errorData }, "ERROR SVELTE SIDE CONTENT CREATION");
      return fail(500);
    }

    const newResult = await handleApiResponse<string>(response);

    if (!isSuccessResponse(newResult)) {
      logger.error({ newResult }, "ERROR AXUM SIDE CONTENT CREATION");
      return fail(newResult.status, { message: newResult.message });
    }

    const id = newResult.data;

    return redirect(301, `/admin/content/${id}/edit`);
  },
};
