import { dev } from "$app/environment";
import { env } from "$env/dynamic/public";
import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { captchaVerify, setTokenCookie, ValidateAccess } from "$lib/server";
import { createUser } from "$lib/server/mock/user";
import { type Claims, type User } from "$lib/types";
import { isSuperUser, validateForm } from "$lib/utils";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();

    const validation = validateForm(formData, z.signinBody);
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

    if (!dev) {
      const captchaToken = formData.get("smart-token");
      if (!captchaToken || typeof captchaToken !== "string") {
        return fail(400, {
          message: "Please complete the CAPTCHA verification",
        });
      }

      try {
        const captchaResponse: Response = await captchaVerify(captchaToken);
        if (!captchaResponse.ok) {
          return fail(400, {
            message: "Captcha verification failed",
          });
        }
      } catch (error) {
        logger.error({ error }, "CAPTCHA verification error");
        return fail(500, {
          message: "Verification service unavailable",
        });
      }
    }

    const { username, pass } = validation.data;

    const response = await fetch(routes.auth.signin(), {
      method: "POST",
      body: JSON.stringify({ username, pass }),
    });

    if (!response.ok) {
      if (response.status === 401) {
        return fail(401, {
          message: "Invalid credentials",
        });
      }
      if (response.status === 429) {
        return fail(429, {
          message: "Too many login attempts. Please try again later.",
        });
      }
      return fail(response.status, {
        message: "Authentication service error",
      });
    }

    let data: unknown;
    try {
      data = await response.json();
    } catch (error) {
      logger.error({ error }, "Failed to parse auth response JSON");
      return fail(500, {
        message: "Invalid server response",
      });
    }

    const parsed = z.signinResponse.safeParse(data);
    if (!parsed.success) {
      logger.error(
        {
          parseErrors: parsed.error,
          responseData: data,
        },
        "Auth response validation failed",
      );
      return fail(500, {
        message: "Invalid authentication response",
      });
    }

    let user: Claims | User;
    if (!env.PUBLIC_MOCK_MODE) {
      const { accessToken, refreshToken } = parsed.data;

      setTokenCookie(cookies, "accessToken", accessToken);
      setTokenCookie(cookies, "refreshToken", refreshToken);
      user = await ValidateAccess(accessToken.token);

      console.log(user);
      console.log(isSuperUser(user.role));
      if (isSuperUser(user.role)) {
        throw redirect(302, "/admin");
      } else if (user.role == "teacher") {
        throw redirect(302, "/t/dashboard");
      } else if (user.role == "student") {
        throw redirect(302, "/s/dashboard");
      }

      logger.debug({ userId: user?.id }, "Successful login");
    } else {
      user = createUser();
    }

    return {
      username: false,
      pass: false,
      success: true,
    };
  },
};
