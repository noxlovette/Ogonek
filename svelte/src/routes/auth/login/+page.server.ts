import { env } from "$env/dynamic/public";
import logger from "$lib/logger";
import {
  handleApiResponse,
  isSuccessResponse,
  setTokenCookie,
  turnstileVerify,
  ValidateAccess,
} from "$lib/server";
import type { AuthResponse } from "$lib/types";
import { validateRequired } from "@noxlovette/svarog";
import { fail, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
  default: async ({ request, fetch, cookies, locals }) => {
    try {
      const data = await request.formData();
      const username = data.get("username") as string;
      const pass = data.get("password") as string;
      const validateUsername = validateRequired("username");
      const validatePass = validateRequired("password");

      const usernameError = validateUsername(username);
      const passError = validatePass(pass);

      if (usernameError) {
        return fail(400, {
          message: usernameError,
        });
      }

      if (passError) {
        return fail(400, {
          message: passError,
        });
      }

      if (env.PUBLIC_APP_ENV !== "development") {
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
      }

      const response = await fetch("/axum/auth/signin", {
        method: "POST",
        body: JSON.stringify({ username, pass }),
      });

      const authResult = await handleApiResponse<AuthResponse>(response);
      if (!isSuccessResponse(authResult)) {
        return fail(authResult.status, { message: authResult.message });
      }

      const { accessToken, refreshToken } = authResult.data;
      setTokenCookie(cookies, "accessToken", accessToken);
      setTokenCookie(cookies, "refreshToken", refreshToken);

      const user = await ValidateAccess(accessToken.token);

      logger.debug({ user }, "user from the login");

      return {
        user,
        success: true,
      };
    } catch (error) {
      logger.error("Signin error:", error);
      return fail(500, {
        message:
          error instanceof Error ? error.message : "Internal server error",
      });
    }
  },
};
