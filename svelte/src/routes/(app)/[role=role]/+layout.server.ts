import { routes } from "$lib/routes";
import type { AppContext, NotificationBadges } from "$lib/types";
import type { LayoutServerLoad } from "./$types";
export const load: LayoutServerLoad = async ({ fetch }) => {
  const context = (await fetch(routes.state.context()).then((res) =>
    res.json(),
  )) as AppContext;

  const badges = (await fetch(routes.state.badges()).then((res) =>
    res.json(),
  )) as NotificationBadges;

  return {
    badges,
    context,
    user: context.user,
    profile: context.profile,
    callURL: context.callUrl,
    students: context.students,
  };
};
