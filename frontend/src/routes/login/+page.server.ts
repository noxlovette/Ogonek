import type { Actions } from './$types';
import { error, redirect, fail } from '@sveltejs/kit';

const DJANGO_URL = 'http://backend-firelight:8000';

export const actions: Actions = {
  login: async ({ request, cookies, url }) => {
    const data = await request.formData();
    const csrfToken = cookies.get("csrftoken");
    const sessionid = cookies.get("sessionid");
    const username = data.get('username') as string;
    const password = data.get('password') as string;

      const response = await fetch(`${DJANGO_URL}/api/login/`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
          Cookie: `sessionid=${sessionid}; csrftoken=${csrfToken}`,
        },
        body: new URLSearchParams({
          username,
          password,
        }),
      });

      const result: App.ResponseLogin = await response.json();

      if (response.ok) {

        console.log(result)
        // Set the session cookie with the new sessionid
        cookies.set('sessionid', result.sessionid, { 
          path: '/', 
        });

        // Check for a redirect parameter from the URL
        const redirectTo = url.searchParams.get('redirectTo') || '/u/dashboard';
        
        // Redirect to the specified path or default to home
        throw redirect(302, redirectTo);
      } else {
        // Check for specific error messages from the server
        if (result.message?.toLowerCase().includes('password')) {
          return fail(400, { username, incorrect: true, message: result.message });
        } else {
          return fail(400, { success: false, message: result.message || 'Login failed' });
        }
      }
  }
};