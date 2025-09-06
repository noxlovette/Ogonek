import type { FileSmall, TaskFull, TaskSmall, TaskWithFiles } from "$lib/types";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";

export function createTaskSmall(): TaskSmall {
  return {
    id: nanoid(),
    title: faker.book.title(),
    dueDate: faker.date.soon().toISOString(),
    completed: faker.datatype.boolean(),
    priority: faker.number.int({ min: 1, max: 3 }),
    assigneeName: faker.person.fullName(),
    seen: faker.datatype.boolean(),
  };
}

export function createTaskFull(): TaskFull {
  const base = createTaskSmall();

  return {
    ...base,
    markdown: faker.lorem.text(),
    createdAt: faker.date.past().toISOString(),
    updatedAt: faker.date.recent().toISOString(),
    createdBy: nanoid(),
    assignee: nanoid(),
  };
}

export function createFileSmall(): FileSmall {
  return {
    id: nanoid(),
    name: faker.system.fileName(),
    size: faker.number.int(),
    mimeType: faker.system.mimeType(),
    s3Key: `tasks/${nanoid()}/${nanoid()}.pdf`,
    ownerId: nanoid(),
  };
}

export function createTaskWithFiles(): TaskWithFiles {
  const task = createTaskFull();
  const files = faker.helpers.multiple(createFileSmall, {
    count: faker.number.int({ min: 0, max: 5 }),
  });

  return { task, files };
}
