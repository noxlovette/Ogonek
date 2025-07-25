import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { TaskWithFiles } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const { id, role } = params;
  const response = await fetch(routes.tasks.single(id));
  if (!response.ok) {
    logger.error({ id }, "Task load failed");

    throw redirect(303, `/${role}/tasks/`);
  }
  const taskWithFiles: TaskWithFiles = await response.json();

  const { task, files } = taskWithFiles;

  const rendered = await parseMarkdown(task.markdown);

  return {
    task,
    files,
    rendered,
  };
};
