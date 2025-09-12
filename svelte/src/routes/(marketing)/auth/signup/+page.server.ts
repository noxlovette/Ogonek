import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import {
  captchaVerify,
  handleApiResponse,
  isSuccessResponse,
} from "$lib/server";
import { validateForm } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({ request, url, fetch }) => {
    const formData = await request.formData();

    const validation = validateForm(formData, z.signupBody);
    if (!validation.success) {
      const fieldErrors: Record<string, boolean> = {};
      validation.errors.issues.forEach((issue) => {
        const fieldPath = issue.path[0];
        if (typeof fieldPath === "string") {
          fieldErrors[fieldPath] = true;
        }
      });
      return fail(400, {
        ...fieldErrors,
        message: "Validation failed",
      });
    }

    const { username, pass, email, role, name } = validation.data;

    const inviteToken = url.searchParams.get("invite");

    const captchaToken = formData.get("smart-token");
    if (!captchaToken || typeof captchaToken !== "string") {
      return fail(400, {
        captcha: true,
        name: false,
        username: false,
        email: false,
        pass: false,
        message: "Please complete the CAPTCHA verification",
      });
    }

    let captchaResponse;
    try {
      captchaResponse = await captchaVerify(captchaToken);
    } catch (error) {
      logger.error({ error }, "CAPTCHA verification network error");
      return fail(500, {
        captcha: true,
        name: false,
        username: false,
        email: false,
        pass: false,
        message: "Verification service unavailable",
      });
    }

    if (!captchaResponse.ok) {
      return fail(400, {
        captcha: true,
        name: false,
        username: false,
        email: false,
        pass: false,
        message: "Captcha verification failed",
      });
    }

    const response = await fetch(routes.auth.signup(), {
      method: "POST",
      body: JSON.stringify({ username, pass, email, role, name }),
    });

    const result = await handleApiResponse<string>(response);
    if (!isSuccessResponse(result)) {
      if (result.status === 409) {
        return fail(409, {
          message: "Username or email already exists",
          username: true,
          email: true,
          pass: false,
          name: false,
          captcha: false,
        });
      }
      if (result.status === 422) {
        return fail(422, {
          message: "Invalid registration data",
          username: false,
          email: false,
          pass: false,
          name: false,
          captcha: false,
        });
      }
      logger.error({ result }, "Signup failed");
      return fail(400, {
        message: result.message,
        username: false,
        email: false,
        pass: false,
        name: false,
        captcha: false,
      });
    }

    if (inviteToken && typeof inviteToken === "string") {
      const studentId = result.data;

      const inviteResponse = await fetch("/axum/auth/bind", {
        method: "POST",
        body: JSON.stringify({ inviteToken, studentId }),
      });

      const inviteResult = await handleApiResponse(inviteResponse);
      if (!isSuccessResponse(inviteResult)) {
        logger.warn(
          {
            inviteToken: inviteToken.slice(0, 8) + "...",
            studentId,
            error: inviteResult,
          },
          "Invite binding failed - account created but not linked",
        );

        return redirect(302, "/auth/login?invite_failed=true");
      }

      logger.info({ studentId }, "Successfully bound invite to new account");
    }

    logger.info({ username }, "Successful user registration");
    return {
      success: true,
      name: false,
      username: false,
      pass: false,
      email: false,
      captcha: false,
    };
  },
} satisfies Actions;
