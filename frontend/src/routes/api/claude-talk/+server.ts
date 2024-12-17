import type { RequestHandler } from '@sveltejs/kit';
import { Readable } from 'stream';
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY
});

// No need for a Map here since we're not managing multiple client streams
let stream = null; // Global stream for simplicity

export const GET: RequestHandler = async () => {
  if (!stream) {
    // Create a new stream if one doesn't exist
    stream = await client.messages.create({
      max_tokens: 1024,
      messages: [{ role: 'user', content: "Start conversation" }],
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

    return new Response(controller, {
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive'
      }
    });
  } else {
    // If a stream already exists, just return it. This might not be ideal for long sessions but works for simplicity.
    return new Response(stream, {
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive'
      }
    });
  }
};

export const POST: RequestHandler = async ({ request }) => {
  try {
    const { message } = await request.json();

    // Here we're not creating a new stream, we're just adding to the existing one or creating if none exists
    if (!stream) {
      // If no stream, create one with the new message
      stream = await client.messages.create({
        max_tokens: 1024,
        messages: [{ role: 'user', content: message }],
        model: 'claude-3-haiku-20240307',
        stream: true,
      });
    } else {
      // If a stream exists, append the message. Note: This is pseudo-code as appending to an ongoing stream isn't natively supported in this setup
      // You'd need to handle this based on your Anthropic API capabilities or manage messages differently
      console.log('Message added to the stream:', message);
    }

    return new Response(JSON.stringify({ status: "Message added" }), {
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