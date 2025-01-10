import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    // First attempt with existing token
    const response = await resolve(event);
    
    // If we hit 401 and have a refresh token cookie, let's try to refresh
    if (response.status === 401) {
        try {

            console.log('Refreshing token...');
            // Hit your refresh endpoint
            const refreshRes = await event.fetch(`/axum/auth/refresh`);

           // if (!refreshRes.ok) {
                
           //     event.cookies.delete('refreshToken', {path: "/"});
           //     return response;
//            }

            // Extract and parse the new token
            const { accessToken } = await refreshRes.json();
            
            console.log('Got new token:', accessToken);
            // Set it in locals for your handleFetch to use
            event.locals.accessToken = accessToken;

            event.request.headers.set("Authorization", `Bearer ${accessToken || ''}`);
            
            // Retry the original request
            return await resolve(event);
        } catch (error) {
            console.error('Refresh token flow failed:', error);
            return response;
        }
    }

    return response;
};
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	console.log('Got request FETCH', request.headers);
    request.headers.set("X-API-KEY", env.API_KEY_AXUM);
    request.headers.set("Content-Type", "application/json");
    if (event.locals.accessToken) {
        request.headers.set("Authorization", `Bearer ${event.locals.accessToken || ''}`);
    }
    // request.headers.set('cookie', event.request.headers.get("cookie") || '');
    
    console.log('Sending request FETCH', request.headers);
	return fetch(request);
};