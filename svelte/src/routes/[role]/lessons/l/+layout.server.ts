import type { Lesson } from "$lib/types";
import { parseMarkdown } from "$lib/utils";
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
    throw redirect(303, `/${params.role}/lessons/`);
  }
};
