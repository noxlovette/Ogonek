import { env } from "$env/dynamic/private";

export const turnstileVerify = async (turnstileToken: string) => {
  const response = await fetch(
    "https://challenges.cloudflare.com/turnstile/v0/siteverify",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: new URLSearchParams({
        secret: env.CLOUDFLARE_SECRET,
        response: turnstileToken,
      }),
    },
  );

  if (!response.ok) {
    throw new Error(`Turnstile verification failed: ${response.status}`);
  }
  return response;
};
