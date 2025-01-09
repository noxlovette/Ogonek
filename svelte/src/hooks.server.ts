import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    // console.log('Got request', event.request);
    // event.request.headers.append("X-API-KEY", env.API_KEY_AXUM);
    const response = await resolve(event);
    // console.log('Sent response', response);
    return response;
};

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	console.log('Got request', request);
    request.headers.set("X-API-KEY", env.API_KEY_AXUM);
    request.headers.set("Content-Type", "application/json");
    
    console.log('Sending request', request);
	return fetch(request);
};