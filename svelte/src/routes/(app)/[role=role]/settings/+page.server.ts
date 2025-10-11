import { redirect, type Actions } from "@sveltejs/kit";
export const actions = {
  logout: async (event) => {
    event.cookies.delete("accessToken", { path: "/" });
    event.cookies.delete("refreshToken", { path: "/" });
    throw redirect(301, "/");
  },
} satisfies Actions;
