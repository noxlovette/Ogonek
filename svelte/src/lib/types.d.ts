export interface Task {
    id: string;
    title: string;
    content: string;
    priority: int;
    createdAt: string;
    updatedAt: string;
    status: string;
    dueDate: string;
    completed: boolean;
    file: string;
}

export interface User {
    id: string;
    email: string;
    username: string;
    is_authenticated: boolean;
    csrfToken: string;
    quizlet_url: string;
}

export interface Lesson {
    id: string;
    manual_date?: string;
    title: string;
    content: string;
    created_at: string;
    updated_at: string;
    topic: string;
    bookmarked: boolean;
}