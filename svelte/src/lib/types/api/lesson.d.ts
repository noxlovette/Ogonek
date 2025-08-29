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
