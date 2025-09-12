import type { superUser } from "$lib/types";

export const qualityButtons = [
  {
    quality: 0,
    label: "1066",
    color: "ring-red-600 hover:bg-red-700/10 ring",
    key: 1,
  },

  {
    quality: 3,
    label: "Ok",
    color: "ring-yellow-500 hover:bg-yellow-600/10 ring",
    key: 2,
  },
  {
    quality: 5,
    label: "Easy",
    color: "ring-green-500 hover:bg-green-500 ring",
    key: 3,
  },
];

export function isSuperUser(role: string): role is superUser {
  return ["moderator", "admin", "god"].includes(role);
}
