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

export type UrgencyLevel = "overdue" | "urgent" | "soon" | "normal";

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
  dueDate: string;
  completed: boolean;
  priority: number;
  assigneeName: string;
  seen: boolean;
}

interface BadgeWrapper<T> {
  data: T[];
  count: number;
}

export interface DashboardData {
  students: Student[];
  user: User;
  lessons: BadgeWrapper<LessonSmall>;
  tasks: BadgeWrapper<TaskSmall>;
  decks: BadgeWrapper<DeckSmall>;
  profile: ProfileComposite;
}
export interface Toast {
  message: string | null;
  type: "success" | "error" | "info" | null;
}
export interface Profile {
  userId: string;
  videoCallUrl: string | null;
  avatarUrl: string | null;
  telegramId: string | null;
  [key: string]: string | undefined | null;
}

export interface TeacherData {
  teacherVideoCallUrl: string;
  teacherTelegramId: string;
}

export interface ProfileComposite {
  profile: Profile;
  teacherData: TeacherData | null;
}

export interface User {
  name: string | null;
  username: string | null;
  role: string | null;
  email: string | null;
  id: string | null;
  [key: string]: string | undefined | null;
}

export interface Lesson {
  id: string;
  createdAt: string;
  updatedAt: string;
  assignee: string;
  topic: string;
  title: string;
  assigneeName: string;
  markdown: string;
}

export interface LessonSmall {
  id: string;
  createdAt: string;
  title: string;
  topic: string;
  assigneeName: string;
  seen: boolean;
}

export interface Student {
  id: string;
  name: string;
  username: string;
  email: string;
  markdown: string | null;
  studentTelegramId: string | null;
}

export interface CompositeStudent {
  student: Student;
  tasks: TaskSmall[];
  lessons: LessonSmall[];
  decks: DeckSmall[];
}

export interface DeckSmall {
  id: string;
  isSubscribed?: boolean;
  name: string;
  description: string;
  visibility: "private" | "assigned" | "public";
  seen: boolean;
  assigneeName: string;
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
  isSubscribed: boolean;
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

export interface ImportWord {
  front: string;
  back: string;
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
