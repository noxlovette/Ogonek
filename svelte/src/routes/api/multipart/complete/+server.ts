import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch, url }) => {
  const taskId = url.searchParams.get("taskId");
  const payload = await request.json();

  logger.debug(`Processing multipart completion with taskId: ${taskId}`);

  const response = await fetch(routes.files.multipart.complete(), {
    method: "POST",
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    const error = (await response.text()) as any;
    logger.error("Error completing multipart upload:", error);
    return new Response(error, { status: response.status });
  }

  return new Response(JSON.stringify({ success: true }), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
};
