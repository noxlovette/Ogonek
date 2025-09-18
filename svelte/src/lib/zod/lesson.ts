import z from "zod";

export const updateLessonBody = z.object({
  assignee: z.string().nullish(),
  createdBy: z.string().nullish(),
  id: z.string().nullish(),
  markdown: z.string().nullish(),
  mediaUrl: z.string().nullish(),
  title: z.string().nullish(),
  topic: z.string().nullish(),
});
