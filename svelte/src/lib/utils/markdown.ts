import logger from "$lib/logger";
import rehypeSanitize, { defaultSchema } from "rehype-sanitize";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";

export async function parseMarkdown(content: string): Promise<string> {
  try {
    const processor = unified()
      .use(remarkParse)
      .use(remarkGfm)
      .use(remarkRehype, { allowDangerousHtml: false })
      .use(rehypeSanitize, defaultSchema)
      .use(rehypeStringify);

    const result = await processor.process(content);
    return String(result);
  } catch (error) {
    logger.error({ error });
    return "С вашим Markdown что-то не так";
  }
}
