import z from "zod";

export const updateTaskBody = z.object({
  assignee: z.string().nullish(),
  completed: z.boolean().nullish(),
  dueDate: z.iso.datetime({}).nullish(),
  markdown: z.string().nullish(),
  title: z.string().nullish(),
});
