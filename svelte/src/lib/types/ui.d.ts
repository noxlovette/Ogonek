export type UrgencyLevel = "overdue" | "urgent" | "soon" | "normal";

export type Role = "t" | "s";

export interface Toast {
  message: string | null;
  type: "success" | "error" | "info" | null;
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

export interface Word {
  word: string;
  results: WordResult[];
}

export interface WordResult {
  definition: string;
}

export interface ImportWord {
  front: string;
  back: string;
}
