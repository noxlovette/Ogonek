import {
  handleApiResponse,
  isSuccessResponse,
  parseCookieOptions,
  turnstileVerify,
  ValidateAccess,
} from "$lib/server";
import type { AuthResponse, Profile, User } from "$lib/types";
import { validateRequired } from "@noxlovette/svarog";
import { fail, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    try {
      const data = await request.formData();
      const username = data.get("username") as string;
      const pass = data.get("password") as string;
      const turnstileToken = data.get("cf-turnstile-response") as string;

      if (!turnstileToken) {
        return fail(400, {
          message: "Please complete the CAPTCHA verification",
        });
      }

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

      const turnstileResponse = await turnstileVerify(turnstileToken);
      if (!turnstileResponse.ok) {
        return fail(400, {
          message: "Turnstile verification failed",
        });
      }

      const response = await fetch("/axum/auth/signin", {
        method: "POST",
        body: JSON.stringify({ username, pass }),
      });

      const authResult = await handleApiResponse<AuthResponse>(response);

      if (!isSuccessResponse(authResult)) {
        return fail(authResult.status, { message: authResult.message });
      }

      // Handle cookies from response
      response.headers.getSetCookie().forEach((cookie) => {
        const [fullCookie, ...opts] = cookie.split(";");
        const [name, value] = fullCookie.split("=");
        const cookieOptions = parseCookieOptions(opts);
        cookies.set(name, value, cookieOptions);
      });

      // Validate the access token
      const { accessToken } = authResult.data;
      const user = (await ValidateAccess(accessToken)) as User;

      if (!user) {
        return fail(401, {
          message: "Invalid access token",
        });
      }

      // Fetch user profile with typed response handling
      const profileResponse = await fetch("/axum/profile");
      const profileResult = await handleApiResponse<Profile>(profileResponse);

      if (!isSuccessResponse(profileResult)) {
        return fail(profileResult.status, { message: profileResult.message });
      }

      return {
        success: true,
        message: "Login successful",
        user,
        profile: profileResult.data,
      };
    } catch (error) {
      console.error("Signin error:", error);
      return fail(500, {
        message:
          error instanceof Error ? error.message : "Internal server error",
      });
    }
  },
};
