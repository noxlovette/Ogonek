import z from "zod";

export const updateDeckBody = z.object({
  cards: z.array(
    z.object({
      front: z.string().min(1, "Le recto ne peut pas être vide").trim(),
      back: z.string().min(1, "Le verso ne peut pas être vide").trim(),
      id: z.string().nullable(),
      mediaUrl: z.string().nullable(),
    }),
  ),
  deck: z.object({
    assignee: z.string().nullish(),
    description: z.string().nullish(),
    title: z.string().nullish(),
    visibility: z.string().nullish(),
  }),
});
