import { dev } from "$app/environment";
import { env } from "$env/dynamic/public";
import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { captchaVerify, setTokenCookie, ValidateAccess } from "$lib/server";
import { createUser } from "$lib/server/mock/user";
import type { User } from "$lib/types";
import { validateForm } from "$lib/utils";
import { fail } from "@sveltejs/kit";
import type { JWTPayload } from "jose";
import type { Actions } from "./$types";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();
    const validation = validateForm(formData, z.signinBody);
    if (!validation.success) {
      const fieldErrors: Record<string, boolean> = {};
      validation.errors.issues.forEach((issue) => {
        const fieldPath = issue.path[0] as string;
        fieldErrors[fieldPath] = true;
      });

      return fail(400, {
        ...fieldErrors,
        message: "Validation failed",
      });
    }

    if (!dev) {
      const captchaToken = formData.get("smart-token") as string;

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
    }
    const { username, pass } = validation.data;
    const response = await fetch(routes.auth.signin(), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, pass }),
    });

    const data = await response.json();
    const parsed = z.signinResponse.safeParse(data);

    if (!parsed.success) {
      return fail(400);
    }

    let user: JWTPayload | User;
    if (!env.PUBLIC_MOCK_MODE) {
      const { accessToken, refreshToken } = parsed.data;
      setTokenCookie(cookies, "accessToken", accessToken);
      setTokenCookie(cookies, "refreshToken", refreshToken);
      user = await ValidateAccess(accessToken.token);
      logger.debug({ user }, "user from the login");
    } else {
      user = createUser();
    }

    return {
      user,
      success: true,
    };
  },
};
