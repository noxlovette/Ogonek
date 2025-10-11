import { type Actions } from "@sveltejs/kit";

import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail } from "@sveltejs/kit";

export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();

    const email = formData.get("email")?.toString();

    const name = formData.get("name")?.toString();
    const username = formData.get("username")?.toString();

    if (!name) {
      return fail(400, { name: true });
    }

    if (name.length < 3) {
      return fail(400, { name: true });
    }

    // Validate name: only letters, spaces, hyphens, and apostrophes
    if (!/^[a-zA-Z\s'-]+$/.test(name)) {
      return fail(400, { name: true });
    }

    // Validate username: no spaces, only alphanumeric, underscores, and hyphens
    if (username) {
      if (username.length < 3) {
        return fail(400, { username: true });
      }
      if (!/^[a-zA-Z0-9_-]+$/.test(username)) {
        return fail(400, { username: true });
      }
    }
    const url = formData.get("videoCallUrl")?.toString();
    if (params.role === "t") {
      if (!url) {
        return fail(400, { url: true });
      }
      if (!/^https?:\/\//.test(url)) {
        return fail(400, { url: true });
      }
    }

    const validateEmail = (email?: string) => {
      if (!email) return false;
      return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email);
    };

    if (!validateEmail(email)) {
      return fail(400, { email: true });
    }

    const profileBody = {
      videoCallUrl: url,
      avatarUrl: formData.get("avatarUrl"),
      telegramId: formData.get("telegramId"),
    };

    const userBody = {
      email,
      username: formData.get("username"),
      name,
    };

    const [profileRes, userRes] = await Promise.all([
      fetch(routes.users.profile(), {
        method: "PATCH",
        body: JSON.stringify(profileBody),
      }),
      fetch(routes.users.me(), {
        method: "PATCH",
        body: JSON.stringify(userBody),
      }),
    ]);

    if (!profileRes.ok) {
      const err = profileRes.text();
      logger.error({ err });
      return fail(500);
    }

    if (!userRes.ok) {
      const err = userRes.text();
      logger.error({ err });
      return fail(500);
    }
    return {
      success: true,
      message: "Profile updated successfully",
    };
  },
} satisfies Actions;
