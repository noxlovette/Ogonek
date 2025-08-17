import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch }) => {
  const payload = await request.json();

  const response = await fetch(routes.files.multipart.init(), {
    method: "POST",
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    const error = await response.text();
    logger.error("Error initializing multipart upload:", error);
    return new Response(error, { status: response.status });
  }

  const data = await response.json();
  return json(data);
};
