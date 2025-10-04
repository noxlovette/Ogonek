import { ValidateAccess } from "$lib/server";

import logger from "$lib/logger";
import { routes } from "$lib/routes";
import type { User } from "$lib/types";
import { redirect } from "@sveltejs/kit";
import { error } from "console";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, url }) => {
  const invite = url.searchParams.get("invite");
  if (invite) {
    const inviter = await fetch(routes.users.inviter({ invite })).then(
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

    const response = await fetch(routes.auth.bind_student_to_teacher(), {
      method: "POST",
      body: JSON.stringify({ inviteToken, studentId }),
    });

    if (!response.ok) {
      const err = await response.text();
      logger.error({ err }, "binding an existing user failed");
      return error(500);
    }

    return redirect(302, "/");
  },
} satisfies Actions;
