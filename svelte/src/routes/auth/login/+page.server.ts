import { fail, type Actions } from '@sveltejs/kit';
import { ValidateAccess } from '$lib/utils';

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const data = await request.formData();
    const username = data.get('username') as string;
    const pass = data.get('password') as string;

    try {
      const response = await fetch("/axum/auth/signin", {
        method: 'POST',
        body: JSON.stringify({
          username,
          pass,
        })
      });

      if (!response.ok) {
        const { error } = await response.json();
        console.log(error)
        return fail(400, { message: error })
      }

      response.headers.getSetCookie().forEach(cookie => {
        const [fullCookie, ...opts] = cookie.split(';');
        const [name, value] = fullCookie.split('=');

        // Create a more robust options parser
        const cookieOpts: Record<string, string | boolean> = {};
        opts.forEach(opt => {
          const [key, val] = opt.trim().split('=');
          // Normalize keys by removing hyphens and lowercasing
          cookieOpts[key.toLowerCase().replace(/-/g, '')] = val || true;
        });

        cookies.set(name, value, {
          path: cookieOpts.path as string || '/',
          httpOnly: 'httponly' in cookieOpts,
          secure: 'secure' in cookieOpts,
          sameSite: cookieOpts.samesite as 'lax' | 'strict' | 'none' || 'lax',
          domain: cookieOpts.domain as string,
          // Look for both max-age and maxage
          maxAge: cookieOpts.maxage ?
            parseInt(cookieOpts.maxage as string) :
            cookieOpts['max-age'] ?
              parseInt(cookieOpts['max-age'] as string) :
              undefined
        });
      });

      const { accessToken } = await response.json();
      const user = await ValidateAccess(accessToken);

      if (!user) {
        return fail(500, { message: "Invalid Token" })
      }

      const profile = await fetch('/axum/profile').then(res => res.json());

      return {
        success: true,
        message: 'Login successful',
        user,
        profile
      };

    } catch (error) {
      console.error('Signin error:', error);
      return fail(400, {
        message: error instanceof Error ? error.message : 'Login failed'
      });
    }
  }
};