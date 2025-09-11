import { routes } from "$lib/routes";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch(routes.admin.content());

  const content = await response.json();

  return {
    content: content || [],
  };
};
