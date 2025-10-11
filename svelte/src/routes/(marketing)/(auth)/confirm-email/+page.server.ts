import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ fetch, url }) => {
  const token = url.searchParams.get("token");

  if (!token) {
    return fail(400, { tokenAbsent: true });
  }

  const response = await fetch(routes.auth.confirm_email({ token }), {
    method: "POST",
  });

  if (!response.ok) {
    const err = await response.text();
    logger.error({ err, status: response.status });
    return {
      success: false,
    };
  }
  return {
    success: true,
  };
}) satisfies PageServerLoad;
