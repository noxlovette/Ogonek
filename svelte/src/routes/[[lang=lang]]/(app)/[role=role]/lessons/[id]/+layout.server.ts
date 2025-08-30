import { routes } from "$lib/routes";
import type { LessonFull } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const response = await fetch(routes.lessons.lesson(params.id));
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
