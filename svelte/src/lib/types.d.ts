export interface Task {
  id: string;
  title: string;
  markdown: string;
  priority: int;
  createdAt: string;
  updatedAt: string;
  dueDate: string;
  completed: boolean;
  filePath: string;
  createdBy: string;
  assignee: string;
  assigneeName: string;
}
export interface Toast {
  message: string | null;
  type: "success" | "error" | "info" | null;
}
export interface Profile {
  quizletUrl: string | null;
  zoomUrl: string | null;
  bio: string | null;
  avatarUrl: string | null;
}

export interface User {
  name: string | null;
  username: string | null;
  role: string | null;
  email: string | null;
  sub: string | null;
}

export interface Lesson {
  id: string;
  manualDate?: string;
  title: string;
  markdown: string;
  createdAt: string;
  updatedAt: string;
  topic: string;
  bookmarked: boolean;
  highlighted: string;
  assignee: string;
  assigneeName: string;
}

export interface LessonStore {
  title: string;
  markdown: string;
  topic: string;
}

export interface Student {
  id: string;
  name: string;
  username: string;
  email: string;
  role: string;
  markdown: string;
  joined: string;
  telegramId: string;
}

export interface BaseTableItem {
  id: string;
}

export interface TableConfig<T extends BaseTableItem> {
  columns: {
    key: keyof T;
    label: string;
    searchable?: boolean;
    formatter?: (value: T[keyof T]) => string;
  }[];
}

export interface UserData {
  user: User;
  profile: Profile;
}

export interface Word {
  word: string;
  results: WordResult[];
}

export interface WordResult {
  definition: string;
}

export interface Deck {
  id: string;
  name: string;
  description?: string;
  assignee: string;
  visibility: "public" | "private" | "assigned";
  created_by: string;
  created_at: string;
}

export interface Card {
  id: string;
  front: string;
  back: string;
  media_url?: string;
  deck_id: string;
  created_at?: string;
}

export interface DeckWithCards {
  deck: Deck;
  cards: Card[];
  dueToday: number;
  totalReviewed: number;
}
export interface CardProgress {
  id: string;
  card_id: string;
  user_id: string;
  review_count: number;
  last_reviewed: string | null;
  due_date: string;
  ease_factor: number;
  interval: number;
  front: string;
  back: string;
}

export interface PaginatedResponse<T> {
  data: Vec<T>;
  total: number;
  page: number;
  per_page: number;
}
