import matter from "gray-matter";
import rehypeSanitize, { defaultSchema } from "rehype-sanitize";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";

export async function parseMarkdown(content: string): Promise<string> {
  const processor = unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype, { allowDangerousHtml: false })
    .use(rehypeSanitize, defaultSchema)
    .use(rehypeStringify);

  const result = await processor.process(content);
  return String(result);
}

/**
 * Extract frontmatter if you need it (keeping this since you use it)
 */
export function extractFrontmatter(
  content: string,
): [Record<string, any>, string] {
  const { data, content: markdownContent } = matter(content);
  return [data, markdownContent];
}
