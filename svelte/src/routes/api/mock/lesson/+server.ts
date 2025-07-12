import logger from "$lib/logger";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const lessons: LessonSmall[] = [
    {
      id: "lesson1",
      title: "Murrrrrr",
      topic: "Hello there",
      assigneeName: "Kotya",
      createdAt: "2025-07-03T10:40:10.957506Z",
      seen: true,
    },
    {
      id: "lesson2",
      title: "Kotya Assigned",
      topic: "Hello there",
      assigneeName: "Kotya",
      createdAt: "2025-07-03T10:40:10.957506Z",
      seen: true,
    },
  ];

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
