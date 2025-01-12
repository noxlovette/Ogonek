import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    const path = event.url.pathname;
    if (path.startsWith('/auth/login') || path.startsWith('/public')) {
        console.log('Skipping auth check for', path);
        return await resolve(event);
    }

    const response = await resolve(event);
    if (response.status === 401 && event.cookies.get('refreshToken')) {
        try {
            const refreshRes = await event.fetch(`/axum/auth/refresh`);
            if (!refreshRes.ok) {
                throw new Error('Failed to refresh token');
            }

            // Get ALL the Set-Cookie headers (it's an array!)
            const setCookieHeaders = refreshRes.headers.getSetCookie();

            // Create a new response with the copied cookies
            const newResponse = new Response(response.body, response);
            setCookieHeaders.forEach(cookie => {
                newResponse.headers.append('set-cookie', cookie);
            });

            return newResponse;
        } catch (error) {
            console.error('Refresh token flow failed:', error);
            return await resolve(event);
        }
    } else if (response.status === 401) {
        return await resolve(event);
    }

    return response;
};
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
    console.log("locals", event.locals);
    request.headers.set("X-API-KEY", env.API_KEY_AXUM);
    request.headers.set("Content-Type", "application/json");
    const accessToken = event.cookies.get("accessToken");
    if (accessToken) {
        request.headers.set("Authorization", `Bearer ${accessToken}`);
    }
    // request.headers.set('cookie', event.request.headers.get("cookie") || '');

    console.log('Sending request FETCH', request.headers);
    return fetch(request);
};