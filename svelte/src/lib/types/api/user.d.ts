export interface Profile {
  userId: string;
  videoCallUrl: string | null;
  avatarUrl: string | null;
  telegramId: string | null;
  [key: string]: string | undefined | null;
}

export interface User {
  name: string | null;
  username: string | null;
  role: string | null;
  email: string | null;
  id: string | null;
  [key: string]: string | undefined | null;
}

export interface Student {
  id: string;
  name: string;
  username: string;
  email: string;
  markdown: string | null;
  studentTelegramId: string | null;
}
