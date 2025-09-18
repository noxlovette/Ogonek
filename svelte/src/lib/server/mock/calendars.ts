import type {
  Calendar,
  CalendarEvent,
  EventAttendee,
} from "$lib/types/api/calendar";
import { faker } from "@faker-js/faker";
import { nanoid } from "nanoid";

export function createCalendar(): Calendar {
  return {
    id: nanoid(),
    name: faker.company.name() + " Calendar",
    description: faker.lorem.sentence(),
    colour: faker.color.rgb(),
  };
}

export function createCalendarEvent(): CalendarEvent {
  const startDate = faker.date.between({
    from: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000), // 30 days ago
    to: new Date(Date.now() + 60 * 24 * 60 * 60 * 1000), // 60 days from now
  });

  const endDate = new Date(
    startDate.getTime() + faker.number.int({ min: 30, max: 240 }) * 60 * 1000,
  ); // 30min to 4hr duration
  const allDay = faker.datatype.boolean({ probability: 0.2 });

  return {
    uid: nanoid(),
    summary: faker.helpers.arrayElement([
      "Team Meeting",
      "Project Review",
      "Client Call",
      "Workshop",
      "Training Session",
      "Standup",
      "Demo",
      "Planning Session",
      "Retrospective",
      "All Hands",
    ]),
    description: faker.lorem.paragraph(),
    dtstart: startDate.toISOString(),
    dtend: allDay ? null : endDate.toISOString(),
    allDay,
    location: faker.helpers.arrayElement([
      "https://us06web.zoom.us/j/3661071003?pwd=RTlrUkRPaHJaakljZXQxaGpOYmdIZz09",
      "Client Site",
      null,
    ]),
    status: faker.helpers.arrayElement(["confirmed", "tentative", "cancelled"]),
    class: faker.helpers.arrayElement(["public", "private", "confidential"]),
    priority: faker.helpers.arrayElement([1, 2, 3, 4, 5, null]),
    sequence: faker.number.int({ min: 0, max: 10 }),
    etag: nanoid(),
    organiserName: faker.person.fullName(),
    organiserEmail: faker.internet.email(),
    categories: faker.helpers.maybe(() =>
      faker.helpers.multiple(
        () =>
          faker.helpers.arrayElement([
            "work",
            "personal",
            "meeting",
            "travel",
            "training",
            "social",
          ]),
        { count: { min: 1, max: 3 } },
      ),
    ),
    timezone: faker.helpers.arrayElement([
      "UTC",
      "America/New_York",
      "Europe/London",
      "Asia/Tokyo",
      null,
    ]),
    transp: faker.helpers.arrayElement(["opaque", "transparent"]),
    url: faker.helpers.maybe(() => faker.internet.url()),
    rrule: faker.helpers.maybe(() => "FREQ=WEEKLY;BYDAY=MO,WE,FR"),
    rdate: null,
    exdate: null,
    recurrenceId: null,
  };
}

export function createEventAttendee(): EventAttendee {
  return {
    id: nanoid(),
    email: faker.internet.email(),
    name: faker.person.fullName(),
    role: faker.helpers.arrayElement([
      "req-participant",
      "chair",
      "opt-participant",
      "non-participant",
    ]),
    status: faker.helpers.arrayElement([
      "needs-action",
      "accepted",
      "declined",
      "tentative",
      "delegated",
    ]),
  };
}
