import { m } from "$lib/paraglide/messages";
import type { TaskFull, Urgency } from "$lib/types";

// File Renders
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 Bytes";
  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

// File Renders
export function getFileExtension(filename: string): string {
  return filename.split(".").pop()?.toUpperCase() || "";
}

// Test and Learn Modes
export function shuffleArray<T>(array: T[]): T[] {
  const newArray = [...array];
  for (let i = newArray.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [newArray[i], newArray[j]] = [newArray[j], newArray[i]];
  }
  return newArray;
}

export function formatPercentage(value: number): number {
  return Math.min(100, Math.max(0, Math.round(value)));
}

/* 
Used in Badge renders
*/
export function getUrgency(task: TaskFull): Urgency {
  const now = new Date();
  const due = new Date(task.dueDate || "");
  const diffDays = Math.ceil(
    (due.getTime() - now.getTime()) / (1000 * 60 * 60 * 24),
  );

  if (diffDays < 1) return "overdue";
  if (diffDays <= 1) return "urgent";
  if (diffDays <= 3) return "soon";
  return "normal";
}
/* 
Used in Test and Learn modes
*/
export const qualityButtons = [
  {
    quality: 0,
    label: m.such_loose_blackbird_offer(),
    color:
      "ring-red-600 dark:ring-red-500/50 hover:bg-red-700/10 dark:hover:rose-900/20 ring",
    key: 1,
  },

  {
    quality: 3,
    label: m.fuzzy_noble_cod_borrow(),
    color:
      "ring-amber-500 dark:ring-amber-500/50 hover:bg-amber-600/10 dark:hover:bg-amber-900/20 ring",
    key: 2,
  },
  {
    quality: 5,
    label: m.slow_vexed_pigeon_boil(),
    color:
      "ring-emerald-500 dark:ring-emerald-500/50 hover:bg-emerald-500/10 dark:hover:bg-emerald-900/20 ring",
    key: 3,
  },
];
