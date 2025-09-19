/// Formats the day event for the day view
export function formatEventTime(
  dtstart: string,
  dtend?: string,
  isAllDay?: boolean,
): string {
  if (isAllDay) return "All day";

  const start = new Date(dtstart);
  const end = dtend ? new Date(dtend) : null;

  const timeFormat = new Intl.DateTimeFormat("en-US", {
    hour: "numeric",
    minute: "2-digit",
    hour12: true,
  });

  if (!end) return timeFormat.format(start);

  // Same day event
  if (start.toDateString() === end.toDateString()) {
    return `${timeFormat.format(start)} - ${timeFormat.format(end)}`;
  }

  // Multi-day event
  return `${timeFormat.format(start)} - ${end.toLocaleDateString()}`;
}
