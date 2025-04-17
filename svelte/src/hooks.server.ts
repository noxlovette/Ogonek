import { env } from "$env/dynamic/private";
import redis from "$lib/redisClient";
import { ValidateAccess } from "$lib/server";
import * as Sentry from "@sentry/sveltekit";
import type {
  Handle,
  HandleFetch,
  RequestEvent,
  ServerInit,
} from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

if (env.PUBLIC_APP_ENV !== "development") {
  Sentry.init({
    dsn: "https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808",
    environment: env.PUBLIC_APP_ENV || "development",
    tracesSampleRate: 1,
  });
}

const PROTECTED_PATHS = new Set(["/t/", "/s/", "/download/"]);

function isProtectedPath(path: string): boolean {
  return (
    PROTECTED_PATHS.has(path) ||
    Array.from(PROTECTED_PATHS).some((prefix) => path.startsWith(prefix))
  );
}
export const init: ServerInit = async () => {};

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
      throw redirect(303, "/unauthorised");
    }
    if (isStudentRoute && user.role !== "student") {
      throw redirect(303, "/unauthorised");
    }

    const response = await resolve(event);
    return response;
  },
);

async function handleTokenRefresh(event: RequestEvent) {
  const refreshToken = event.cookies.get("refreshToken");
  if (!refreshToken) {
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
      headers: {
        Cookie: `refreshToken=${refreshToken}`,
        Accept: "application/json",
      },
    });

    if (!refreshRes.ok) {
      throw new Error("Refresh failed");
    }

    const newAccessToken = event.cookies.get("accessToken");
    if (newAccessToken) {
      const user = await ValidateAccess(newAccessToken);

      // Cache the result for other processes waiting
      await redis.set(
        `${refreshCacheKey}:result`,
        JSON.stringify(user),
        "EX",
        3,
      );

      return user;
    }
    throw new Error("No access token after refresh");
  } catch (error) {
    console.error("Token refresh failed:", error);
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
