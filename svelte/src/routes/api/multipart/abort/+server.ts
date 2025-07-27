import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch }) => {
  const payload = await request.json();
  logger.debug("hit abort");
  const response = await fetch(routes.s3.multipart.abort(), {
    method: "POST",
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    const error = await response.text();
    logger.error("Error aborting multipart upload:", error);
    return new Response(error, { status: response.status });
  }

  return new Response("Upload aborted successfully", { status: 200 });
};
