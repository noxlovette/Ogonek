import type { ParamMatcher } from "@sveltejs/kit";

export const match = ((
  param: string,
): param is "en" | "fr" | "it" | "de" | "ru" => {
  return (
    param === "en" ||
    param === "fr" ||
    param === "it" ||
    param === "de" ||
    param === "ru"
  );
}) satisfies ParamMatcher;
