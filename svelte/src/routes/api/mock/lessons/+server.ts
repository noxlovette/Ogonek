import { createLessonSmall } from "$lib/server/mock";
import type { LessonSmall, PaginatedResponse } from "$lib/types";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async () => {
  const paginatedResponse: PaginatedResponse<LessonSmall> = {
    data: faker.helpers.multiple(createLessonSmall),
    perPage: 3,
    page: 1,
  };

  return json(paginatedResponse);
};
export const POST: RequestHandler = async () => {
  const id = nanoid();
  return json(id);
};
