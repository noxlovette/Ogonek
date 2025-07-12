import type { CardProgress } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  const today = new Date();
  const dueDate = new Date(today.setDate(today.getDate() + 1)).toISOString();
  const progress: CardProgress[] = [
    {
      id: "progress1",
      cardId: "card1",
      userId: "1",
      reviewCount: 3,
      lastReviewed: today.toISOString(),
      dueDate,
      easeFactor: 3,
      interval: 10,
      front: "HELLO",
      back: "BACK",
      mediaUrl: null,
    },
    {
      id: "progress2",
      cardId: "card1",
      userId: "1",
      reviewCount: 3,
      lastReviewed: today.toISOString(),
      dueDate,
      easeFactor: 3,
      interval: 10,
      front: "HELLO",
      back: "BACK",
      mediaUrl: null,
    },
  ];

  return new Response(JSON.stringify(progress), {
    headers: { "Content-Type": "application/json" },
  });
};
