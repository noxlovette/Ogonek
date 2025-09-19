type DayMapping = {
  label: string;
  full: string;
  ical: string;
  index: number;
};

export const WEEKDAYS: DayMapping[] = [
  { label: "S", full: "Sunday", ical: "SU", index: 0 },
  { label: "M", full: "Monday", ical: "MO", index: 1 },
  { label: "T", full: "Tuesday", ical: "TU", index: 2 },
  { label: "W", full: "Wednesday", ical: "WE", index: 3 },
  { label: "T", full: "Thursday", ical: "TH", index: 4 },
  { label: "F", full: "Friday", ical: "FR", index: 5 },
  { label: "S", full: "Saturday", ical: "SA", index: 6 },
];

export const parseRRuleDays = (
  rrule: string | null | undefined,
  days: DayMapping[] = WEEKDAYS,
): number[] => {
  if (!rrule) return [];

  const byDayMatch = rrule.match(/BYDAY=([^;]+)/);
  if (!byDayMatch) return [];

  const ruleDays = byDayMatch[1].split(",");
  return days
    .filter((day) => ruleDays.includes(day.ical))
    .map((day) => day.index);
};
