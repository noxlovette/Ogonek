import type { RequestHandler } from "./$types";

export const DELETE: RequestHandler = async () => {
  return new Response(null, { status: 204 });
};

export const POST: RequestHandler = async () => {
  return new Response(null, { status: 204 });
};
