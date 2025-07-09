import type { PaginatedResponse, Task } from "$lib/types";

const taskModules = import.meta.glob("./task/t/*.ts");

export const GET = async () => {
  const tasks: TaskSmall[] = [
    {
      id: "task1",
      title: "Murrrrrr",
      priority: 3,
      completed: false,
      assigneeName: "Kotya",
      seen: true,
      dueDate: "2025-07-15T23:59:59Z",
    },
    {
      id: "task2",
      title: "Kotya Assigned",
      priority: 2,
      completed: false,
      assigneeName: "Kotya",
      seen: true,
      dueDate: "2025-07-17T23:59:59Z",
    },
  ];

  const paginatedResponse: PaginatedResponse<Task> = {
    data: tasks,
    perPage: 3,
    total: 10,
    page: 1,
  };

  return new Response(JSON.stringify(paginatedResponse), {
    headers: { "Content-Type": "application/json" },
  });
};
