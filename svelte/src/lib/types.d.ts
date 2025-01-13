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
}

export interface User {
    name: string;
    username: string;
    role: string;
    // quizlet_url: string;
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
    // quizlet_url: string;
}