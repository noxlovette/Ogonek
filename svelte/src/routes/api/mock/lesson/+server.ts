import logger from "$lib/logger";
import { lessons } from "$lib/server/mock";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<LessonSmall> = {
    data: lessons,
    perPage: 10,
    total: 10,
    page: 1,
  };

  return new Response(JSON.stringify(paginatedResponse), {
    headers: { "Content-Type": "application/json" },
  });
};
export const POST: RequestHandler = async ({ request }) => {
  const body = await request.json();

  logger.warn({ body }, "HIT POST LESSON with body:");

  const id = "lesson1";

  return new Response(JSON.stringify({ id }), {
    headers: { "Content-Type": "application/json" },
  });
};
