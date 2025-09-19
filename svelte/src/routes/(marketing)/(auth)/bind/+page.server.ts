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
  if (invite) {
    const inviter = await fetch(routes.users.inviter(invite)).then(
      (res) => res.json() as Promise<User>,
    );

    return {
      inviter,
    };
  }
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
