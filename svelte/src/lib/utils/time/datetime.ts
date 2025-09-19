type DateFormatOptions = {
  year?: boolean;
  month?: "numeric" | "2-digit" | "long" | "short" | "narrow";
  day?: boolean;
  hour?: boolean;
  minute?: boolean;
  second?: boolean;
  hour12?: boolean;
  timeZone?: string;
  locale?: string;
};

/**
 * Gets the locale from the PARAGLIDE_LOCALE cookie.
 */
export const getLocaleFromCookie = (): string => {
  if (typeof document === "undefined") return "en";

  const value = `; ${document.cookie}`;
  const parts = value.split(`; PARAGLIDE_LOCALE=`);
  if (parts.length === 2) {
    const langCode = parts.pop()!.split(";").shift() || "en";
    // Convert language codes to proper locale identifiers
    const localeMap: Record<string, string> = {
      en: "en-UK",
      de: "de-DE",
      ru: "ru-RU",
      fr: "fr-FR",
    };
    return localeMap[langCode] || `${langCode}-${langCode.toUpperCase()}`;
  }
  return "en-UK";
};

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

export const formatDateOnly = (
  dateInput: string | Date,
  locale?: string,
): string => {
  return formatDate(dateInput, { month: "short", day: true }, locale);
};

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
