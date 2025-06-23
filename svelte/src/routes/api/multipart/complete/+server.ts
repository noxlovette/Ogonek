import logger from "$lib/logger";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch, url }) => {
  try {
    const taskId = url.searchParams.get("taskId");
    const payload = await request.json();

    logger.debug(`Processing multipart completion with taskId: ${taskId}`);

    const response = await fetch("/axum/s3/multipart/complete", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const error = await response.text();
      logger.error("Error completing multipart upload:", error);
      return new Response(error, { status: response.status });
    }

    return new Response(JSON.stringify({ success: true }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    logger.error("Error in complete multipart upload:", error);
    return new Response(
      JSON.stringify({
        error: error instanceof Error ? error.message : "Unknown error",
      }),
      {
        status: 500,
        headers: { "Content-Type": "application/json" },
      },
    );
  }
};
