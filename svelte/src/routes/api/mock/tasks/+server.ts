import { createTasksSmall } from "$lib/server/mock/tasks";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import { json, type RequestHandler } from "@sveltejs/kit";
import { nanoid } from "nanoid";

export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<TaskSmall> = {
    data: createTasksSmall(5),
    perPage: 3,
    page: 1,
  };

  return json(paginatedResponse);
};

export const POST: RequestHandler = async ({ request }) => {
  const body = await request.json();

  const id = nanoid();

  return json(id);
};
