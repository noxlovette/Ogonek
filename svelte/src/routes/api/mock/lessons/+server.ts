import { createLessonsSmall } from "$lib/server/mock/lessons";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<LessonSmall> = {
    data: createLessonsSmall(3),
    perPage: 10,
    page: 1,
  };

  return json(paginatedResponse);
};
export const POST: RequestHandler = async () => {
  const id = nanoid();
  return json(id);
};
