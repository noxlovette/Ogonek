import logger from "$lib/logger";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async () => {
  const tasks: TaskSmall[] = [
    {
      id: "task1",
      title: "Murrrrrr",
      priority: 3,
      completed: false,
      assigneeName: "Kotya",
      seen: true,
      dueDate: "2025-07-15T23:59:59Z",
    },
    {
      id: "task2",
      title: "Kotya Assigned",
      priority: 2,
      completed: false,
      assigneeName: "Kotya",
      seen: true,
      dueDate: "2025-07-17T23:59:59Z",
    },
  ];

  const paginatedResponse: PaginatedResponse<TaskSmall> = {
    data: tasks,
    perPage: 3,
    total: 10,
    page: 1,
  };

  return new Response(JSON.stringify(paginatedResponse), {
    headers: { "Content-Type": "application/json" },
  });
};

export const POST: RequestHandler = async ({ request }) => {
  const body = await request.json();

  logger.warn("HIT POST TASK with body:", body);

  const id = "task1";

  return new Response(JSON.stringify(id), {
    headers: { "Content-Type": "application/json" },
  });
};
