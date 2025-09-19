import type { CalendarEvent } from "$lib/types/api/calendar";

export type MonthDay = {
  day: number;
  isCurrentMonth: boolean;
  isToday: boolean;
  events: CalendarEvent[];
  actualDate: Date;
};

export const createDateKey = (date: Date): string =>
  `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;

export const groupEventsByDate = (
  events: CalendarEvent[],
): Record<string, CalendarEvent[]> => {
  return events.reduce(
    (acc, event) => {
      const eventDate = new Date(event.dtstart);
      const dateKey = createDateKey(eventDate);
      if (!acc[dateKey]) acc[dateKey] = [];
      acc[dateKey].push(event);
      return acc;
    },
    {} as Record<string, CalendarEvent[]>,
  );
};

export const isSameDay = (date1: Date, date2: Date): boolean =>
  date1.getFullYear() === date2.getFullYear() &&
  date1.getMonth() === date2.getMonth() &&
  date1.getDate() === date2.getDate();

// SINGLE function that does what your original code did - no fancy breakdown
export const generateMonthDays = (
  referenceDate: Date,
  events: CalendarEvent[],
  locale: string = "en-US",
): { monthDays: MonthDay[]; monthName: string; year: number } => {
  const now = referenceDate;
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  // Quick event grouping
  const eventsByDate = groupEventsByDate(events);
  const getEventsForDay = (day: number, month: number, year: number) => {
    const dateKey = `${year}-${month}-${day}`;
    return eventsByDate[dateKey] || [];
  };

  const monthDays: MonthDay[] = [];
  const startDayOfWeek = firstDay.getDay();
  const prevMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1);
  const prevMonthLastDay = new Date(
    now.getFullYear(),
    now.getMonth(),
    0,
  ).getDate();

  // Previous month overflow
  for (let i = 0; i < startDayOfWeek; i++) {
    const dayNum = prevMonthLastDay - startDayOfWeek + i + 1;
    monthDays.push({
      day: dayNum,
      isCurrentMonth: false,
      isToday: false,
      events: [],
      actualDate: new Date(
        prevMonth.getFullYear(),
        prevMonth.getMonth(),
        dayNum,
      ),
    });
  }

  // Current month days
  for (let day = 1; day <= lastDay.getDate(); day++) {
    const actualDate = new Date(
      firstDay.getFullYear(),
      firstDay.getMonth(),
      day,
    );
    monthDays.push({
      day,
      isCurrentMonth: true,
      isToday: isSameDay(actualDate, now),
      events: getEventsForDay(day, firstDay.getMonth(), firstDay.getFullYear()),
      actualDate,
    });
  }

  // Next month overflow
  const remainingCells = 42 - monthDays.length;
  const nextMonth = new Date(now.getFullYear(), now.getMonth() + 1, 1);
  for (let i = 1; i <= remainingCells; i++) {
    const actualDate = new Date(
      nextMonth.getFullYear(),
      nextMonth.getMonth(),
      i,
    );
    monthDays.push({
      day: i,
      isCurrentMonth: false,
      isToday: false,
      events: getEventsForDay(i, nextMonth.getMonth(), nextMonth.getFullYear()),
      actualDate,
    });
  }

  return {
    monthDays,
    monthName: firstDay.toLocaleDateString(locale, { month: "long" }),
    year: firstDay.getFullYear(),
  };
};
