import logger from "$lib/logger";
import { parseMarkdown } from "@noxlovette/svarog";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import termsContent from "./terms.md?raw";

export const load: PageServerLoad = async () => {
  try {
    const parsedMarkdown = await parseMarkdown(termsContent);
    return { markdown: parsedMarkdown };
  } catch (err) {
    logger.error("Error reading markdown file:", err);
    throw error(500, "Error loading project content");
  }
};
