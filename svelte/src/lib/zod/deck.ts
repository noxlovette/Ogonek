import z from "zod";

export const updateDeckBody = z.object({
  cards: z.array(
    z.object({
      back: z.string(),
      front: z.string(),
      id: z.string().nullish(),
      mediaUrl: z.string().nullish(),
    }),
  ),
  deck: z.object({
    assignee: z.string().nullish(),
    description: z.string().nullish(),
    title: z.string().nullish(),
    visibility: z.string().nullish(),
  }),
});
