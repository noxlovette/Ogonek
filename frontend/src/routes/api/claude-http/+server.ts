import type { RequestHandler } from '@sveltejs/kit';
import Anthropic from '@anthropic-ai/sdk';

export const GET: RequestHandler = async ({ request }) => {
	const client = new Anthropic({
		apiKey: process.env.ANTHROPIC_API_KEY
	});

	try {
		const response = await client.messages.create({
			max_tokens: 1024,
			messages: [{ role: 'user', content: 'Hello, Claude' }],
			model: 'claude-3-haiku-20240307'
		});

		// Extract the message text from the content array
		const messageText = response.content[0].text;

		return new Response(
			JSON.stringify({
				message: messageText,
				id: response.id,
				model: response.model
			}),
			{
				headers: {
					'Content-Type': 'application/json',
					'Access-Control-Allow-Origin': '*'
				}
			}
		);
	} catch (error) {
		return new Response(JSON.stringify({ error: error.message }), {
			status: 500,
			headers: {
				'Content-Type': 'application/json',
				'Access-Control-Allow-Origin': '*'
			}
		});
	}
};
