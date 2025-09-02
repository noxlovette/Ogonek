import type { Cookies } from "@sveltejs/kit";
import logger from "../logger";

export function setTokenCookie(
  cookies: Cookies,
  name: string,
  token: { token: string; expiresAt: number },
) {
  logger.debug("setting up token as cookie");
  const expiry = new Date(token.expiresAt * 1000);
  const maxAge = Math.floor((expiry.getTime() - Date.now()) / 1000);

  cookies.set(name, token.token, {
    path: "/",
    maxAge,
  });

  logger.debug("set up token as cookie");
}
