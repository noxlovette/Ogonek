import { env } from "$env/dynamic/private";

export const captchaVerify = async (smartToken: string) => {
  const response = await fetch(
    "https://smartcaptcha.yandexcloud.net/validate",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: new URLSearchParams({
        secret: env.YANDEX_SECRET,
        token: smartToken,
      }),
    },
  );

  if (!response.ok) {
    throw new Error(`Yandex verification failed: ${response.status}`);
  }
  return response;
};
