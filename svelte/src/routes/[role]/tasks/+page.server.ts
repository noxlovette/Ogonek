import { env } from "$env/dynamic/private";
import { notifyTelegram } from "$lib/server";
import { fail, redirect, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
  new: async ({ fetch }) => {
    const body = {
      title: "New Task",
      markdown: "## Try adding some content here",
    };

    const response = await fetch(`/axum/task`, {
      method: "POST",
      body: JSON.stringify(body),
    });

    const { id } = await response.json();

    if (response.ok) {
      return redirect(301, `/t/tasks/t/${id}/edit`);
    }
  },
  requestHW: async ({ request }) => {
    const formData = await request.formData();
    const username = formData.get("username");

    const message = `${username} needs homework`;

    const telegramResponse = await notifyTelegram(
      message,
      env.TELEGRAM_CHAT_ID,
    );
    if (telegramResponse.status !== 200) {
      return fail(400);
    }
  },
};
