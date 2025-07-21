import { parseMarkdown } from "@noxlovette/svarog";
import type { PageServerLoad } from "./$types";
import termsContent from "./terms.md?raw";

export const load: PageServerLoad = async () => {
  const parsedMarkdown = await parseMarkdown(termsContent);
  return { markdown: parsedMarkdown };
};
