import type { DashboardData } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const dashboardData: DashboardData = {
    tasks: [],
    lessons: [],
    decks: [],
    activity: [],
    learnData: {
      cardsStudiedToday: 9,
      currentStreak: 10,
    },
  };

  return json(dashboardData);
};
