import { createTaskSmall } from "$lib/server/mock";
import type { PaginatedResponse, TaskSmall } from "$lib/types";
import { faker } from "@faker-js/faker";
import { json, type RequestHandler } from "@sveltejs/kit";
import { nanoid } from "nanoid";

export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<TaskSmall> = {
    data: faker.helpers.multiple(createTaskSmall),
    perPage: 3,
    page: 1,
  };

  return json(paginatedResponse);
};

export const POST: RequestHandler = async () => {
  const id = nanoid();

  return json(id);
};
