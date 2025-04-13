import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch }) => {
  try {
    const payload = await request.json();
    console.log("hit abort");
    const response = await fetch("/axum/s3/multipart/abort", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const error = await response.text();
      console.error("Error aborting multipart upload:", error);
      return new Response(error, { status: response.status });
    }

    return new Response("Upload aborted successfully", { status: 200 });
  } catch (error) {
    console.error("Error in abort multipart upload:", error);
    return new Response("Internal server error", { status: 500 });
  }
};
