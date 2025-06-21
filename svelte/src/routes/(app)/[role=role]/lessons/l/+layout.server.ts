import logger from "$lib/logger";
import type { Lesson } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  try {
    const response = await fetch(`/axum/lesson/l/${params.id}`);
    if (!response.ok) {
      throw redirect(303, `/${params.role}/lessons/`);
    }
    const lesson: Lesson = await response.json();
    const rendered = await parseMarkdown(lesson.markdown);

    return {
      lesson,
      rendered,
    };
  } catch (e) {
    logger.warn({ e }, "lesson load failed");
    throw redirect(303, `/${params.role}/lessons/`);
  }
};
