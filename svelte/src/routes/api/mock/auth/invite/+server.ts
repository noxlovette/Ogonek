import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
export const GET: RequestHandler = async ({ url }) => {
  // Query params: invite
  // Generates the invite link for the teacher

  const isRegistered = url.searchParams.get("invite");

  return json(isRegistered, { status: 200 });
};
