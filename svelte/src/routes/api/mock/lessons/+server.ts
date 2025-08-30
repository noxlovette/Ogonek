import { generateMockLessonsSmall } from "$lib/server/mock/lessons";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<LessonSmall> = {
    data: generateMockLessonsSmall(3),
    perPage: 10,
    page: 1,
  };

  return json(paginatedResponse);
};
export const POST: RequestHandler = async () => {
  return json({ id: "lesson1" });
};
