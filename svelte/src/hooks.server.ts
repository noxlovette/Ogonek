import { version } from "$app/environment";
import { env } from "$env/dynamic/private";
import { env as envPublic } from "$env/dynamic/public";
import logger from "$lib/logger";
import { paraglideMiddleware } from "$lib/paraglide/server";
import { ValidateAccess, setTokenCookie } from "$lib/server";
import redis from "$lib/server/redisClient";
import type { RefreshResponse } from "$lib/types";
import * as Sentry from "@sentry/sveltekit";
import type {
  Handle,
  HandleFetch,
  HandleServerError,
  RequestEvent,
  ServerInit,
} from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

Sentry.init({
  dsn: envPublic.PUBLIC_SENTRY_DSN,
  release: version,
  environment: envPublic.PUBLIC_APP_ENV || "development",
  tracesSampleRate: envPublic.PUBLIC_SENTRY_TRACING_RATE
    ? Number(envPublic.PUBLIC_SENTRY_TRACING_RATE)
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

export const paraglideHandle: Handle = ({ event, resolve }) =>
  paraglideMiddleware(event.request, ({ request, locale }) => {
    event.request = request;

    return resolve(event, {
      transformPageChunk: ({ html }) =>
        html.replace("%paraglide.lang%", locale),
    });
  });

export const authenticationHandle: Handle = async ({ event, resolve }) => {
  // skip authentication if in dev mode
  if (envPublic.PUBLIC_MOCK_MODE) return resolve(event);
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
};

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

    await redis.del(refreshCacheKey);
  }

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
    await redis.del(refreshCacheKey);
  }
}

async function getUserFromToken(event: RequestEvent) {
  if (envPublic.PUBLIC_MOCK_MODE) {
    return {
      id: "dev-user",
      email: "dev@example.com",
      role: "teacher",
      name: "Dev Mode User",
    };
  }
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
    const cleanPath = url.pathname.replace("/axum/", "/api/v1/");

    if (envPublic.PUBLIC_MOCK_MODE) {
      const mockURL = `${env.ORIGIN}/api/mock${cleanPath}`;
      request = new Request(mockURL, request);

      return fetch(request);
    } else {
      const newUrl = new URL(cleanPath, env.BACKEND_URL);

      url.searchParams.forEach((value, key) => {
        newUrl.searchParams.set(key, value);
      });

      request = new Request(newUrl, request);
    }

    if (!request.headers.get("Content-Type")?.includes("multipart/form-data")) {
      request.headers.set("Content-Type", "application/json");
    }

    const accessToken = event.cookies.get("accessToken");
    if (accessToken) {
      request.headers.set("Authorization", `Bearer ${accessToken}`);
    }
  }

  return fetch(request);
};

export const handleError: HandleServerError = async ({
  error,
  event,
  status,
  message,
}) => {
  const errorID = crypto.randomUUID();

  const requestContext = {
    method: event.request.method,
    url: event.url.pathname + event.url.search,
    userAgent: event.request.headers.get("user-agent"),
    ip: event.getClientAddress(),
    timestamp: new Date().toISOString(),
    userId: event.locals.user?.id || "anonymous",
  };

  logger.error({
    errorID,
    message: error.message ?? message,
    status,
    request: requestContext,
  });

  Sentry.captureException(error, {
    tags: {
      errorID,
      endpoint: event.url.pathname,
      method: event.request.method,
    },
    extra: {
      ...requestContext,
      originalMessage: message,
      status,
    },
    user: {
      ip_address: event.getClientAddress(),
    },
  });

  return {
    message: "Something went sideways on our end",
    errorID,
  };
};

export const handle: Handle = sequence(
  Sentry.sentryHandle(),
  authenticationHandle,
);
