// hooks.client.ts
import { dev, version } from "$app/environment";
import { env } from "$env/dynamic/public";
import logger from "$lib/logger";
import * as Sentry from "@sentry/sveltekit";
import type { HandleClientError } from "@sveltejs/kit";

Sentry.init({
  dsn: env.PUBLIC_SENTRY_DSN,
  release: version,
  environment: dev ? "development" : "production",
  tracesSampleRate: Number(env.PUBLIC_SENTRY_TRACING_RATE) || 1,
  integrations: [Sentry.browserTracingIntegration()],

  tracePropagationTargets: ["localhost", /^https:\/\/ogonek\.app/],
});

export const handleError: HandleClientError = async ({
  error,
  event,
  status,
  message,
}) => {
  const errorID = crypto.randomUUID();

  logger.error({ message });

  Sentry.captureException(error, {
    extra: { event, errorID, status },
  });

  return {
    message: "Whoops!",
    errorID,
  };
};
