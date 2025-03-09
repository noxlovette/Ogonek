import { env } from "$env/dynamic/private";
import type { AuthResponse } from "$lib/types";
import { parseCookieOptions } from "@noxlovette/svarog";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ cookies, fetch }) => {
  const refreshToken = cookies.get("refreshToken");
  const response = await fetch("/axum/auth/refresh", {
    headers: {
      cookie: `refreshToken=${refreshToken}`,
      "X-API-KEY": env.API_KEY_AXUM,
    },
  });

  response.headers.getSetCookie().forEach((cookie) => {
    const [fullCookie, ...opts] = cookie.split(";");
    const [name, value] = fullCookie.split("=");
    const cookieOptions = parseCookieOptions(opts);
    cookies.set(name, value, cookieOptions);
  });

  const { accessToken } = (await response.json()) as AuthResponse;

  return json({ success: true, accessToken });
};
