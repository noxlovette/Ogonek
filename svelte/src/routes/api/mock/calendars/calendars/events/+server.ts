import { json } from "@sveltejs/kit";
import { nanoid } from "nanoid";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async () => {
  return json({ id: nanoid() });
};
