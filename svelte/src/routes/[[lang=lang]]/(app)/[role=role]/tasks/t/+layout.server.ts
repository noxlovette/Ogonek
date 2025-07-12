import logger from "$lib/logger";
import type { TaskWithFiles } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const { id, role } = params;
  try {
    const response = await fetch(`/axum/task/t/${id}`);
    if (!response.ok) {
      logger.error({ errorData: response.json(), id }, "Task load failed");
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
  } catch (err) {
    logger.error({ err, id }, "Task load failed");
    throw redirect(303, `/${role}/tasks/`);
  }
};
