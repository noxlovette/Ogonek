// hooks.client.ts
import { env } from "$env/dynamic/public";
import * as Sentry from "@sentry/sveltekit";

if (env.PUBLIC_APP_ENV !== "development") {
  Sentry.init({
    dsn: env.PUBLIC_SENTRY_DSN,
    environment: env.PUBLIC_APP_ENV || "development",
    tracesSampleRate: 1,
  });
}

export const handleError = Sentry.handleErrorWithSentry();
