import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  formatDate,
  formatDateOnly,
  formatDateTime,
  formatRelativeTime,
} from "./datetime";

describe("formatDate", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should format date with default options", () => {
    const date = new Date("2025-01-15T10:00:00Z");
    const result = formatDate(date);
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should handle string date input", () => {
    const result = formatDate("2025-01-15T10:00:00Z");
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should return fallback for null input", () => {
    const result = formatDate(null as any);
    expect(result).toBe("Не понятно");
  });

  it("should return fallback for undefined input", () => {
    const result = formatDate(undefined as any);
    expect(result).toBe("Не понятно");
  });

  it("should return fallback for empty string", () => {
    const result = formatDate("");
    expect(result).toBe("Не понятно");
  });

  it("should return fallback for invalid date string", () => {
    const result = formatDate("invalid-date");
    expect(result).toBe("Не понятно");
  });

  it("should return 'Сегодня' for today in Russian locale", () => {
    const today = new Date("2025-01-15T10:00:00Z");
    const result = formatDate(today, {}, "ru-RU");
    expect(result).toBe("Сегодня");
  });

  it("should return 'Завтра' for tomorrow in Russian locale", () => {
    const tomorrow = new Date("2025-01-16T10:00:00Z");
    const result = formatDate(tomorrow, {}, "ru-RU");
    expect(result).toBe("Завтра");
  });

  it("should return 'Вчера' for yesterday in Russian locale", () => {
    const yesterday = new Date("2025-01-14T10:00:00Z");
    const result = formatDate(yesterday, {}, "ru-RU");
    expect(result).toBe("Вчера");
  });

  it("should not use Russian labels for non-Russian locale", () => {
    const today = new Date("2025-01-15T10:00:00Z");
    const result = formatDate(today, {}, "en-US");
    expect(result).not.toBe("Сегодня");
  });

  it("should format with custom options", () => {
    const date = new Date("2025-01-15T10:30:45Z");
    const result = formatDate(date, {
      year: true,
      month: "long",
      day: true,
      hour: true,
      minute: true,
      hour12: false,
    });
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should handle custom locale parameter", () => {
    const date = new Date("2025-01-15T10:00:00Z");
    const result = formatDate(date, { month: "long" }, "fr-FR");
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should use locale from options.locale if provided", () => {
    const date = new Date("2025-01-15T10:00:00Z");
    const result = formatDate(date, { locale: "de-DE", month: "long" });
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });
});

describe("formatDateOnly", () => {
  it("should format date with short month and day", () => {
    const date = new Date("2025-01-15T10:00:00Z");
    const result = formatDateOnly(date);
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should pass through locale parameter", () => {
    const date = new Date("2025-01-15T10:00:00Z");
    const result = formatDateOnly(date, "fr-FR");
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });
});

describe("formatDateTime", () => {
  it("should format date with time", () => {
    const date = new Date("2025-01-15T14:30:00Z");
    const result = formatDateTime(date);
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });

  it("should pass through locale parameter", () => {
    const date = new Date("2025-01-15T14:30:00Z");
    const result = formatDateTime(date, "en-US");
    expect(result).toBeDefined();
    expect(typeof result).toBe("string");
  });
});

describe("formatRelativeTime", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should return 'just now' for very recent times", () => {
    const recent = new Date("2025-01-15T11:59:30Z"); // 30 seconds ago
    const result = formatRelativeTime(recent);
    expect(result).toBe("just now");
  });

  it("should format minutes correctly", () => {
    const fiveMinutesAgo = new Date("2025-01-15T11:55:00Z");
    const result = formatRelativeTime(fiveMinutesAgo);
    expect(result).toBe("5 minutes ago");
  });

  it("should format single minute correctly", () => {
    const oneMinuteAgo = new Date("2025-01-15T11:59:00Z");
    const result = formatRelativeTime(oneMinuteAgo);
    expect(result).toBe("1 minute ago");
  });

  it("should format hours correctly", () => {
    const twoHoursAgo = new Date("2025-01-15T10:00:00Z");
    const result = formatRelativeTime(twoHoursAgo);
    expect(result).toBe("2 hours ago");
  });

  it("should format single hour correctly", () => {
    const oneHourAgo = new Date("2025-01-15T11:00:00Z");
    const result = formatRelativeTime(oneHourAgo);
    expect(result).toBe("1 hour ago");
  });

  it("should format days correctly", () => {
    const threeDaysAgo = new Date("2025-01-12T12:00:00Z");
    const result = formatRelativeTime(threeDaysAgo);
    expect(result).toBe("3 days ago");
  });

  it("should format single day correctly", () => {
    const oneDayAgo = new Date("2025-01-14T12:00:00Z");
    const result = formatRelativeTime(oneDayAgo);
    expect(result).toBe("1 day ago");
  });

  it("should format weeks correctly", () => {
    const twoWeeksAgo = new Date("2025-01-01T12:00:00Z");
    const result = formatRelativeTime(twoWeeksAgo);
    expect(result).toBe("2 weeks ago");
  });

  it("should format single week correctly", () => {
    const oneWeekAgo = new Date("2025-01-08T12:00:00Z");
    const result = formatRelativeTime(oneWeekAgo);
    expect(result).toBe("1 week ago");
  });

  it("should format months correctly", () => {
    const twoMonthsAgo = new Date("2024-11-15T12:00:00Z");
    const result = formatRelativeTime(twoMonthsAgo);
    expect(result).toBe("2 months ago");
  });

  it("should format single month correctly", () => {
    const oneMonthAgo = new Date("2024-12-15T12:00:00Z");
    const result = formatRelativeTime(oneMonthAgo);
    expect(result).toBe("1 month ago");
  });

  it("should format years correctly", () => {
    const twoYearsAgo = new Date("2023-01-15T12:00:00Z");
    const result = formatRelativeTime(twoYearsAgo);
    expect(result).toBe("2 years ago");
  });

  it("should format single year correctly", () => {
    const oneYearAgo = new Date("2024-01-15T12:00:00Z");
    const result = formatRelativeTime(oneYearAgo);
    expect(result).toBe("1 year ago");
  });

  it("should handle string input", () => {
    const oneHourAgo = "2025-01-15T11:00:00Z";
    const result = formatRelativeTime(oneHourAgo);
    expect(result).toBe("1 hour ago");
  });
});
