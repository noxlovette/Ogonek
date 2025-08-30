import type { FileSmall, TaskFull, TaskSmall, TaskWithFiles } from "$lib/types";
import { daysAgo, defaultTaskDueDate, tomorrow, yesterday } from "$lib/utils";
import { nanoid } from "nanoid";

export function createTaskSmall(overrides: Partial<TaskSmall> = {}): TaskSmall {
  const dueDate = defaultTaskDueDate();

  return {
    id: nanoid(),
    title: "Sample Task",
    dueDate,
    completed: false,
    priority: 2,
    assigneeName: "John Doe",
    seen: true,
    ...overrides,
  };
}

export function createTaskFull(overrides: Partial<TaskFull> = {}): TaskFull {
  const base = createTaskSmall(overrides);

  return {
    ...base,
    markdown:
      "## Task Description\n\nThis is a sample task with markdown content.",
    createdAt: daysAgo(14),
    updatedAt: daysAgo(1),
    createdBy: "teacher_" + nanoid(8),
    assignee: "student_" + nanoid(8),
    ...overrides,
  };
}

export function createFileSmall(overrides: Partial<FileSmall> = {}): FileSmall {
  let id = nanoid();

  return {
    id: id,
    name: "sample_file.pdf",
    size: 1024 * 50, // 50KB
    mimeType: "application/pdf",
    s3Key: `tasks/user1/${id}.pdf`,
    ownerId: "user1",
    ...overrides,
  };
}

export function createTaskWithFiles(
  taskOverrides: Partial<TaskFull> = {},
  filesCount: number = 2,
): TaskWithFiles {
  const task = createTaskFull(taskOverrides);
  const files = Array.from({ length: filesCount }, (_, i) =>
    createFileSmall({
      name: `attachment_${i + 1}.pdf`,
      size: Math.floor(Math.random() * 1024 * 100) + 1024, // 1KB to 100KB
    }),
  );

  return { task, files };
}

// Batch generators for when you need arrays
export function createTasks(
  count: number,
  overrides: Partial<TaskFull> = {},
): TaskFull[] {
  return Array.from({ length: count }, (_, i) =>
    createTaskFull({
      title: `Task ${i + 1}`,
      priority: Math.floor(Math.random() * 3) + 1,
      completed: Math.random() > 0.7,
      ...overrides,
    }),
  );
}

export function createTasksSmall(
  count: number = 4,
  overrides: Partial<TaskSmall> = {},
): TaskSmall[] {
  return Array.from({ length: count }, (_, i) =>
    createTaskSmall({
      title: `Task ${i + 1}`,
      priority: Math.floor(Math.random() * 3) + 1,
      completed: Math.random() > 0.7,
      ...overrides,
    }),
  );
}

// Realistic scenario builders
export const Scenarios = {
  // High priority overdue tasks
  urgentTasks: (count: number = 3) =>
    createTasks(count, {
      priority: 1,
      completed: false,
      dueDate: daysAgo(2),
    }),

  // Completed recent tasks
  recentCompletions: (count: number = 5) =>
    createTasks(count, {
      completed: true,
      updatedAt: yesterday(),
    }),

  // Mixed priority upcoming tasks
  upcomingTasks: (count: number = 10) =>
    createTasks(count, {
      completed: false,
      dueDate: tomorrow(),
    }),

  // Student specific tasks
  studentTasks: (studentId: string, count: number = 5) =>
    createTasks(count, {
      assignee: studentId,
      assigneeName: `Student ${studentId.slice(-4)}`,
    }),
};

// Usage examples:
// const singleTask = createTaskFull({ title: "Custom Task", priority: 1 });
// const taskList = createTasks(10);
// const urgentStuff = Scenarios.urgentTasks(3);
// const taskWithAttachments = createTaskWithFiles({ title: "Assignment" }, 3);
