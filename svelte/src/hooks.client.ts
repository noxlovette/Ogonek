import { setProfile, setUser } from "$lib/stores";
import * as Sentry from "@sentry/sveltekit";
import type { ClientInit } from "@sveltejs/kit";

// If you don't want to use Session Replay, remove the `Replay` integration,
// `replaysSessionSampleRate` and `replaysOnErrorSampleRate` options.
Sentry.init({
  dsn: "https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808",
  // environment: "development", // TODO MAKE FUCKING DYNAMIC
  tracesSampleRate: 1,
  replaysSessionSampleRate: 0.1,
  replaysOnErrorSampleRate: 1,
  integrations: [Sentry.replayIntegration()],
});

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
