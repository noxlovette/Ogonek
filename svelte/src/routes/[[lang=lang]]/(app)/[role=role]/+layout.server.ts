import { env } from "$env/dynamic/private";
import { routes } from "$lib/routes";
import redis from "$lib/server/redisClient";
import type { AppContext, NotificationBadges } from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const context = (await fetch(routes.state.context()).then((res) =>
    res.json(),
  )) as AppContext;

  const badges = (await fetch(routes.state.badges()).then((res) =>
    res.json(),
  )) as NotificationBadges;

  const word = redis.get("wordAPI").then(async (cachedWord) => {
    if (cachedWord) {
      return JSON.parse(cachedWord);
    } else {
      const response = await fetch(
        "https://wordsapiv1.p.rapidapi.com/words?random=true",
        {
          method: "GET",
          headers: {
            "x-rapidapi-host": "wordsapiv1.p.rapidapi.com",
            "x-rapidapi-key": env.API_WORD_KEY,
          },
        },
      );
      const word = await response.json();
      await redis.set("wordAPI", JSON.stringify(word), "EX", 60 * 60 * 24);
      return word;
    }
  });

  return {
    word,
    badges,
    context,
    user: context.user,
    profile: context.profile,
    callURL: context.callUrl,
    students: context.students,
  };
};
