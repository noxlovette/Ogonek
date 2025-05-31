import type { ParamMatcher } from "@sveltejs/kit";

export const match = ((param: string): param is "t" | "s" => {
  return param === "t" || param === "s";
}) satisfies ParamMatcher;
