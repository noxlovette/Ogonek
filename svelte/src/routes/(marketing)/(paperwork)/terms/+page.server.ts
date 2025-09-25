import { parseMarkdown } from "$lib/utils/markdown";
import type { PageServerLoad } from "./$types";
import termsContent from "./terms.md?raw";

export const load: PageServerLoad = async () => {
  const parsedMarkdown = await parseMarkdown(termsContent);
  return { markdown: parsedMarkdown };
};
