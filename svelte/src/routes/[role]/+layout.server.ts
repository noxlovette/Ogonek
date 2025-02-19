import { env } from "$env/dynamic/private";
import redis from "$lib/redisClient";
import type { Lesson, Student, Task } from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const [students, lessons, tasks] = await Promise.all([
    fetch("/axum/student").then((res) => res.json() as Promise<Student[]>),
    fetch("/axum/lesson").then((res) => res.json() as Promise<Lesson[]>),
    fetch("/axum/task").then((res) => res.json() as Promise<Task[]>),
  ]);

  let word;
  const cachedWord = await redis.get("wordAPI");

  if (cachedWord) {
    word = await JSON.parse(cachedWord);
  } else {
    word = await fetch("https://wordsapiv1.p.rapidapi.com/words?random=true", {
      method: "GET",
      headers: {
        "x-rapidapi-host": "wordsapiv1.p.rapidapi.com",
        "x-rapidapi-key": env.API_WORD_KEY,
      },
    }).then((res) => res.json());
    await redis.set("wordAPI", JSON.stringify(word), "EX", 60 * 60 * 24);
  }

  console.log(word);

  return {
    students,
    lessons,
    tasks,
    word,
  };
};
