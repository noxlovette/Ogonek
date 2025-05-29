import logger from "$lib/logger";
import type { RefreshResponse } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ cookies, fetch }) => {
  const refreshToken = cookies.get("refresh_token");
  logger.debug(refreshToken);
  logger.debug("request has reached the server.ts");
  const response = await fetch("/axum/auth/refresh", {
    method: "POST",
    body: JSON.stringify({ refreshToken }),
  });
  logger.debug("response on the server received");
  logger.debug(response);
  const { accessToken } = (await response.json()) as RefreshResponse;

  return json({ success: true, accessToken });
};
