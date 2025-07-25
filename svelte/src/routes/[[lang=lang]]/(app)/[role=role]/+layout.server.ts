import { env } from "$env/dynamic/private";
import { routes } from "$lib/routes";
import redis from "$lib/server/redisClient";
import type { DashboardData } from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const dashboardData = (await fetch(routes.users.dashboard()).then((res) =>
    res.json(),
  )) as DashboardData;

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
    students: dashboardData.students,
    lessons: dashboardData.lessons,
    tasks: dashboardData.tasks,
    decks: dashboardData.decks,
    word,
    profile: dashboardData.profile.profile,
    teacherData: dashboardData.profile.teacherData,
    user: dashboardData.user,
    activity: dashboardData.activity,
    learn: dashboardData.learn,
    preferences: dashboardData.preferences,
  };
};
