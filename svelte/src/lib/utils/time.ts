// src/lib/utils/time.ts

/**
 * Clean time utilities to avoid `new Date()` scattered everywhere
 * All dates return ISO strings for consistency with your API
 */

const MS_PER_DAY = 24 * 60 * 60 * 1000;
const MS_PER_HOUR = 60 * 60 * 1000;
const MS_PER_WEEK = 7 * MS_PER_DAY;

// Core date constants - always fresh when imported
export const now = () => new Date();
export const nowISO = () => new Date().toISOString();
export const nowTimestamp = () => Date.now();

// Common date shortcuts
export const today = () => new Date().toISOString();
export const yesterday = () => new Date(Date.now() - MS_PER_DAY).toISOString();
export const tomorrow = () => new Date(Date.now() + MS_PER_DAY).toISOString();

// Relative date builders
export const daysFromNow = (days: number) =>
  new Date(Date.now() + days * MS_PER_DAY).toISOString();

export const daysAgo = (days: number) =>
  new Date(Date.now() - days * MS_PER_DAY).toISOString();

export const weeksFromNow = (weeks: number) =>
  new Date(Date.now() + weeks * MS_PER_WEEK).toISOString();

export const weeksAgo = (weeks: number) =>
  new Date(Date.now() - weeks * MS_PER_WEEK).toISOString();

export const hoursFromNow = (hours: number) =>
  new Date(Date.now() + hours * MS_PER_HOUR).toISOString();

export const hoursAgo = (hours: number) =>
  new Date(Date.now() - hours * MS_PER_HOUR).toISOString();

// Date input helpers (for HTML date inputs)
export const todayDateInput = () => new Date().toISOString().split("T")[0];
export const tomorrowDateInput = () =>
  new Date(Date.now() + MS_PER_DAY).toISOString().split("T")[0];
export const dateInputFromISO = (isoString: string) =>
  new Date(isoString).toISOString().split("T")[0];

// Flexible date builder
export const customDate = (options: {
  year?: number;
  month?: number; // 1-12 (not 0-11)
  day?: number;
  hours?: number;
  minutes?: number;
  seconds?: number;
}) => {
  const date = new Date();

  if (options.year !== undefined) date.setFullYear(options.year);
  if (options.month !== undefined) date.setMonth(options.month - 1); // Convert to 0-11
  if (options.day !== undefined) date.setDate(options.day);
  if (options.hours !== undefined) date.setHours(options.hours);
  if (options.minutes !== undefined) date.setMinutes(options.minutes);
  if (options.seconds !== undefined) date.setSeconds(options.seconds);

  return date.toISOString();
};

// Random date generators for realistic mock data
export const randomDateBetween = (startDate: string, endDate: string) => {
  const start = new Date(startDate).getTime();
  const end = new Date(endDate).getTime();
  const randomTime = start + Math.random() * (end - start);
  return new Date(randomTime).toISOString();
};

export const randomRecentDate = (maxDaysAgo: number = 30) =>
  randomDateBetween(daysAgo(maxDaysAgo), today());

export const randomFutureDate = (maxDaysFromNow: number = 30) =>
  randomDateBetween(today(), daysFromNow(maxDaysFromNow));

// Academic calendar helpers (since you're building an education app)
export const startOfSemester = () => customDate({ month: 9, day: 1 }); // Sept 1st
export const endOfSemester = () => customDate({ month: 12, day: 15 }); // Dec 15th
export const midtermWeek = () => customDate({ month: 10, day: 15 }); // Oct 15th
export const finalsWeek = () => customDate({ month: 12, day: 8 }); // Dec 8th

// Task-specific helpers matching your domain
export const defaultTaskDueDate = () => daysFromNow(7); // Week from now
export const urgentDueDate = () => daysFromNow(2); // 2 days
export const relaxedDueDate = () => daysFromNow(14); // 2 weeks

// Common time ranges for queries
export const timeRanges = {
  thisWeek: {
    start: daysAgo(new Date().getDay()), // Start of current week
    end: daysFromNow(7 - new Date().getDay()), // End of current week
  },
  thisMonth: {
    start: customDate({ day: 1 }),
    end: customDate({ month: new Date().getMonth() + 2, day: 0 }), // Last day of current month
  },
  lastMonth: {
    start: customDate({ month: new Date().getMonth(), day: 1 }),
    end: customDate({ day: 0 }), // Last day of previous month
  },
};

// Utility for consistent date formatting
export const formatters = {
  // For display in UI
  displayDate: (isoString: string) => new Date(isoString).toLocaleDateString(),
  displayDateTime: (isoString: string) => new Date(isoString).toLocaleString(),

  // For API/database consistency
  toUTC: (date: Date) => date.toISOString(),

  // For form inputs
  toDateInput: (isoString: string) =>
    new Date(isoString).toISOString().split("T")[0],

  // Relative time (basic implementation)
  timeAgo: (isoString: string) => {
    const diffMs = Date.now() - new Date(isoString).getTime();
    const diffDays = Math.floor(diffMs / MS_PER_DAY);

    if (diffDays === 0) return "Today";
    if (diffDays === 1) return "Yesterday";
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
    return `${Math.floor(diffDays / 30)} months ago`;
  },
};

// Quick sanity check exports for debugging
export const debug = {
  allConstants: () => ({
    today: today(),
    tomorrow: tomorrow(),
    yesterday: yesterday(),
    weekFromNow: daysFromNow(7),
    weekAgo: daysAgo(7),
  }),

  // Test if a date is in the past/future
  isPast: (isoString: string) => new Date(isoString).getTime() < Date.now(),
  isFuture: (isoString: string) => new Date(isoString).getTime() > Date.now(),
  isToday: (isoString: string) => {
    const date = new Date(isoString);
    const today = new Date();
    return date.toDateString() === today.toDateString();
  },
};
