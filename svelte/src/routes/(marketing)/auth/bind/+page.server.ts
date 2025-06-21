import {
  handleApiResponse,
  isSuccessResponse,
  ValidateAccess,
} from "$lib/server";

import type { UserAndTeacher } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const invite = url.searchParams.get("invite");
  const response = await fetch(`/axum/user?invite=${invite}`).then(
    (res) => res.json() as Promise<UserAndTeacher>,
  );

  const { teacher, user } = response;
  return {
    user,
    teacher,
  };
};

export const actions: Actions = {
  default: async ({ url, fetch, cookies }) => {
    const inviteToken = url.searchParams.get("invite");

    const accessToken = cookies.get("accessToken");

    const user = await ValidateAccess(accessToken);
    const studentId = user.sub;

    const inviteResponse = await fetch("/axum/auth/bind", {
      method: "POST",
      body: JSON.stringify({ inviteToken, studentId }),
    });

    const inviteResult = await handleApiResponse(inviteResponse);

    if (!isSuccessResponse(inviteResult)) {
      return fail(inviteResult.status, { message: inviteResult.message });
    }

    return redirect(302, "/");
  },
} satisfies Actions;
