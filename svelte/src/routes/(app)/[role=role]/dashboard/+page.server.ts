import { routes } from "$lib/routes";
import type { DashboardData } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const dashboardData = (await fetch(routes.state.dashboard()).then((res) =>
    res.json(),
  )) as DashboardData;

  return {
    tasks: dashboardData.tasks,
    events: dashboardData.events,
  };
};
