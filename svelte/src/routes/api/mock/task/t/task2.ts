import type { File, Task, TaskWithFiles } from "$lib/types";

export const GET = async () => {
  const task: Task = {
    title: "Water the plants",
    assignee: "1",
    assigneeName: "Michael",
    markdown: "A task",
    createdBy: "",
    priority: 3,
    createdAt: "2025-07-03T10:36:53.047197Z",
    updatedAt: "2025-07-03T14:06:58.038355Z",
    completed: false,
    id: "task2",
    dueDate: "2025-07-15T23:59:59Z",
  };

  const files: File[] = [
    {
      id: "file1",
      name: "File 1",
      s3Key: "tasks/Ng4Dj62hTanaxauGWFrh1/6EiOJ67gIihi-mRXoQIed.vtt",
      mimeType: "text/vtt",
      size: 2516,
      ownerId: "user1",
    },
  ];

  const taskWithFiles: TaskWithFiles = {
    task,
    files,
  };

  return new Response(JSON.stringify(taskWithFiles));
};
