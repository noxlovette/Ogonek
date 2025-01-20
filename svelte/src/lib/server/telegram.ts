import { env } from '$env/dynamic/private';

interface TelegramResponse {
	ok: boolean;
	description?: string;
}

/**
 * Sends a notification message via Telegram Bot API
 * @param message - The message to send
 * @param addressee - Telegram chat ID
 * @returns Promise<Response> - SvelteKit response object
 */
export async function notifyTelegram(message: string, addressee: string): Promise<Response> {
	if (!env.TELEGRAM_API) {
		console.error('Telegram API token not configured');
		return new Response(JSON.stringify({ message: 'Telegram configuration missing' }), {
			status: 500
		});
	}

	try {
		const response = await fetch(`https://api.telegram.org/bot${env.TELEGRAM_API}/sendMessage`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				chat_id: addressee,
				text: message,
				parse_mode: 'MarkdownV2'
			})
		});

		const data = (await response.json()) as TelegramResponse;

		if (!response.ok) {
			console.error('Telegram API error:', data.description);
			return new Response(
				JSON.stringify({
					message: 'Failed to send message',
					error: data.description
				}),
				{ status: response.status }
			);
		}

		return new Response(JSON.stringify({ message: 'Message sent successfully' }), { status: 200 });
	} catch (error) {
		console.error('Failed to send Telegram message:', error);
		return new Response(
			JSON.stringify({
				message: 'Internal server error',
				error: error instanceof Error ? error.message : 'Unknown error'
			}),
			{ status: 500 }
		);
	}
}
