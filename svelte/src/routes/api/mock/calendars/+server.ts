import { createCalendar } from "$lib/server/mock/calendars";
import logger from "$lib/logger";
import { faker } from "@faker-js/faker";
import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const GET: RequestHandler = async ({}) => {
  // Get all calendars for the authenticated user
  const calendars = faker.helpers.multiple(createCalendar, { count: { min: 2, max: 5 } });
  return json(calendars);
};

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/calendars with body:", body);
  // Create a new calendar
  const newCalendar = createCalendar();
  return json(newCalendar);
};