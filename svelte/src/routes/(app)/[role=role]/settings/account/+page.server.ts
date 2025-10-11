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

    if (name?.trim() == "" || !name) {
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
    if (!username || username.trim() == "") {
      return fail(400, { username: true });
    } else {
      if (username.length < 3) {
        return fail(400, { username: true });
      }
      if (!/^[a-zA-Z0-9_-]+$/.test(username)) {
        return fail(400, { username: true });
      }
    }

    const validateEmail = (email?: string) => {
      if (!email) return false;
      return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email);
    };

    if (!validateEmail(email)) {
      return fail(400, { email: true });
    }

    const body = {
      email,
      username: formData.get("username"),
      name,
    };

    const response = await fetch(routes.users.me(), {
      method: "PATCH",
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const err = response.text();
      logger.error({ err });
      if (response.status === 409) return fail(500, { conflict: true });
    }
    return {
      success: true,
    };
  },
  resendEmailVerification: async ({ fetch }) => {
    const response = await fetch(routes.auth.resend_verification(), {
      method: "POST",
    });

    if (!response.ok) {
      const err = await response.text();
      return fail(400, { verification: true });
    }

    return {
      success: true,
    };
  },
} satisfies Actions;
