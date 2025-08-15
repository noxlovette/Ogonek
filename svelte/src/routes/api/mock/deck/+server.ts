import type { DeckSmall } from "$lib/types";
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const decks: DeckSmall[] = [
    {
      id: "deck1",
      title: "Hello there",
      description: "Murrrrrr",
      assigneeName: "Kotya",
      visibility: "assigned",
      isSubscribed: true,
      seen: true,
    },
    {
      id: "deck2",
      title: "Hello there",
      description: "Kotya Assigned",
      assigneeName: "Kotya",
      visibility: "private",
      isSubscribed: false,
      seen: true,
    },
  ];
  return json(decks);
};
export const POST: RequestHandler = async () => {
  const id = "deck1";

  return json(id);
};
