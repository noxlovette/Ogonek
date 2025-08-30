export interface ActivityLog {
  modelType: string;
  modelId: string;
  action: string;
  createdAt: string;
}

export interface Deck {
  id: string;
  title: string;
  description?: string;
  assignee: string;
  isSubscribed: boolean;
  visibility: "public" | "private" | "assigned";
  createdBy: string;
  createdAt: string;
}
export interface UserPreferences {
  auto_subscribe: boolean;
  email_notifications: boolean;
  push_notifications: boolean;
  theme: "light" | "dark" | "system";
  language: "en" | "ru" | "fr" | "de" | "it";
}

export interface AppContext {
  user: User;
  profile: Profile;
  callUrl: string | null;
  students: Student[];
  preferences: UserPreferences;
}

export interface DashboardData {
  tasks: TaskSmall[];
  lessons: LessonSmall[];
  decks: DeckSmall[];
  activity: ActivityLog[];
  learnData: SimpleStats;
}

export interface NotificationBadges {
  unseenTasks: number;
  unseenLessons: number;
  unseenDecks: number;
  dueCards?: number;
}

export interface CompositeStudent {
  student: Student;
  tasks: TaskSmall[];
  lessons: LessonSmall[];
  decks: DeckSmall[];
}

export interface UserData {
  user: User;
  profile: Profile;
}

export interface DeckWithCards {
  deck: Deck;
  cards: Card[];
  isSubscribed: boolean;
}

export interface PaginatedResponse<T> {
  data: Vec<T>;
  total: number;
  page: number;
  perPage: number;
}

export interface LearnDataDashboard {
  dueCards: number;
  stats: SimpleStats;
}

export interface SimpleStats {
  cardsStudiedToday: number;
  currentStreak: number;
}
