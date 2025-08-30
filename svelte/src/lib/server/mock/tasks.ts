import type { FileSmall, TaskFull, TaskSmall, TaskWithFiles } from "$lib/types";
import { daysAgo, defaultTaskDueDate, tomorrow, yesterday } from "$lib/utils";
import { nanoid } from "nanoid";

export function createMockTaskSmall(
  overrides: Partial<TaskSmall> = {},
): TaskSmall {
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

export function createMockTaskFull(
  overrides: Partial<TaskFull> = {},
): TaskFull {
  const base = createMockTaskSmall(overrides);

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

export function createMockFileSmall(
  overrides: Partial<FileSmall> = {},
): FileSmall {
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

export function createMockTaskWithFiles(
  taskOverrides: Partial<TaskFull> = {},
  filesCount: number = 2,
): TaskWithFiles {
  const task = createMockTaskFull(taskOverrides);
  const files = Array.from({ length: filesCount }, (_, i) =>
    createMockFileSmall({
      name: `attachment_${i + 1}.pdf`,
      size: Math.floor(Math.random() * 1024 * 100) + 1024, // 1KB to 100KB
    }),
  );

  return { task, files };
}

// Batch generators for when you need arrays
export function generateMockTasks(
  count: number,
  overrides: Partial<TaskFull> = {},
): TaskFull[] {
  return Array.from({ length: count }, (_, i) =>
    createMockTaskFull({
      title: `Task ${i + 1}`,
      priority: Math.floor(Math.random() * 3) + 1,
      completed: Math.random() > 0.7,
      ...overrides,
    }),
  );
}

export function generateMockTasksSmall(
  count: number,
  overrides: Partial<TaskSmall> = {},
): TaskSmall[] {
  return Array.from({ length: count }, (_, i) =>
    createMockTaskSmall({
      title: `Task ${i + 1}`,
      priority: Math.floor(Math.random() * 3) + 1,
      completed: Math.random() > 0.7,
      ...overrides,
    }),
  );
}

// Realistic scenario builders
export const MockScenarios = {
  // High priority overdue tasks
  urgentTasks: (count: number = 3) =>
    generateMockTasks(count, {
      priority: 1,
      completed: false,
      dueDate: daysAgo(2),
    }),

  // Completed recent tasks
  recentCompletions: (count: number = 5) =>
    generateMockTasks(count, {
      completed: true,
      updatedAt: yesterday(),
    }),

  // Mixed priority upcoming tasks
  upcomingTasks: (count: number = 10) =>
    generateMockTasks(count, {
      completed: false,
      dueDate: tomorrow(),
    }),

  // Student specific tasks
  studentTasks: (studentId: string, count: number = 5) =>
    generateMockTasks(count, {
      assignee: studentId,
      assigneeName: `Student ${studentId.slice(-4)}`,
    }),
};

// Usage examples:
// const singleTask = createMockTaskFull({ title: "Custom Task", priority: 1 });
// const taskList = generateMockTasks(10);
// const urgentStuff = MockScenarios.urgentTasks(3);
// const taskWithAttachments = createMockTaskWithFiles({ title: "Assignment" }, 3);
