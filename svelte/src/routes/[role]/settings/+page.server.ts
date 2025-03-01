import {
  ValidateAccess,
  handleApiResponse,
  isSuccessResponse,
} from "$lib/server";
import type { AuthResponse, Profile, User } from "$lib/types";
import { fail, redirect, type Actions } from "@sveltejs/kit";
export const actions = {
  update: async ({ request, fetch }) => {
    const formData = await request.formData();
    const validateZoom = (url: string) => {
      if (!url) return false;
      return /^https?:\/\/(?:[a-z0-9-]+\.)?zoom\.us\/j\/\d{9,11}(?:\?pwd=[a-zA-Z0-9]+)?$/.test(
        url,
      );
    };

    const validateQuizlet = (url: string) => {
      if (!url) return false;
      return /^https?:\/\/quizlet\.com\/.*$/.test(url);
    };
    if (!validateZoom(formData.get("zoom") as string)) {
      return fail(400, { message: "Please enter a Zoom Room URL" });
    }

    if (!validateQuizlet(formData.get("quizlet") as string)) {
      return fail(400, { message: "Please enter a valid Quizlet URL" });
    }

    const profileBody = {
      quizletUrl: formData.get("quizlet"),
      zoomUrl: formData.get("zoom"),
      bio: formData.get("bio"),
      avatar_url: formData.get("avatarUrl"),
    };

    const userBody = {
      email: formData.get("email"),
      username: formData.get("username"),
      name: formData.get("name"),
    };

    const validateEmail = (email: string) => {
      if (!email) return false;
      return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email);
    };

    if (!validateEmail(formData.get("email") as string)) {
      return fail(400, { message: "Invalid Email" });
    }

    const [profileRes, _, refreshRes] = await Promise.all([
      fetch("/axum/profile", {
        method: "PATCH",
        body: JSON.stringify(profileBody),
      }),
      fetch("/axum/user", {
        method: "PATCH",
        body: JSON.stringify(userBody),
      }),
      fetch("/auth/refresh"),
    ]);

    const profileResult = await handleApiResponse<Profile>(profileRes);
    if (!isSuccessResponse(profileResult)) {
      return fail(profileResult.status, { message: profileResult.message });
    }

    const authResult = await handleApiResponse<AuthResponse>(refreshRes);

    if (!isSuccessResponse(authResult)) {
      return fail(authResult.status, { message: authResult.message });
    }

    const { accessToken } = authResult.data;
    const user = (await ValidateAccess(accessToken)) as User;

    if (!user) {
      return fail(500, { message: "Invalid Token" });
    }

    return {
      success: true,
      profile: profileResult.data,
      user,
      message: "Profile updated successfully",
    };
  },
  logout: async (event) => {
    event.cookies.delete("accessToken", { path: "/" });
    event.cookies.delete("refreshToken", { path: "/" });
    throw redirect(301, "/");
  },
} satisfies Actions;
