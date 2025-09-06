import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params }) => {
  if (params.role !== "t") {
    redirect(301, "/unauthorised");
  }
};

export const actions: Actions = {
  default: async ({ fetch, request }) => {
    const formData = await request.formData();
    const invite = formData.has("isRegistered") || false;

    logger.info({ formData });
    const response = await fetch(
      routes.auth.generate_invite_link(invite.toString()),
      {
        method: "POST",
      },
    );

    if (!response.ok) {
      logger.error({ response }, "Failed to generate invite link");
      return fail(400, { error: "Failed to generate invite link" });
    }

    const link = await response.json();

    return {
      link,
    };
  },
};
