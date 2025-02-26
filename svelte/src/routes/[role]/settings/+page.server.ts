import { ValidateAccess } from "$lib/server";
import type { Profile } from "$lib/types";
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

    const profile: Profile = await profileRes.json();

    const { accessToken } = await refreshRes.json();
    const user = await ValidateAccess(accessToken);

    if (!user) {
      return fail(500, { message: "Invalid Token" });
    }

    return {
      success: true,
      profile,
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
