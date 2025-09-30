import z from "zod";

export const updateLessonBody = z.object({
  assignee: z.string().nullable(),
  unassign: z.boolean().nullable(),
  markdown: z.string().nullish(),
  title: z.string().nullish(),
  topic: z.string().nullish(),
});
