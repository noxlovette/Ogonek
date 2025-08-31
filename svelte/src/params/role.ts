import type { Role } from "$lib/types";
import type { ParamMatcher } from "@sveltejs/kit";

export const match = ((param: string): param is Role => {
  return param === "t" || param === "s";
}) satisfies ParamMatcher;
