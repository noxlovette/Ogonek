import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from "../$types";


const VITE_API_URL = "http://backend:8000";

export const load: LayoutServerLoad = async ({ fetch, cookies, url, depends }) => {
  const sessionid = cookies.get('sessionid');
  const csrfToken = cookies.get('csrftoken') || '';

  // Check session first
  const sessionCheckResponse = await fetch(`${VITE_API_URL}/api/check-session/`, {
    method: "GET",
    headers: {
      'Cookie': `sessionid=${sessionid}; csrftoken=${csrfToken}`
    },
  });

  if (sessionCheckResponse.status === 401) {
    // Redirect to login if unauthorized, but only if not already on the login page
    const redirectTo = url.pathname;
    throw redirect(302, `/login?redirectTo=${encodeURIComponent(redirectTo)}`);
  } else if (!sessionCheckResponse.ok) {
    throw new Error(`HTTP error during session check! status: ${sessionCheckResponse.status}`);
  }

  let user = await sessionCheckResponse.json() as App.User;

    cookies.set("csrftoken", user.csrfToken, { path: "/" });    // Update CSRF token

  // If session is valid, proceed to fetch data for authenticated users
  const headers = {
    'Cookie': `sessionid=${sessionid}; csrftoken=${csrfToken}`
  };

  const [tasks, lessons] = await Promise.all([
    fetch(`${VITE_API_URL}/api/tasks/`, {
      headers: headers
    }).then((res) => res.json()),
    fetch(`${VITE_API_URL}/api/lessons/`, {
      headers: headers
    }).then((res) => res.json()),
  ]);
  depends("app:user:login");
  return { tasks, lessons, user };
};