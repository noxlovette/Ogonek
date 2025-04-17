export interface Task {
  id: string;
  title: string;
  markdown: string;
  priority: number;
  createdAt: string;
  updatedAt: string;
  dueDate: string;
  completed: boolean;
  filePath: string;
  createdBy: string;
  assignee: string;
  assigneeName: string;
}

export interface TaskWithFiles {
  task: Task;
  files: FileSmall[];
}

export interface FileSmall {
  id: string;
  name: string;
  s3Key: string;
  mimeType?: string;
  size: number;
  ownerId: string;
}

export interface File {
  id: string;
  name: string;
  s3Key: string;
  path: string;
  mimeType?: string;
  size: number;
  isFolder: boolean;
  parentId: string;
  ownerId: string;
  visibility: string;
  createdAt: string;
  updatedAt: string;
}

export interface TaskSmall {
  id: string;
  title: string;
  markdown: string;
  dueDate: string;
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
  [key: string]: string | undefined;
}

export interface ProfileComposite {
  profile: Profile;
  teacher_telegram_id: string;
}

export interface User {
  name: string | null;
  username: string | null;
  role: string | null;
  email: string | null;
  sub: string | null;
  [key: string]: string | undefined;
}

export interface Lesson {
  id: string;
  title: string;
  markdown: string;
  createdAt: string;
  updatedAt: string;
  topic: string;
  assignee: string;
  assigneeName: string;
}

export interface LessonSmall {
  id: string;
  title: string;
  markdown: string;
  topic: string;
  createdAt: string;
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

export interface CompositeStudent {
  student: Student;
  tasks: TaskSmall[];
  lessons: LessonSmall[];
  decks: DeckSmall[];
}

export interface DeckSmall {
  id: string;
  name: string;
  description: string;
}

export interface BaseTableItem {
  id: string;
}

export interface IdResponse {
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
  createdBy: string;
  createdAt: string;
}

export interface Card {
  id: string;
  front: string;
  back: string;
  mediaUrl?: string;
  deckId: string;
  createdAt?: string;
}

export interface DeckWithCards {
  deck: Deck;
  cards: Card[];
  isSubscribed: boolean;
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
  mediaUrl: string;
}

export interface PaginatedResponse<T> {
  data: Vec<T>;
  total: number;
  page: number;
  perPage: number;
}
