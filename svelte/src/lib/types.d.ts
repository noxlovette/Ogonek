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

export interface Profile {
    quizletUrl: string,
    zoomUrl: string,
    bio: string,
    avatarUrl: string,
}

export interface User {
    name: string;
    username: string;
    role: string;
    email: string;
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
}

export interface BaseTableItem {
    id: string;
}

export interface TableConfig<T extends BaseTableItem> {
    columns: {
        key: keyof T;
        label: string;
        searchable?: boolean;
        formatter?: (value: string) => string;
    }[];
}