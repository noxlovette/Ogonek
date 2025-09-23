type DayMapping = {
  label: string;
  full: string;
  ical: string;
  index: number;
};
export const WEEKDAYS: DayMapping[] = [
  { label: "В", full: "Воскресенье", ical: "SU", index: 0 },
  { label: "П", full: "Понедельник", ical: "MO", index: 1 },
  { label: "В", full: "Вторник", ical: "TU", index: 2 },
  { label: "С", full: "Среда", ical: "WE", index: 3 },
  { label: "Ч", full: "Четверг", ical: "TH", index: 4 },
  { label: "П", full: "Пятница", ical: "FR", index: 5 },
  { label: "С", full: "Суббота", ical: "SA", index: 6 },
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
