import { version } from "$app/environment";
import { env } from "$env/dynamic/private";
import logger from "$lib/logger";
import redis from "$lib/redisClient";
import { ValidateAccess, setTokenCookie } from "$lib/server";
import type { RefreshResponse } from "$lib/types";
import * as Sentry from "@sentry/sveltekit";
import type {
  Handle,
  HandleFetch,
  RequestEvent,
  ServerInit,
} from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

Sentry.init({
  dsn: env.PUBLIC_SENTRY_DSN,
  release: version,
  environment: env.PUBLIC_APP_ENV || "development",
  tracesSampleRate: env.PUBLIC_SENTRY_TRACING_RATE
    ? Number(env.PUBLIC_SENTRY_TRACING_RATE)
    : 1.0,
});

const PROTECTED_PATHS = new Set(["/t/", "/s/", "/auth/bind/"]);

function isProtectedPath(path: string): boolean {
  return (
    PROTECTED_PATHS.has(path) ||
    Array.from(PROTECTED_PATHS).some((prefix) => path.startsWith(prefix))
  );
}
export const init: ServerInit = async () => {
  logger.info("App Booted");
};

export const handle: Handle = sequence(
  Sentry.sentryHandle(),
  async ({ event, resolve }) => {
    const path = event.url.pathname;
    const role = event.params.role;

    if (path === "/") {
      const user = await getUserFromToken(event);
      if (user) {
        if (user.role === "student") {
          throw redirect(303, "/s/dashboard");
        } else if (user.role === "teacher") {
          throw redirect(303, "/t/dashboard");
        }
      }
    }

    if (!isProtectedPath(path)) {
      return resolve(event);
    }

    const user = await getUserFromToken(event);
    if (!user) {
      throw redirect(302, "/auth/login");
    }

    const isTeacherRoute = role === "t";
    const isStudentRoute = role === "s";

    if (isTeacherRoute && user.role !== "teacher") {
      logger.warn("Redirecting to unauthorised as student");
      throw redirect(303, "/unauthorised");
    }
    if (isStudentRoute && user.role !== "student") {
      logger.warn("Redirecting to unauthorised as teacher");
      throw redirect(303, "/unauthorised");
    }

    const response = await resolve(event);
    return response;
  },
);

async function handleTokenRefresh(event: RequestEvent) {
  const refreshToken = event.cookies.get("refreshToken");
  if (!refreshToken) {
    logger.warn("Redirecting unauthorised user");
    throw redirect(302, "/auth/login");
  }

  const refreshCacheKey = `refresh:${refreshToken}`;

  const inProgress = await redis.get(refreshCacheKey);

  if (inProgress === "true") {
    for (let i = 0; i < 10; i++) {
      await new Promise((resolve) => setTimeout(resolve, 200));

      const cachedUser = await redis.get(`${refreshCacheKey}:result`);
      if (cachedUser) {
        return JSON.parse(cachedUser);
      }
    }

    // If we waited too long, assume the other process failed and try again
    await redis.del(refreshCacheKey);
  }

  // Mark refresh as in progress with expiration (5 seconds)
  await redis.set(refreshCacheKey, "true", "EX", 5);

  try {
    const refreshRes = await event.fetch("/auth/refresh", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
    });
    if (!refreshRes.ok) {
      logger.error("Refresh failed after sending request to Svelte backend");
      throw new Error("Refresh failed");
    }
    const { accessToken } = (await refreshRes.json()) as RefreshResponse;
    if (!accessToken) {
      logger.error("No access token in refresh response");
      throw new Error("No access token in refresh response");
    }
    setTokenCookie(event.cookies, "accessToken", accessToken);

    const user = await ValidateAccess(accessToken.token);

    await redis.set(`${refreshCacheKey}:result`, JSON.stringify(user), "EX", 3);

    return user;
  } catch (error) {
    logger.error("Token refresh failed:", error);
    throw redirect(302, "/auth/login");
  } finally {
    // Clean up
    await redis.del(refreshCacheKey);
  }
}

async function getUserFromToken(event: RequestEvent) {
  const accessToken = event.cookies.get("accessToken");
  let user = null;

  if (accessToken) {
    try {
      user = await ValidateAccess(accessToken);
    } catch (error) {
      logger.debug({ error }, "Attempting to refresh user from token");
      user = await handleTokenRefresh(event);
    }
  } else if (event.cookies.get("refreshToken")) {
    user = await handleTokenRefresh(event);
  }

  return user;
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  const url = new URL(request.url);

  if (url.pathname.startsWith("/axum/")) {
    const cleanPath = url.pathname.replace("/axum/", "/");
    // Create new URL with the base and path
    const newUrl = new URL(cleanPath, env.BACKEND_URL);

    // IMPORTANT: Copy all search parameters from original URL
    url.searchParams.forEach((value, key) => {
      newUrl.searchParams.set(key, value);
    });

    // Create new request with the full URL including query parameters
    request = new Request(newUrl, request);
  }
  request.headers.set("X-API-KEY", env.API_KEY_AXUM);
  // Only set Content-Type for non-FormData requests
  if (!request.headers.get("Content-Type")?.includes("multipart/form-data")) {
    request.headers.set("Content-Type", "application/json");
  }
  const accessToken = event.cookies.get("accessToken");
  if (accessToken) {
    request.headers.set("Authorization", `Bearer ${accessToken}`);
  }
  return fetch(request);
};
export const handleError = Sentry.handleErrorWithSentry();
