import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeStringify from 'rehype-stringify';
import rehypeFormat from 'rehype-format';
import rehypeSanitize from 'rehype-sanitize';
import remarkGfm from 'remark-gfm';


export async function parseMarkdown(content: string) {
	const processor = unified()
		.use(remarkParse)
		.use(remarkGfm)
		.use(remarkRehype, { allowDangerousHtml: true })
		.use(rehypeFormat)
		.use(rehypeSanitize)
		.use(rehypeStringify);

	const result = await processor.process(content);
	return String(result);
}

export const stripUUID = (str: string): string => {
	const uuidPattern = /^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}/;
	return str.replace(uuidPattern, '').replace(/^-+/, '');
};

