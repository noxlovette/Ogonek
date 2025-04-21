// hooks.client.ts
import { env } from "$env/dynamic/public";
import * as Sentry from "@sentry/sveltekit";

if (env.PUBLIC_APP_ENV !== "development") {
  Sentry.init({
    dsn: "https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808",
    environment: env.PUBLIC_APP_ENV || "development",
    tracesSampleRate: 1,
  });
}

export const handleError = Sentry.handleErrorWithSentry();
