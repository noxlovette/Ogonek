import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch }) => {
  try {
    const payload = await request.json();
    console.log("hit complete");
    const response = await fetch("/axum/s3/multipart/complete", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const error = await response.text();
      console.error("Error completing multipart upload:", error);
      return new Response(error, { status: response.status });
    }

    return new Response("Success", { status: 200 });
  } catch (error) {
    console.error("Error in complete multipart upload:", error);
    return new Response("Internal server error", { status: 500 });
  }
};
