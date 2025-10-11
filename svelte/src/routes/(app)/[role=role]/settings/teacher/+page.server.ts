import logger from "$lib/logger";
import { routes } from "$lib/routes";
import { fail, type Actions } from "@sveltejs/kit";
export const actions = {
  update: async ({ request, fetch, params }) => {
    const formData = await request.formData();

    const url = formData.get("videoCallUrl")?.toString();
    if (!url) {
      return fail(400, { url: true });
    }
    if (!/^https?:\/\//.test(url)) {
      return fail(400, { url: true });
    }

    const profileBody = {
      videoCallUrl: url,
      avatarUrl: formData.get("avatarUrl"),
      telegramId: formData.get("telegramId"),
    };

    const profileRes = await fetch(routes.users.profile(), {
      method: "PATCH",
      body: JSON.stringify(profileBody),
    });
    if (!profileRes.ok) {
      const err = profileRes.text();
      logger.error({ err });
      return fail(500);
    }

    return {
      success: true,
    };
  },
} satisfies Actions;
