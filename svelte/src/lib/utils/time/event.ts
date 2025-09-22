import type { TimeSpan } from "$lib/types/api/calendar";
/// Formats the day event for the day view
export function formatEventTime(
  dtstartTime: string,
  dtendTime?: string,
  isAllDay?: boolean,
): string {
  if (isAllDay) return "Весь день";

  const start = new Date(dtstartTime);
  const end = dtendTime ? new Date(dtendTime) : null;

  const timeFormat = new Intl.DateTimeFormat("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });

  if (!end) return timeFormat.format(start);

  // Same day event
  if (start.toDateString() === end.toDateString()) {
    return `${timeFormat.format(start)} - ${timeFormat.format(end)}`;
  }

  // Multi-day event
  return `${timeFormat.format(start)} - ${end.toLocaleDateString("ru-RU")}`;
}

export function createMonthSpan(year: number, month: number): TimeSpan {
  const start = new Date(year, month - 1, 1); // month-1 car JS commence à 0
  const end = new Date(year, month, 0, 23, 59, 59, 999); // dernier jour du mois

  return {
    start: start.toISOString(),
    end: end.toISOString(),
  };
}

export function createDaySpan(
  year: number,
  month: number,
  day: number,
): TimeSpan {
  const start = new Date(year, month - 1, day, 0, 0, 0, 0);
  const end = new Date(year, month - 1, day, 23, 59, 59, 999);

  return {
    start: start.toISOString(),
    end: end.toISOString(),
  };
}
