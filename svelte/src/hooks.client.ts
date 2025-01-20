import type { ClientInit } from '@sveltejs/kit';
import { setProfile, setUser } from '$lib/stores';
import { dev } from '$app/environment';
import * as Sentry from "@sentry/svelte";

if (!dev) {
	Sentry.init({
	  dsn: 'https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808',
	  tracesSampleRate: 0.2,        // Lower for production
	  replaysSessionSampleRate: 0.05, // 5% of sessions
	  replaysOnErrorSampleRate: 1,    // Keep this at 1 for error tracking
	  integrations: [Sentry.replayIntegration()]
	});
}
export const init: ClientInit = async () => {
	const user = localStorage.getItem('user') || '';
	const profile = localStorage.getItem('profile') || '';
	
	if (user) {
		setUser(JSON.parse(user));
		setProfile(JSON.parse(profile));
	}
};


export const handleError = Sentry.handleErrorWithSentry();