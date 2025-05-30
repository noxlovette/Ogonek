import logger from "$lib/logger";
import { notifyTelegram } from "$lib/server";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, fetch, url }) => {
  try {
    const taskId = url.searchParams.get("taskId");
    const shouldNotify = url.searchParams.get("notify") === "true";
    const teacherTelegramId = url.searchParams.get("teacherTelegramId");
    const payload = await request.json();

    logger.debug(
      `Processing multipart completion with taskId: ${taskId}, notify: ${shouldNotify}`,
    );

    const response = await fetch("/axum/s3/multipart/complete", {
      method: "POST",
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      const error = await response.text();
      logger.error("Error completing multipart upload:", error);
      return new Response(error, { status: response.status });
    }

    // Only send notification if both conditions are met:
    // 1. shouldNotify flag is true
    // 2. taskId is present
    // 3. telegramId is present
    if (shouldNotify && taskId && teacherTelegramId) {
      const message = `A task has been completed\\. Check the student's homework on [Ogonek](https://Ogonek\\.app/t/tasks/t/${taskId})\\.`;

      try {
        const telegramResponse = await notifyTelegram(
          message,
          teacherTelegramId,
        );

        // Log notification status but don't fail the request if notification fails
        if (telegramResponse.status !== 200) {
          logger.warn(
            `Telegram notification failed with status: ${telegramResponse.status}`,
          );
        } else {
          logger.info("Telegram notification sent successfully");
        }
      } catch (notifyError) {
        logger.error("Failed to send Telegram notification:", notifyError);
        // Don't fail the main request just because notification failed
      }
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
