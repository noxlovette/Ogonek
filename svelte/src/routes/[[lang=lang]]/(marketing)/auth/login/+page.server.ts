import { env } from "$env/dynamic/public";
import { z } from "$lib";
import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { captchaVerify, setTokenCookie, ValidateAccess } from "$lib/server";
import { createUser } from "$lib/server/mock/user";
import { validateForm } from "$lib/utils";
import { fail, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();
    const validation = validateForm(formData, z.signupBody);
    if (!validation.success) return validation.failure;
    const { pass, username } = validation.data;
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

    if (!env.PUBLIC_MOCK_MODE) {
      const { accessToken, refreshToken } = parsed.data;
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
