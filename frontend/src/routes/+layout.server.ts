import type { LayoutServerLoad } from "./$types";
const VITE_API_URL = import.meta.env.VITE_API_URL || "http://backend-firelight:8000/";

export const load: LayoutServerLoad = async ({ cookies, fetch, depends }) => {
  const sessionid = cookies.get("sessionid");
  const endpoint = `${VITE_API_URL}/api/check-session/`;

  const response = await fetch(endpoint, {
    method: "GET",
    headers: {
      Cookie: `sessionid=${sessionid}`,
      'X-CSRFToken': cookies.get('csrftoken') || ''
    },
  });

  if (response.status === 401) {
    return { user: null };
  } else if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  if (response.headers.get('content-type')?.includes('application/json')) {
    let user = await response.json() as App.User;

    console.log("User is authenticated", user);

    cookies.set("csrftoken", user.csrfToken, { path: "/" });
    depends("app:user:login");
    return { user };
  } else {
    console.error("Response was not JSON from", endpoint);
    return { user: null };
  }
};