export interface TaskWithFiles {
  task: TaskFull;
  files: FileSmall[];
}
interface TaskSmall {
  id: string;
  title: string;
  dueDate: string;
  completed: boolean;
  priority: number;
  assigneeName: string;
  seen: boolean;
}

interface TaskFull extends TaskSmall {
  markdown: string;
  createdAt: string;
  updatedAt: string;
  createdBy: string;
  assignee: string;
}

function createMockTask(): TaskFull {
  const now = Date.now();
  return {
    id: "1",
    title: "Mock Task",
    dueDate: new Date(now + 7 * 24 * 60 * 60 * 1000).toISOString(),
    completed: false,
    priority: 2,
    seen: false,
    markdown: "Default Markdown",
    createdAt: new Date(now - 14 * 24 * 60 * 60 * 1000).toISOString(),
    updatedAt: new Date(now - 12 * 24 * 60 * 60 * 1000).toISOString(),
    createdBy: "teacher1",
    assignee: "student1",
    assigneeName: "Student One",
  };
}
