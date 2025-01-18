import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { ValidateAccess } from '$lib/utils';

let isRefreshing = false;

export const handle: Handle = async ({ event, resolve }) => {
    // Allow non-protected routes to pass through
    if (!(/\/[ts]\//i).test(event.url.pathname)) {
        return resolve(event);
    }

    const accessToken = event.cookies.get("accessToken");
    let user = null;

    // Try to get valid user - either from current token or refreshed token
    if (accessToken) {
        try {

            user = await ValidateAccess(accessToken);
        } catch (error) {
            user = await handleTokenRefresh(event);
        }
    } else if (event.cookies.get("refreshToken")) {
        user = await handleTokenRefresh(event);
    } else {

        throw redirect(302, '/auth/login');
    }

    // Now that we have a valid user, check role permissions
    if (user) {
        const isTeacherRoute = /\/t\//i.test(event.url.pathname);
        const isStudentRoute = /\/s\//i.test(event.url.pathname);

        if (isTeacherRoute && user.role !== 'teacher') {
            console.log("unauthorised teacher access attempt");
            throw redirect(303, '/unauthorised');
        }
        if (isStudentRoute && user.role !== 'student') {
            console.log("unauthorised student access attempt");
            throw redirect(303, '/unauthorised');
        }
    }

    const response = await resolve(event);
    return response;
};

// Helper function to handle token refresh
async function handleTokenRefresh(event) {
    if (isRefreshing) {
        // Wait for ongoing refresh to complete
        while (isRefreshing) {
            await new Promise(resolve => setTimeout(resolve, 100));
        }
        return null;
    }

    isRefreshing = true;
    const refreshToken = event.cookies.get("refreshToken");

    try {
        console.debug('attempting to refresh token');
        const refreshRes = await event.fetch("/auth/refresh", {
            headers: {
                'Cookie': `refreshToken=${refreshToken}`,
                'Accept': 'application/json'
            }
        });

        if (!refreshRes.ok) {
            throw new Error("Refresh failed");
        }

        console.log("Token refreshed successfully");

        // After successful refresh, validate the new token
        const newAccessToken = event.cookies.get("accessToken");
        if (newAccessToken) {
            return await ValidateAccess(newAccessToken);
        }
    } catch (error) {
        console.error("Refresh failed:", error);
        throw redirect(302, '/auth/login');
    } finally {
        isRefreshing = false;
    }
}

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
    const url = new URL(request.url);

    if (url.pathname.startsWith('/axum/')) {
        // Remove the /axum/ prefix and construct the new URL
        const cleanPath = url.pathname.replace('/axum/', '/');
        const newUrl = new URL(cleanPath, 'http://axum:3000');
        request = new Request(newUrl, request);
    }

    // Your existing header logic
    request.headers.set("X-API-KEY", env.API_KEY_AXUM);
    request.headers.set("Content-Type", "application/json");

    const accessToken = event.cookies.get("accessToken");
    if (accessToken) {
        request.headers.set("Authorization", `Bearer ${accessToken}`);
    }

    return fetch(request);
};