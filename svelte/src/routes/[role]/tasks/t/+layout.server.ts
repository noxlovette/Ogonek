import type { Task } from "$lib/types";
import { parseMarkdown } from "$lib/utils";
import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const { id, role } = params;
  try {
    const response = await fetch(`/axum/task/t/${id}`);
    if (!response.ok) {
      throw redirect(303, `/${role}/tasks/`);
    }
    const task: Task = await response.json();

    const rendered = await parseMarkdown(task.markdown);

    return {
      task,
      rendered,
    };
  } catch (err) {
    throw redirect(303, `/${role}/tasks/`);
  }
};
