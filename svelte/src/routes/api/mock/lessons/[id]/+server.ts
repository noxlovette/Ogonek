import logger from "$lib/logger";
import { createLessonFull } from "$lib/server/mock";
import { mockResponder } from "$lib/server/mock/helpers";
import type { LessonUpdateBody } from "$lib/types";
import { json, type RequestEvent } from "@sveltejs/kit";

export async function GET({ url }: RequestEvent) {
  return mockResponder(url, {
    200: () => createLessonFull(),
    401: () => null,
    404: () => null,
  });
}

export async function PATCH({ request, url }: RequestEvent) {
  const body: LessonUpdateBody = await request.json();

  logger.debug(body);

  return mockResponder(url, {
    200: () => json(null, { status: 200 }),
    401: () => null,
    404: () => null,
  });
}

export async function DELETE({ url }: RequestEvent) {
  return mockResponder(url, {
    204: () => null,
    401: () => null,
    404: () => null,
  });
}
