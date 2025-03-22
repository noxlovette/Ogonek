// hooks.client.ts
import { env } from "$env/dynamic/public";
import { setProfile, setUser } from "$lib/stores";
import * as Sentry from "@sentry/sveltekit";
import type { ClientInit } from "@sveltejs/kit";

if (env.PUBLIC_APP_ENV !== "development") {
  Sentry.init({
    dsn: "https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808",
    environment: env.PUBLIC_APP_ENV || "development",
    tracesSampleRate: 1,
  });
}

export const init: ClientInit = async () => {
  const user = localStorage.getItem("user") || "";
  const profile = localStorage.getItem("profile") || "";

  if (user) {
    setUser(JSON.parse(user));
  }

  if (profile) {
    setProfile(JSON.parse(profile));
  }
};

export const handleError = Sentry.handleErrorWithSentry();
