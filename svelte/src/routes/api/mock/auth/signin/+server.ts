import logger from "$lib/logger";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
type POSTRequestBody = any;

export const POST: RequestHandler = async ({ request, params, url }) => {
  const body = await request.json();
  logger.info("POST /api/v1/auth/signin with body:", body);
  // signin

  return json({
    accessToken: {
      token: "access token",
      expiresAt: 1756710415,
    },
    refreshToken: {
      token: "refresh token",
      expiresAt: 1759301515,
    },
  });
};
