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
                return response;
            }

            // Extract and parse the new token
            const { accessToken } = await refreshRes.json();

            console.log('Got new token:', accessToken);
            event.locals.accessToken = accessToken;
            event.request.headers.set("Authorization", `Bearer ${accessToken || ''}`);

            // Retry the original request
            return await resolve(event);
        } catch (error) {
            console.error('Refresh token flow failed:', error);
            return new Response(null, {
                status: 303,
                headers: {
                    location: '/auth/login'
                }
            })
        }
    } else if (response.status === 401) {
        return new Response(null, {
            status: 303,
            headers: {
                location: '/auth/login'
            }
        })
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