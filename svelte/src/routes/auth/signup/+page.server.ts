import { turnstileVerify } from "$lib/server/turnstile";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export const actions: Actions = {
  default: async ({ request, url, fetch }) => {
    const data = await request.formData();
    const username = data.get("username") as string;
    const pass = data.get("password") as string;
    const confirmPassword = data.get("confirmPassword") as string;
    const email = data.get("email") as string;
    const role = data.get("role") as string;
    const name = data.get("name") as string;
    const invite_token = url.searchParams.get("invite");

    if (!isValidEmail(email)) {
      return fail(400, { message: "Invalid email address" });
    }

    if (name.length < 3 || name.length > 16) {
      return fail(400, {
        message: "Name should be between 3 and 16 characters",
      });
    }

    if (pass !== confirmPassword) {
      return fail(400, { message: "Passwords do not match" });
    }

    const turnstileToken = data.get("cf-turnstile-response") as string;
    if (!turnstileToken) {
      return fail(400, {
        message: "Please complete the CAPTCHA verification",
      });
    }

    const turnstileResponse = await turnstileVerify(turnstileToken);
    if (!turnstileResponse.ok) {
      return fail(400, {
        message: "Turnstile verification failed",
      });
    }

    const response = await fetch("/axum/auth/signup", {
      method: "POST",
      body: JSON.stringify({ username, pass, email, role, name }),
    });

    if (invite_token) {
      const student_id = await response.json().then((data) => data.id);
      const invite = await fetch("/axum/auth/bind", {
        method: "POST",
        body: JSON.stringify({ invite_token, student_id }),
      });

      if (!invite.ok) {
        return fail(400, { message: "Failed to bind invite" });
      }
    }

    if (response.ok) {
      return redirect(302, "/auth/login");
    }

    const { error } = await response.json();
    return fail(400, { message: error });
  },
} satisfies Actions;
