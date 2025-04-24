import { env } from "$env/dynamic/private";
import redis from "$lib/redisClient";
import type {
  LessonSmall,
  ProfileComposite,
  Student,
  TaskSmall,
  UserAndTeacher,
} from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch, params }) => {
  const [students, lessons, tasks, userAndTeacher, profileComposite] =
    await Promise.all([
      fetch("/axum/student").then((res) => res.json() as Promise<Student[]>),
      fetch("/axum/lesson/recent").then(
        (res) => res.json() as Promise<LessonSmall[]>,
      ),
      fetch("/axum/task/recent").then(
        (res) => res.json() as Promise<TaskSmall[]>,
      ),
      fetch("/axum/user").then((res) => res.json() as Promise<UserAndTeacher>),
      fetch(
        `/axum/profile?isStudent=${params.role === "t" ? false : true}`,
      ).then((res) => res.json() as Promise<ProfileComposite>),
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

  const { user } = userAndTeacher;

  return {
    students,
    lessons,
    tasks,
    word,
    profile: profileComposite.profile,
    teacherData: profileComposite.teacherData,
    user,
  };
};
