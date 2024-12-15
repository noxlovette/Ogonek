import type { Actions } from './$types';
import { error, redirect, fail } from '@sveltejs/kit';

const DJANGO_URL = 'http://backend-firelight:8000';

export const actions: Actions = {
  login: async ({ request, cookies }) => {
    const data = await request.formData();
    const username = data.get('username');
    const password = data.get('password');

    try {
      const response = await fetch(`${DJANGO_URL}/api/login/`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
          username: username as string,
          password: password as string,
        }),
      });

      const result = await response.json();

      if (response.ok) {
        // Assuming your Django API returns some form of session token or user info
        // Here you might want to set a cookie or session
        cookies.set('session', JSON.stringify(result), { path: '/' });
        return { success: true, message: result.message };
      } else {
        return fail(400, { success: false, message: result.message || 'Login failed' });
      }
    } catch (err) {
      console.error('Login error:', err);
      return fail(500, { success: false, message: 'An error occurred while trying to log in.' });
    }
  }
};