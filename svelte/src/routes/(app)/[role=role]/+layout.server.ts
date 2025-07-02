import { env } from "$env/dynamic/private";
import logger from "$lib/logger";
import redis from "$lib/redisClient";
import type { DashboardData } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const startTime = performance.now();
  try {
    const dashboardData = (await fetch("/axum/dashboard").then((res) =>
      res.json(),
    )) as DashboardData;

    console.log(dashboardData);

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

    logger.info(
      { duration: performance.now() - startTime },
      "Fetching all the good dashboard stuff successful",
    );
    return {
      students: dashboardData.students,
      lessons: dashboardData.lessons,
      tasks: dashboardData.tasks,
      word,
      profile: dashboardData.profile.profile,
      teacherData: dashboardData.profile.teacherData,
      user: dashboardData.user,
    };
  } catch (err: any) {
    logger.error(
      { err, duration: performance.now() - startTime },
      "Error fetching all the sweet stuff for the dashboard",
    );
    return error(500);
  }
};
