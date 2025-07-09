import logger from "$lib/logger";
import type { Lesson } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ params }) => {
  let lesson: Lesson | null = null;

  switch (params.id) {
    case "lesson1":
      lesson = {
        title: "Water the plants",
        topic: "Bonjour",
        assignee: "1",
        assigneeName: "Michael",
        markdown: "A task",
        createdAt: "2025-07-03T10:36:53.047197Z",
        updatedAt: "2025-07-03T14:06:58.038355Z",
        id: "lesson1",
      };
      break;

    case "lesson2":
      lesson = {
        title: "Water the plants",
        topic: "Bonjour",
        assignee: "3",
        assigneeName: "Alexander",
        markdown: "A task",
        createdAt: "2025-07-03T10:36:53.047197Z",
        updatedAt: "2025-07-03T14:06:58.038355Z",
        id: "lesson2",
      };
      break;
  }

  if (!lesson) {
    return new Response("Lesson Not Found", { status: 404 });
  }

  return new Response(JSON.stringify(lesson), {
    headers: { "Content-Type": "application/json" },
  });
};

export const POST: RequestHandler = async () => {
  logger.warn("HIT POST ENDPOINT");
  return new Response(JSON.stringify({ id: "lesson1" }));
};
