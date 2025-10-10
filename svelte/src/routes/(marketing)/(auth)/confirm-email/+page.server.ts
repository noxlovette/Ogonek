import type { PageServerLoad } from "./$types";

export const load = (async ({ fetch, params, url }) => {
  return {
    success: true,
  };
}) satisfies PageServerLoad;
