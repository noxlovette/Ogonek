import type { Card } from "$lib/types";
import { nanoid } from "nanoid";

/**
 * Creates a single card with sensible defaults
 */
export function createCard(overrides: Partial<Card> = {}): Card {
  return {
    id: nanoid(),
    front: "What is the capital of France?",
    back: "Paris",
    mediaUrl: null,
    ...overrides,
  };
}

/**
 * Creates multiple cards with realistic content
 */
export function createCards(
  count: number = 5,
  deckId?: string,
  overrides: Partial<Card> = {},
): Card[] {
  const sampleCards = [
    { front: "What is the capital of France?", back: "Paris" },
    { front: "What does HTML stand for?", back: "HyperText Markup Language" },
    { front: "Who wrote '1984'?", back: "George Orwell" },
    { front: "What is 2 + 2?", back: "4" },
    { front: "What is the largest planet?", back: "Jupiter" },
  ];

  return Array.from({ length: count }, (_, i) => {
    const sample = sampleCards[i % sampleCards.length];
    return createCard({
      front: `${sample.front}`,
      back: `${sample.back}`,
      ...overrides,
    });
  });
}

/**
 * Creates cards with media URLs for testing rich content
 */
export function createCardsWithMedia(
  count: number = 3,
  deckId?: string,
): Card[] {
  return createCards(count, deckId, {
    mediaUrl: `https://picsum.photos/400/300?random=${nanoid(4)}`,
  });
}

/**
 * Creates cards for a specific deck with varied creation dates
 */
export function createCardsForDeck(deckId: string, count: number = 10): Card[] {
  return createCards(count, deckId).map((card) => ({
    ...card,
  }));
}

/**
 * Creates a single flashcard with custom Q&A
 */
export function createFlashcard(
  front: string,
  back: string,
  overrides: Partial<Card> = {},
): Card {
  return createCard({
    front,
    back,
    ...overrides,
  });
}

/**
 * Bulk creates cards from an array of Q&A pairs
 */
export function createCardsFromPairs(
  pairs: Array<{ front: string; back: string }>,
  deckId?: string,
  overrides: Partial<Card> = {},
): Card[] {
  return pairs.map(({ front, back }) =>
    createFlashcard(front, back, overrides),
  );
}
