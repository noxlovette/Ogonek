/**
 * Configuration options for date and time formatting.
 */
type DateFormatOptions = {
  /**
   * Include year in the formatted output.
   */
  year?: boolean;

  /**
   * Format of month in the output.
   * - 'numeric': Numeric representation (e.g., 3)
   * - '2-digit': Two-digit representation (e.g., 03)
   * - 'long': Full month name (e.g., March)
   * - 'short': Abbreviated month name (e.g., Mar)
   * - 'narrow': Narrow month name (e.g., M)
   */
  month?: "numeric" | "2-digit" | "long" | "short" | "narrow";

  /**
   * Include day in the formatted output.
   */
  day?: boolean;

  /**
   * Include hour in the formatted output.
   */
  hour?: boolean;

  /**
   * Include minute in the formatted output.
   */
  minute?: boolean;

  /**
   * Include second in the formatted output.
   */
  second?: boolean;

  /**
   * Whether to use 12-hour time (true) or 24-hour time (false).
   */
  hour12?: boolean;

  /**
   * Time zone to use for formatting.
   */
  timeZone?: string;

  /**
   * Locale to use for formatting (overrides the default locale).
   */
  locale?: string;
};

/**
 * Gets the locale from the PARAGLIDE_LOCALE cookie.
 */
const getLocaleFromCookie = (): string => {
  if (typeof document === "undefined") return "en";

  const value = `; ${document.cookie}`;
  const parts = value.split(`; PARAGLIDE_LOCALE=`);
  if (parts.length === 2) {
    const langCode = parts.pop()!.split(";").shift() || "en";
    // Convert language codes to proper locale identifiers
    const localeMap: Record<string, string> = {
      en: "en-US",
      de: "de-DE",
      ru: "ru-RU",
      fr: "fr-FR",
    };
    return localeMap[langCode] || `${langCode}-${langCode.toUpperCase()}`;
  }
  return "en-US";
};

/**
 * Formats a date/time with highly configurable options using the Intl.DateTimeFormat API.
 *
 * @param dateInput - The date to format (can be a Date object or a date string)
 * @param options - Configuration options for formatting
 * @param locale - Default locale to use if not specified in options (will use PARAGLIDE_LOCALE cookie if not provided)
 * @returns A formatted date/time string according to the specified options
 *
 * @example
 * // Format with default options (short month and day) - uses locale from cookie
 * formatDate(new Date()); // Returns "6 Mar" (in user's selected language)
 *
 * @example
 * // Format with custom options
 * formatDate(new Date(), {
 *   year: true,
 *   month: 'long',
 *   day: true,
 *   hour: true,
 *   minute: true
 * }); // Returns "March 6, 2025, 2:30 PM" (in user's selected language)
 */
export const formatDate = (
  dateInput: string | Date,
  options: DateFormatOptions = {
    month: "short",
    day: true,
  },
  locale?: string,
): string => {
  const date = typeof dateInput === "string" ? new Date(dateInput) : dateInput;
  const formatOptions: Intl.DateTimeFormatOptions = {};

  if (options.year) formatOptions.year = "numeric";
  if (options.month) formatOptions.month = options.month;
  if (options.day) formatOptions.day = "numeric";
  if (options.hour) formatOptions.hour = "numeric";
  if (options.minute) formatOptions.minute = "numeric";
  if (options.second) formatOptions.second = "numeric";
  if (options.hour12 !== undefined) formatOptions.hour12 = options.hour12;
  if (options.timeZone) formatOptions.timeZone = options.timeZone;

  const finalLocale = options.locale || locale || getLocaleFromCookie();
  return new Intl.DateTimeFormat(finalLocale, formatOptions).format(date);
};

/**
 * Formats a date showing only the month and day (shorthand function).
 *
 * @param dateInput - The date to format (can be a Date object or a date string)
 * @param locale - Locale to use for formatting
 * @returns A formatted date string with month and day only
 *
 * @example
 * formatDateOnly(new Date()); // Returns "6 Mar"
 * formatDateOnly("2025-12-25"); // Returns "25 Dec"
 */
export const formatDateOnly = (
  dateInput: string | Date,
  locale?: string,
): string => {
  return formatDate(dateInput, { month: "short", day: true }, locale);
};

/**
 * Formats a date showing both date and time components (shorthand function).
 *
 * @param dateInput - The date to format (can be a Date object or a date string)
 * @param locale - Locale to use for formatting
 * @returns A formatted date-time string with month, day, hour, and minute
 *
 * @example
 * formatDateTime(new Date()); // Returns "6 Mar, 2:30 PM"
 * formatDateTime("2025-12-25T18:30:00"); // Returns "25 Dec, 6:30 PM"
 */
export const formatDateTime = (
  dateInput: string | Date,
  locale?: string,
): string => {
  return formatDate(
    dateInput,
    { month: "short", day: true, hour: true, minute: true, hour12: true },
    locale,
  );
};

/**
 * Formats a date relative to the current time (e.g., "2 hours ago", "3 days ago").
 *
 * @param dateInput - The date to format (can be a Date object or a date string)
 * @param locale - Locale to use for formatting (currently only affects internal calculations)
 * @returns A human-readable string describing the relative time
 *
 * @example
 * formatRelativeTime(new Date(Date.now() - 5000)); // Returns "just now"
 * formatRelativeTime(new Date(Date.now() - 3600000)); // Returns "1 hour ago"
 * formatRelativeTime(new Date(Date.now() - 86400000 * 2)); // Returns "2 days ago"
 *
 * @remarks
 * This function uses fixed approximations for month and year durations:
 * - A month is approximated as 30 days
 * - A year is approximated as 365 days
 */
export const formatRelativeTime = (dateInput: string | Date): string => {
  const date = typeof dateInput === "string" ? new Date(dateInput) : dateInput;
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  // Define time units in seconds
  const minute = 60;
  const hour = minute * 60;
  const day = hour * 24;
  const week = day * 7;
  const month = day * 30;
  const year = day * 365;

  if (diffInSeconds < minute) {
    return "just now";
  } else if (diffInSeconds < hour) {
    const minutes = Math.floor(diffInSeconds / minute);
    return `${minutes} minute${minutes > 1 ? "s" : ""} ago`;
  } else if (diffInSeconds < day) {
    const hours = Math.floor(diffInSeconds / hour);
    return `${hours} hour${hours > 1 ? "s" : ""} ago`;
  } else if (diffInSeconds < week) {
    const days = Math.floor(diffInSeconds / day);
    return `${days} day${days > 1 ? "s" : ""} ago`;
  } else if (diffInSeconds < month) {
    const weeks = Math.floor(diffInSeconds / week);
    return `${weeks} week${weeks > 1 ? "s" : ""} ago`;
  } else if (diffInSeconds < year) {
    const months = Math.floor(diffInSeconds / month);
    return `${months} month${months > 1 ? "s" : ""} ago`;
  } else {
    const years = Math.floor(diffInSeconds / year);
    return `${years} year${years > 1 ? "s" : ""} ago`;
  }
};
