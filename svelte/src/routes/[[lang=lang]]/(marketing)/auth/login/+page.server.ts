import { env } from "$env/dynamic/public";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import {
  captchaVerify,
  handleApiResponse,
  isSuccessResponse,
  setTokenCookie,
  ValidateAccess,
} from "$lib/server";
import { createUser } from "$lib/server/mock/user";
import { z, type AuthResponse } from "$lib/types";
import { validateRequired } from "@noxlovette/svarog";
import { fail, type Actions } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";

export const load = async () => {
  const form = await superValidate(zod4(z.signinBody));

  return { form };
};

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
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

    const response = await fetch(routes.auth.signin(), {
      method: "POST",
      body: JSON.stringify({ username, pass }),
    });

    const authResult = await handleApiResponse<AuthResponse>(response);
    if (!isSuccessResponse(authResult)) {
      return fail(authResult.status);
    }

    if (!env.PUBLIC_MOCK_MODE) {
      const { accessToken, refreshToken } = authResult.data;
      setTokenCookie(cookies, "accessToken", accessToken);
      setTokenCookie(cookies, "refreshToken", refreshToken);

      const user = await ValidateAccess(accessToken.token);

      logger.debug({ user }, "user from the login");
    }

    const user = createUser();

    return {
      user,
      success: true,
    };
  },
};
