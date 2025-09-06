import { z } from "$lib";
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
        const fieldPath = issue.path[0] as string;
        fieldErrors[fieldPath] = true;
      });

      return fail(400, {
        ...fieldErrors,
        message: "Validation failed",
      });
    }
    const { username, pass, email, role, name } = validation.data;
    const inviteToken = url.searchParams.get("invite");

    const captchaToken = formData.get("smart-token") as string;
    if (!captchaToken) {
      return fail(400, {
        captcha: true,
        message: "Please complete the CAPTCHA verification",
      });
    }
    const captchaResponse = await captchaVerify(captchaToken);
    if (!captchaResponse.ok) {
      return fail(400, {
        message: "Captcha verification failed",
      });
    }

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
