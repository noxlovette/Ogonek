import { routes } from "$lib/routes";
import type { AppContext } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ fetch }) => {
  const context = (await fetch(routes.state.context()).then((res) =>
    res.json(),
  )) as AppContext;
  return { context };
}) satisfies LayoutServerLoad;
