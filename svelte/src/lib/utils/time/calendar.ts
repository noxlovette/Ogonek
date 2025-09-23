// generateMonthDays - Version corrigée
import type { EventSmall } from "$lib/types/api/calendar";

export type MonthDay = {
  day: number;
  isCurrentMonth: boolean;
  isToday: boolean;
  events: EventSmall[];
  actualDate: Date;
};

export const createDateKey = (date: Date): string =>
  `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;

export const groupEventsByDate = (
  events: EventSmall[],
): Record<string, EventSmall[]> => {
  return events.reduce(
    (acc, event) => {
      const eventDate = new Date(event.dtstartTime);
      const dateKey = createDateKey(eventDate);
      if (!acc[dateKey]) acc[dateKey] = [];
      acc[dateKey].push(event);
      return acc;
    },
    {} as Record<string, EventSmall[]>,
  );
};

export const isSameDay = (date1: Date, date2: Date): boolean =>
  date1.getFullYear() === date2.getFullYear() &&
  date1.getMonth() === date2.getMonth() &&
  date1.getDate() === date2.getDate();

// Helper pour convertir dimanche (0) vers lundi (0)
const getMondayBasedDayOfWeek = (date: Date): number => {
  const dayOfWeek = date.getDay();
  return dayOfWeek === 0 ? 6 : dayOfWeek - 1; // Dimanche devient 6, lundi devient 0
};

export const generateMonthDays = (
  referenceDate: Date,
  events: EventSmall[],
  locale: string = "ru-RU",
): { monthDays: MonthDay[]; monthName: string; year: number } => {
  // FIX MAJEUR : Utilise la vraie date d'aujourd'hui, pas referenceDate
  const today = new Date();
  const displayMonth = referenceDate;

  const firstDay = new Date(
    displayMonth.getFullYear(),
    displayMonth.getMonth(),
    1,
  );
  const lastDay = new Date(
    displayMonth.getFullYear(),
    displayMonth.getMonth() + 1,
    0,
  );

  // Quick event grouping
  const eventsByDate = groupEventsByDate(events);
  const getEventsForDay = (day: number, month: number, year: number) => {
    const dateKey = `${year}-${month}-${day}`;
    return eventsByDate[dateKey] || [];
  };

  const monthDays: MonthDay[] = [];

  // FIX : Utilise getMondayBasedDayOfWeek au lieu de getDay()
  const startDayOfWeek = getMondayBasedDayOfWeek(firstDay);
  const prevMonth = new Date(
    displayMonth.getFullYear(),
    displayMonth.getMonth() - 1,
    1,
  );
  const prevMonthLastDay = new Date(
    displayMonth.getFullYear(),
    displayMonth.getMonth(),
    0,
  ).getDate();

  // Previous month overflow
  for (let i = 0; i < startDayOfWeek; i++) {
    const dayNum = prevMonthLastDay - startDayOfWeek + i + 1;
    const actualDate = new Date(
      prevMonth.getFullYear(),
      prevMonth.getMonth(),
      dayNum,
    );
    monthDays.push({
      day: dayNum,
      isCurrentMonth: false,
      isToday: isSameDay(actualDate, today), // Utilise today au lieu de displayMonth
      events: getEventsForDay(
        dayNum,
        prevMonth.getMonth(),
        prevMonth.getFullYear(),
      ),
      actualDate,
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
      isToday: isSameDay(actualDate, today), // FIX : today au lieu de displayMonth
      events: getEventsForDay(day, firstDay.getMonth(), firstDay.getFullYear()),
      actualDate,
    });
  }

  // Next month overflow
  const remainingCells = 42 - monthDays.length;
  const nextMonth = new Date(
    displayMonth.getFullYear(),
    displayMonth.getMonth() + 1,
    1,
  );
  for (let i = 1; i <= remainingCells; i++) {
    const actualDate = new Date(
      nextMonth.getFullYear(),
      nextMonth.getMonth(),
      i,
    );
    monthDays.push({
      day: i,
      isCurrentMonth: false,
      isToday: isSameDay(actualDate, today), // Utilise today
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
