// hooks.client.ts
import { version } from "$app/environment";
import { env } from "$env/dynamic/public";
import * as Sentry from "@sentry/sveltekit";

Sentry.init({
  dsn: env.PUBLIC_SENTRY_DSN,
  release: version,
  environment: env.PUBLIC_APP_ENV || "development",
  tracesSampleRate: Number(env.PUBLIC_SENTRY_TRACING_RATE) || 1,
  integrations: [Sentry.browserTracingIntegration()],

  tracePropagationTargets: ["localhost", /^https:\/\/ogonek\.app/],
});
export const handleError = Sentry.handleErrorWithSentry();
