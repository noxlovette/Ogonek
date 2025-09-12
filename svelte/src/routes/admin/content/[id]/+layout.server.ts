import { routes } from "$lib/routes";
import type { Content } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
  const { id } = params;

  const response = await fetch(routes.admin.delete_content(id));

  const content: Content = await response.json();
  const rendered = await parseMarkdown(content.markdown);
  return {
    content,
    rendered,
  };
}) satisfies LayoutServerLoad;
