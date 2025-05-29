import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch }) => {
  try {
    const payload = await request.json();

    const response = await fetch("/axum/s3/multipart/init", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const error = await response.text();
      logger.error("Error initializing multipart upload:", error);
      return new Response(error, { status: response.status });
    }

    const data = await response.json();
    logger.debug(data);
    return json(data);
  } catch (error) {
    logger.error("Error in init multipart upload:", error);
    return new Response("Internal server error", { status: 500 });
  }
};
