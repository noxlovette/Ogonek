import { routes } from "$lib/routes";
import type { CompositeStudent } from "$lib/types";
import { parseMarkdown } from "@noxlovette/svarog";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  const { id } = params;

  const response = await fetch(routes.students.single(id));

  const data: CompositeStudent = await response.json();

  const { student, tasks, lessons, decks } = data;

  const rendered = await parseMarkdown(student.markdown || "");

  return {
    student,
    studentTasks: tasks,
    studentLessons: lessons,
    studentDecks: decks,
    rendered,
  };
};
