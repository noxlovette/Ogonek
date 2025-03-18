import { env } from "$env/dynamic/private";
import redis from "$lib/redisClient";
import type { LessonSmall, Student, TaskSmall } from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const [students, lessons, tasks] = await Promise.all([
    fetch("/axum/student").then((res) => res.json() as Promise<Student[]>),
    fetch("/axum/lesson/recent").then(
      (res) => res.json() as Promise<LessonSmall>,
    ),
    fetch("/axum/task/recent").then((res) => res.json() as Promise<TaskSmall>),
  ]);

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
    students,
    lessons,
    tasks,
    word,
  };
};
