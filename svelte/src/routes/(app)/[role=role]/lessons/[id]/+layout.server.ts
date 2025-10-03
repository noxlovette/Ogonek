import { routes } from "$lib/routes";
import type { LessonFull } from "$lib/types";

import { parseMarkdown } from "$lib/utils/markdown";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch, depends }) => {
  depends("edit:photo");
  const response = await fetch(routes.lessons.lesson({ id: params.id }));
  if (!response.ok) {
    throw redirect(303, `/${params.role}/lessons/`);
  }
  const lesson: LessonFull = await response.json();
  const rendered = await parseMarkdown(lesson.markdown);

  return {
    lesson,
    rendered,
  };
};
