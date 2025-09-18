import z from "zod";

export const updateEventBody = z.object({
  description: z.string().nullish(),
  dtend: z.iso.datetime({}).nullish(),
  dtstart: z.iso.datetime({}).nullish(),
  location: z.string().nullish(),
  rrule: z.string().nullish(),
  timezone: z.string().nullish(),
  attendee: z.string().nullish(),
});
