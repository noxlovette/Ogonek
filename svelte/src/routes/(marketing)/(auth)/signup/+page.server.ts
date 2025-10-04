import { dev } from "$app/environment";
import { env } from "$env/dynamic/public";
import { z } from "$lib";
import logger from "$lib/logger";
import { m } from "$lib/paraglide/messages";
import { routes } from "$lib/routes";
import { captchaVerify } from "$lib/server";
import { validateForm } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async ({ request, url, fetch }) => {
    const formData = await request.formData();
    const passtoConfirm = formData.get("pass")?.toString();
    const confirmPassword = formData.get("confirmPassword")?.toString();

    if (passtoConfirm !== confirmPassword) {
      return fail(400, { confirmPassword: true });
    }
    const validation = validateForm(formData, z.signupBody);
    if (!validation.success) {
      const fieldErrors: Record<string, boolean> = {};
      validation.errors.issues.forEach((issue) => {
        const fieldPath = issue.path[0];
        if (typeof fieldPath === "string") {
          fieldErrors[fieldPath] = true;
        }
      });

      console.log(validation);
      return fail(400, {
        ...fieldErrors,
        message: "Validation failed",
      });
    }

    const { username, pass, email, role, name } = validation.data;

    const inviteToken = url.searchParams.get("invite");

    if (!dev && !env.PUBLIC_MOCK_MODE) {
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
          message: m.careful_misty_kestrel_renew(),
        });
      }

      if (!captchaResponse.ok) {
        return fail(400, {
          captcha: true,
          name: false,
          username: false,
          email: false,
          pass: false,
          message: m.careful_misty_kestrel_renew(),
        });
      }
    }
    const response = await fetch(routes.auth.signup(), {
      method: "POST",
      body: JSON.stringify({ username, pass, email, role, name }),
    });

    if (!response.ok) {
      const err = await response.text();
      if (response.status === 409) {
        return fail(409, {
          message: m.even_fair_barbel_spur(),
          username: true,
          email: true,
          pass: false,
          name: false,
          captcha: false,
        });
      }
      if (response.status === 422) {
        return fail(422, {
          message: "Invalid registration data",
          username: false,
          email: false,
          pass: false,
          name: false,
          captcha: false,
        });
      }
      logger.error({ err }, "Signup failed");
      return fail(400, {
        username: false,
        email: false,
        pass: false,
        name: false,
        captcha: false,
      });
    }

    if (inviteToken && typeof inviteToken === "string") {
      const studentId = await response.json();

      const inviteResponse = await fetch(
        routes.auth.bind_student_to_teacher(),
        {
          method: "POST",
          body: JSON.stringify({ inviteToken, studentId }),
        },
      );

      if (!inviteResponse.ok) {
        const err = await inviteResponse.text();
        logger.warn(
          {
            inviteToken: inviteToken.slice(0, 8) + "...",
            studentId,
            err,
          },
          "Invite binding failed - account created but not linked",
        );

        return redirect(302, "/login?invite_failed=true");
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
