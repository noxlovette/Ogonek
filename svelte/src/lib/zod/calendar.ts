import z from "zod";

export const updateEventBody = z.object({
  description: z.string().nullish(),
  dtendTime: z.iso.datetime({}).nullish(),
  dtstartTime: z.iso.datetime({}).nullish(),
  location: z.string().nullish(),
  rrule: z.string().nullish(),
  scope: z.string(),
  dtendTz: z.string().nullish(),
  dtstartTz: z.string().nullish(),
  attendee: z.string().nullish(),
});

export const createEventBody = z.object({
  attendee: z.string(),
  dtstartTime: z.iso.datetime({}).nullish(),
  dtendTime: z.iso.datetime({}).nullish(),
});
