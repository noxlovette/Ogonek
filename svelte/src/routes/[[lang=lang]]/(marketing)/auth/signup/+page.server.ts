import logger from "$lib/logger";
import { routes } from "$lib/routes";
import {
  captchaVerify,
  handleApiResponse,
  isSuccessResponse,
} from "$lib/server";
import {
  validateEmail,
  validatePassword,
  validatePasswordMatch,
  validateUsername,
} from "@noxlovette/svarog";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";
export const actions: Actions = {
  default: async ({ request, url, fetch }) => {
    const data = await request.formData();
    const username = data.get("username") as string;
    const pass = data.get("password") as string;
    const confirmPassword = data.get("confirmPassword") as string;
    const email = data.get("email") as string;
    const role = data.get("role") as string;
    const name = data.get("name") as string;
    const inviteToken = url.searchParams.get("invite");

    if (validateEmail(email)) {
      return fail(400, { message: "Invalid Email" });
    }

    const usernameValidation = validateUsername(username);
    const passValidation = validatePassword(pass);
    const passMatchValidation = validatePasswordMatch(pass, confirmPassword);

    if (usernameValidation) {
      return fail(400, { message: usernameValidation });
    }

    if (pass !== confirmPassword) {
      return fail(400, { message: "Passwords do not match" });
    }

    if (passValidation) {
      return fail(400, { message: passValidation });
    }

    if (passMatchValidation) {
      return fail(400, { message: passMatchValidation });
    }

    const captchaToken = data.get("smart-token") as string;
    if (!captchaToken) {
      return fail(400, {
        message: "Please complete the CAPTCHA verification",
      });
    }
    const captchaResponse = await captchaVerify(captchaToken);
    if (!captchaResponse.ok) {
      return fail(400, {
        message: "Captcha verification failed",
      });
    }

    // Signup API call
    const response = await fetch(routes.auth.signup(), {
      method: "POST",
      body: JSON.stringify({ username, pass, email, role, name }),
    });

    const result = await handleApiResponse<string>(response);

    if (!isSuccessResponse(result)) {
      return fail(result.status, { message: result.message });
    }

    if (inviteToken) {
      const studentId = result.data;
      logger.debug(studentId);

      const inviteResponse = await fetch("/axum/auth/bind", {
        method: "POST",
        body: JSON.stringify({ inviteToken, studentId }),
      });

      const inviteResult = await handleApiResponse(inviteResponse);

      if (!isSuccessResponse(inviteResult)) {
        return fail(inviteResult.status, { message: inviteResult.message });
      }
    }

    return redirect(302, "/auth/login");
  },
} satisfies Actions;
