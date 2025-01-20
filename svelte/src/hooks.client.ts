import * as Sentry from '@sentry/sveltekit';
import type { ClientInit } from '@sveltejs/kit';
import { setProfile, setUser } from '$lib/stores';

if (!import.meta.env.DEV) {
	Sentry.init({
		dsn: 'https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808',
		tracesSampleRate: 1,
		replaysSessionSampleRate: 0.1,
		replaysOnErrorSampleRate: 1,
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
