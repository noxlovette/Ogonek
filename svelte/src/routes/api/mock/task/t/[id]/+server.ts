import type { TaskWithFiles } from "$lib/types";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ params }) => {
  let taskWithFiles: TaskWithFiles | null = null;

  switch (params.id) {
    case "task1":
      taskWithFiles = {
        task: {
          title: "Water the plants",
          assignee: "1",
          assigneeName: "Michael",
          markdown: "A task",
          createdBy: "",
          priority: 3,
          completed: false,
          createdAt: "2025-07-03T10:36:53.047197Z",
          updatedAt: "2025-07-03T14:06:58.038355Z",
          id: "task1",
          dueDate: "2025-07-15T23:59:59Z",
        },
        files: [
          {
            id: "file1",
            name: "File 1",
            s3Key: "tasks/Ng4Dj62hTanaxauGWFrh1/6EiOJ67gIihi-mRXoQIed.vtt",
            mimeType: "text/vtt",
            size: 2516,
            ownerId: "user1",
          },
        ],
      };

      break;

    case "task2":
      taskWithFiles = {
        task: {
          title: "Water the plants",
          assignee: "1",
          assigneeName: "Michael",
          markdown: "A task",
          createdBy: "",
          priority: 3,
          completed: false,
          createdAt: "2025-07-03T10:36:53.047197Z",
          updatedAt: "2025-07-03T14:06:58.038355Z",
          id: "task2",
          dueDate: "2025-07-15T23:59:59Z",
        },
        files: [
          {
            id: "file1",
            name: "File 1",
            s3Key: "tasks/Ng4Dj62hTanaxauGWFrh1/6EiOJ67gIihi-mRXoQIed.vtt",
            mimeType: "text/vtt",
            size: 2516,
            ownerId: "user1",
          },
        ],
      };

      break;
  }

  if (!taskWithFiles) {
    return new Response("Task Not Found", { status: 404 });
  }

  return new Response(JSON.stringify(taskWithFiles), {
    headers: { "Content-Type": "application/json" },
  });
};

export const PATCH: RequestHandler = async () => {
  return new Response(null, { status: 204 });
};

export const DELETE: RequestHandler = async () => {
  return new Response(null, { status: 204 });
};
