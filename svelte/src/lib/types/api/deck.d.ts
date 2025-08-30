export interface DeckSmall {
  id: string;
  isSubscribed?: boolean;
  title: string;
  description: string;
  visibility: "private" | "assigned" | "public";
  seen: boolean;
  assigneeName: string;
}
export interface Card {
  id: string;
  front: string;
  back: string;
  mediaUrl?: string | null;
  deckId: string;
  createdAt?: string;
}

export interface CardProgress {
  id: string;
  cardId: string;
  userId: string;
  reviewCount: number;
  lastReviewed: string | null;
  dueDate: string;
  easeFactor: number;
  interval: number;
  front: string;
  back: string;
  mediaUrl: string | null;
}
