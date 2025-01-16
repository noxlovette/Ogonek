import { env } from '$env/dynamic/private';
import type { Handle, HandleFetch } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { ValidateAccess } from '$lib/utils';

let isRefreshing = false;

export const handle: Handle = async ({ event, resolve }) => {
if (event.url.pathname.startsWith('/auth/')) {
    return resolve(event);
  }

    console.log('handle hook');
    console.log(event);
      const accessToken = event.cookies.get("accessToken");
      
      if (accessToken) {
        console.log('accessToken found');
          try {
              // Try to validate the access token
              console.log("Validating token");
              await ValidateAccess(accessToken);

          } catch (error) {
              // Token validation failed - need refresh
              if (!isRefreshing) {
                console.log("Token validation failed - need refresh");
                  const refreshToken = event.cookies.get("refreshToken");
                  if (!refreshToken) {
                      throw redirect(302, '/auth/login');
                  }

                  isRefreshing = true;
                  try {
                    console.debug('attempting to refresh token');
                      const refreshRes = await event.fetch("/auth/refresh", {
                          // Adding headers to ensure proper handling
                          headers: {
                              'Cookie': `refreshToken=${refreshToken}`,
                              'Accept': 'application/json'
                          }
                      });

                      if (!refreshRes.ok) {
                          throw new Error("Refresh failed");
                      }

              
                      console.log("Token refreshed successfully");
                  } catch (error) {
                      console.error("Refresh failed:", error);
                      throw redirect(302, '/auth/login');
                  } finally {
                      isRefreshing = false;
                  }
              }
          }
      } else if (event.cookies.get("refreshToken")){
          const refreshToken = event.cookies.get("refreshToken");
          console.log('refresh token found');
          if (!isRefreshing) {
              isRefreshing = true;
              try {
                console.debug('attempting to refresh token');
                  const refreshRes = await event.fetch("/auth/refresh", {
                      // Adding headers to ensure proper handling
                      headers: {
                          'Cookie': `refreshToken=${refreshToken}`,
                          'Accept': 'application/json'
                      }
                  });

                  if (!refreshRes.ok) {
                      throw new Error("Refresh failed");
                  }

                  console.log("Token refreshed successfully");
              } catch (error) {
                  console.error("Refresh failed:", error);
                  throw redirect(302, '/auth/login');
              } finally {
                  isRefreshing = false;
              }
          }
      
    } else {
        console.log('no access or refresh token found');
          throw redirect(302, '/auth/login');
      }

      // If we got here, either the token was valid or we successfully refreshed
      isRefreshing = false;
      const response = await resolve(event);
      return response;
};

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