import logger from "$lib/logger";
import { generateMockTasksSmall } from "$lib/server/mock/tasks";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<TaskSmall> = {
    data: generateMockTasksSmall(5),
    perPage: 3,
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
