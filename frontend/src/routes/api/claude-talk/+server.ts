import type { RequestHandler } from '@sveltejs/kit';
import { Readable } from 'stream';
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

const streams = new Map();

export const GET: RequestHandler = async ({ request }) => {
  let clientId = request.headers.get('X-Client-ID') || request.url.split('clientId=')[1];

  if (!clientId) {
    clientId = Math.random().toString(36).substring(2, 15); // Fallback to a random ID
  }

  if (!streams.has(clientId)) {
    const stream = await client.messages.create({
      max_tokens: 1024,
      messages: [{ role: 'user', content: "Hello!" }],
      model: 'claude-3-haiku-20240307',
      stream: true,
    });

    const controller = new ReadableStream({
      start: async (controller) => {
        for await (const event of stream) {
          if (event.type === "content_block_delta") {
            controller.enqueue(new TextEncoder().encode(`data: ${JSON.stringify({ text: event.delta.text })}\n\n`));
          }
        }
      }
    });

    streams.set(clientId, { stream: controller, clientMessages: [] });
  }

  return new Response(streams.get(clientId).stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache',
      'Connection': 'keep-alive',
      'X-Client-ID': clientId,
    }
  });
};

export const POST: RequestHandler = async ({ request }) => {
  try {
    const clientId = request.headers.get('X-Client-ID');
    if (!clientId) {
      return new Response(JSON.stringify({ error: "Client ID not provided" }), {
        status: 400,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    const { message } = await request.json();
    let streamInfo = streams.get(clientId);

    if (!streamInfo) {
      // If no stream exists for this client ID, create one (same as GET)
      const stream = await client.messages.create({
        max_tokens: 1024,
        messages: [{ role: 'user', content: message }],
        model: 'claude-3-haiku-20240307',
        stream: true,
      });

      const controller = new ReadableStream({
        start: async (controller) => {
          for await (const event of stream) {
            if (event.type === "content_block_delta") {
              controller.enqueue(new TextEncoder().encode(`data: ${JSON.stringify({ text: event.delta.text })}\n\n`));
            }
          }
        }
      });

      streamInfo = { stream: controller, clientMessages: [] };
      streams.set(clientId, streamInfo);
    }

    // Add the message and potentially trigger a new stream if you need to respond
    streamInfo.clientMessages.push({ role: 'user', content: message });

    return new Response(JSON.stringify({ status: "Message added to the stream" }), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      }
    });
  } catch (error) {
    console.error('Error:', error);
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      }
    });
  }
};