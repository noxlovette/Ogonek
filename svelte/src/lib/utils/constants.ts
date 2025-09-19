import type { superUser } from "$lib/types";

export function isSuperUser(role: string): role is superUser {
  return ["moderator", "admin", "god"].includes(role);
}
