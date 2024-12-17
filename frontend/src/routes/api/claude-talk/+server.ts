import type { RequestHandler } from '@sveltejs/kit';
import { Readable } from 'stream';
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

// Use a Map to store active streams per client session or connection ID
const streams = new Map();

export const GET: RequestHandler = async ({ request }) => {
  // Generate or use a unique identifier for each client session/connection
  const clientId = request.headers.get('X-Client-ID') || Math.random().toString(36).substring(2, 15); // Fallback to a random ID

  // If there's no existing stream for this client, create one
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
      'X-Client-ID': clientId, // Return the client ID so it can be used in subsequent POST requests
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
    const streamInfo = streams.get(clientId);

    if (!streamInfo) {
      return new Response(JSON.stringify({ error: "No active stream for this session" }), {
        status: 404,
        headers: { 'Content-Type': 'application/json' }
      });
    }

    // Add the message to the stream's message queue
    streamInfo.clientMessages.push({ role: 'user', content: message });

    // Here you might want to trigger a new message in the existing stream if necessary
    // For simplicity, this example does not re-initiate the stream but could append messages

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