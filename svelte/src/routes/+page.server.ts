import { ValidateAccess } from "$lib/server";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ cookies }) => {
  const accessToken = cookies.get("accessToken");
  if (accessToken) {
    const user = await ValidateAccess(accessToken);
    const href = user.role === "teacher" ? "/t/dashboard" : "/s/dashboard";
    throw redirect(301, href);
  }
};
