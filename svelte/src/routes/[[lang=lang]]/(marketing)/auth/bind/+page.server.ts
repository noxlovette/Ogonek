import {
  handleApiResponse,
  isSuccessResponse,
  ValidateAccess,
} from "$lib/server";

import { routes } from "$lib/routes";
import type { User } from "$lib/types";
import { fail, redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const invite = url.searchParams.get("invite");
  const inviter = await fetch(`/axum/user/inviter?invite=${invite}`).then(
    (res) => res.json() as Promise<User>,
  );

  return {
    inviter,
  };
};

export const actions: Actions = {
  default: async ({ fetch, cookies }) => {
    const accessToken = cookies.get("accessToken");

    const user = await ValidateAccess(accessToken);
    const studentId = user.sub || "";

    const inviteResponse = await fetch(routes.students.bind(studentId), {
      method: "POST",
    });

    const inviteResult = await handleApiResponse(inviteResponse);

    if (!isSuccessResponse(inviteResult)) {
      return fail(inviteResult.status, { message: inviteResult.message });
    }

    return redirect(302, "/");
  },
} satisfies Actions;
