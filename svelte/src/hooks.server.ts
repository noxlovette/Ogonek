import {sequence} from '@sveltejs/kit/hooks';
import * as Sentry from '@sentry/sveltekit';
import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { ValidateAccess } from '$lib/server';
import type { RequestEvent } from '@sveltejs/kit';

Sentry.init({
    dsn: "https://2d5f51ef45d12264bf0a264dbbbeeacb@o4507272574468096.ingest.de.sentry.io/4507947592777808",
	 environment: "development", // TODO MAKE FUCKING DYNAMIC
    tracesSampleRate: 1
})

let isRefreshing = false;

const PROTECTED_PATHS = new Set(['/t/', '/s/', '/download/']);

function isProtectedPath(path: string): boolean {
				return (
								PROTECTED_PATHS.has(path) ||
								Array.from(PROTECTED_PATHS).some((prefix) => path.startsWith(prefix))
				);
}

export const handle: Handle = sequence(Sentry.sentryHandle(), async ({ event, resolve }) => {
	const path = event.url.pathname;

	if (!isProtectedPath(path)) {
		return resolve(event);
	}

	const accessToken = event.cookies.get('accessToken');
	let user = null;

	// Try to get valid user - either from current token or refreshed token
	if (accessToken) {
		try {
			user = await ValidateAccess(accessToken);
		} catch (error) {
			user = await handleTokenRefresh(event);
		}
	} else if (event.cookies.get('refreshToken')) {
		user = await handleTokenRefresh(event);
	} else {
		throw redirect(302, '/auth/login');
	}

	// Now that we have a valid user, check role permissions
	if (user) {
		const isTeacherRoute = /\/t\//i.test(event.url.pathname);
		const isStudentRoute = /\/s\//i.test(event.url.pathname);

		if (isTeacherRoute && user.role !== 'teacher') {
			throw redirect(303, '/unauthorised');
		}
		if (isStudentRoute && user.role !== 'student') {
			throw redirect(303, '/unauthorised');
		}
	}

	const response = await resolve(event);
	return response;
});

async function handleTokenRefresh(event: RequestEvent) {
				if (isRefreshing) {
								while (isRefreshing) {
												await new Promise((resolve) => setTimeout(resolve, 100));
								}
								return null;
				}

				isRefreshing = true;
				const refreshToken = event.cookies.get('refreshToken');

				try {
								const refreshRes = await event.fetch('/auth/refresh', {
												headers: {
																Cookie: `refreshToken=${refreshToken}`,
																Accept: 'application/json'
												}
								});

								if (!refreshRes.ok) {
												throw new Error('Refresh failed');
								}

								// After successful refresh, validate the new token
								const newAccessToken = event.cookies.get('accessToken');
								if (newAccessToken) {
												return await ValidateAccess(newAccessToken);
								}
				} catch (error) {
								console.error('Refresh failed:', error);
								throw redirect(302, '/auth/login');
				} finally {
								isRefreshing = false;
				}
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
				const url = new URL(request.url);

				if (url.pathname.startsWith('/axum/')) {
								const cleanPath = url.pathname.replace('/axum/', '/');
								const newUrl = new URL(cleanPath, env.BACKEND_URL);
								request = new Request(newUrl, request);
				}

				if (url.pathname.startsWith('/file-server/')) {
								const cleanPath = url.pathname.replace('/file-server/', '/');
								const newUrl = new URL(cleanPath, env.UPLOAD_URL);
								request = new Request(newUrl, request);

								request.headers.append('X-API-KEY', env.API_KEY_FILE);
								return fetch(request);
				}

				request.headers.set('X-API-KEY', env.API_KEY_AXUM);
				request.headers.set('Content-Type', 'application/json');

				const accessToken = event.cookies.get('accessToken');
				if (accessToken) {
								request.headers.set('Authorization', `Bearer ${accessToken}`);
				}

				return fetch(request);
};
export const handleError = Sentry.handleErrorWithSentry();