import { parseMarkdown } from "$lib/utils";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import termsContent from "./privacy.md?raw";

export const load: PageServerLoad = async () => {
  try {
    const parsedMarkdown = await parseMarkdown(termsContent);
    return { markdown: parsedMarkdown };
  } catch (err) {
    console.error("Error reading markdown file:", err);
    throw error(500, "Error loading project content");
  }
};
