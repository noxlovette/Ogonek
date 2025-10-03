import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { Student } from "$lib/types";
import { parseMarkdown } from "$lib/utils/markdown";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const { id } = params;
  const response = await fetch(routes.users.student({ id }));
  if (!response.ok) {
    const err = await response.text();
    logger.error({ err });
    if (!response.ok) {
      throw redirect(303, `/${params.role}/students/`);
    }
  }
  const student: Student = await response.json();
  const rendered = await parseMarkdown(student.markdown || "");

  return {
    student,
    rendered,
  };
};
