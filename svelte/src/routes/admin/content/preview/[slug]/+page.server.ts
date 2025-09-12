import { routes } from "$lib/routes";
import type { ContentPublic } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import type { PageServerLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
  const { slug } = params;
  const publicContent = (await fetch(routes.content.content_public(slug)).then(
    (res) => res.json(),
  )) as ContentPublic;

  const rendered = await parseMarkdown(publicContent.markdown);
  return { publicContent, rendered };
}) satisfies PageServerLoad;
