import type { DeckSmall } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const decks: DeckSmall[] = [
    {
      id: "deck1",
      name: "Hello there",
      description: "Murrrrrr",
      assigneeName: "Kotya",
      visibility: "assigned",
      seen: true,
    },
    {
      id: "deck2",
      name: "Hello there",
      description: "Kotya Assigned",
      assigneeName: "Kotya",
      visibility: "private",
      seen: true,
    },
  ];

  return new Response(JSON.stringify(decks), {
    headers: { "Content-Type": "application/json" },
  });
};
export const POST: RequestHandler = async () => {
  const id = "deck1";

  return new Response(JSON.stringify({ id }), {
    headers: { "Content-Type": "application/json" },
  });
};
