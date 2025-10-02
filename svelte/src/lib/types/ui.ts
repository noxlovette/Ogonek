export type Urgency = "overdue" | "urgent" | "soon" | "normal" | "green";

export type Role = "t" | "s";

export interface Toast {
  message: string | null;
  type: "success" | "error" | "info" | null;
}

export interface BaseTableItem {
  id: string;
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
