import { describe, expect, it } from "vitest";
import { createDaySpan, createMonthSpan, formatEventTime } from "./event";

describe("formatEventTime", () => {
  it("should return 'Весь день' for all-day events", () => {
    const result = formatEventTime(
      "2025-01-15T09:00:00Z",
      "2025-01-15T17:00:00Z",
      true,
    );
    expect(result).toBe("Весь день");
  });

  it("should format start time only when no end time", () => {
    const result = formatEventTime("2025-01-15T14:30:00Z");
    expect(result).toMatch(/\d{2}:\d{2}/);
  });

  it("should format time range for same day events", () => {
    const start = "2025-01-15T09:00:00Z";
    const end = "2025-01-15T10:30:00Z";
    const result = formatEventTime(start, end);

    expect(result).toMatch(/\d{2}:\d{2} - \d{2}:\d{2}/);
  });

  it("should format multi-day events with date", () => {
    const start = "2025-01-15T09:00:00Z";
    const end = "2025-01-16T10:30:00Z";
    const result = formatEventTime(start, end);

    expect(result).toMatch(/\d{2}:\d{2} - /);
    expect(result).toContain("16.01.2025"); // Russian date format
  });

  it("should handle times at midnight", () => {
    const start = "2025-01-15T00:00:00Z";
    const end = "2025-01-15T01:00:00Z";
    const result = formatEventTime(start, end);

    expect(result).toMatch(/03:00 - 04:00/);
  });

  it("should handle times late at night", () => {
    const start = "2025-01-15T23:30:00Z";
    const end = "2025-01-15T23:59:00Z";
    const result = formatEventTime(start, end);

    expect(result).toMatch(/02:30 - 02:59/);
  });

  it("should use 24-hour format", () => {
    const start = "2025-01-15T14:30:00Z";
    const end = "2025-01-15T15:45:00Z";
    const result = formatEventTime(start, end);

    expect(result).toMatch(/17:30 - 18:45/);
  });

  it("should handle invalid dates gracefully", () => {
    const start = "invalid-date";
    const end = "2025-01-15T10:30:00Z";

    expect(() => formatEventTime(start, end)).not.toThrow();
  });
});

describe("createMonthSpan", () => {
  it("should create span for January 2025", () => {
    const span = createMonthSpan(2025, 1);

    expect(span.start).toBe("2024-12-31T21:00:00.000Z");
    expect(span.end).toBe("2025-01-31T20:59:59.999Z");
  });

  it("should create span for February 2025 (non-leap year)", () => {
    const span = createMonthSpan(2025, 2);

    expect(span.start).toBe("2025-01-31T21:00:00.000Z");
    expect(span.end).toBe("2025-02-28T20:59:59.999Z");
  });
  it("should create span for February 2024 (leap year)", () => {
    const span = createMonthSpan(2024, 2);

    expect(span.start).toBe("2024-01-31T21:00:00.000Z"); // Moscow time UTC+3
    expect(span.end).toBe("2024-02-29T20:59:59.999Z");
  });

  it("should create span for December", () => {
    const span = createMonthSpan(2025, 12);

    expect(span.start).toBe("2025-11-30T21:00:00.000Z");
    expect(span.end).toBe("2025-12-31T20:59:59.999Z");
  });

  it("should handle 30-day months", () => {
    const span = createMonthSpan(2025, 4); // April

    expect(span.start).toBe("2025-03-31T21:00:00.000Z");
    expect(span.end).toBe("2025-04-30T20:59:59.999Z");
  });

  it("should start at beginning of month", () => {
    const span = createMonthSpan(2025, 6);
    const startDate = new Date(span.start);

    expect(startDate.getDate()).toBe(1);
    expect(startDate.getHours()).toBe(0);
    expect(startDate.getMinutes()).toBe(0);
    expect(startDate.getSeconds()).toBe(0);
    expect(startDate.getMilliseconds()).toBe(0);
  });

  it("should end at end of month", () => {
    const span = createMonthSpan(2025, 6);
    const endDate = new Date(span.end);

    expect(endDate.getDate()).toBe(30); // June has 30 days
    expect(endDate.getHours()).toBe(23);
    expect(endDate.getMinutes()).toBe(59);
    expect(endDate.getSeconds()).toBe(59);
    expect(endDate.getMilliseconds()).toBe(999);
  });
});

describe("createDaySpan", () => {
  it("should create span for specific day", () => {
    const span = createDaySpan(2025, 1, 15);

    // Moscow time UTC+3: 2025-01-15T00:00:00.000+03:00 == 2025-01-14T21:00:00.000Z
    expect(span.start).toBe("2025-01-14T21:00:00.000Z");
    expect(span.end).toBe("2025-01-15T20:59:59.999Z");
  });

  it("should create span for first day of month", () => {
    const span = createDaySpan(2025, 3, 1);

    expect(span.start).toBe("2025-02-28T21:00:00.000Z");
    expect(span.end).toBe("2025-03-01T20:59:59.999Z");
  });

  it("should create span for last day of month", () => {
    const span = createDaySpan(2025, 1, 31);

    expect(span.start).toBe("2025-01-30T21:00:00.000Z");
    expect(span.end).toBe("2025-01-31T20:59:59.999Z");
  });

  it("should create span for leap day", () => {
    const span = createDaySpan(2024, 2, 29);

    expect(span.start).toBe("2024-02-28T21:00:00.000Z");
    expect(span.end).toBe("2024-02-29T20:59:59.999Z");
  });

  it("should start at beginning of day", () => {
    const span = createDaySpan(2025, 5, 20);
    const startDate = new Date(span.start);

    expect(startDate.getUTCHours()).toBe(21); // 00:00 MSK == 21:00 UTC previous day
    expect(startDate.getUTCMinutes()).toBe(0);
    expect(startDate.getUTCSeconds()).toBe(0);
    expect(startDate.getUTCMilliseconds()).toBe(0);
  });

  it("should end at end of day", () => {
    const span = createDaySpan(2025, 5, 20);
    const endDate = new Date(span.end);

    expect(endDate.getUTCHours()).toBe(20); // 23:59 MSK == 20:59 UTC
    expect(endDate.getUTCMinutes()).toBe(59);
    expect(endDate.getUTCSeconds()).toBe(59);
    expect(endDate.getUTCMilliseconds()).toBe(999);
  });

  it("should handle different years", () => {
    const span2024 = createDaySpan(2024, 1, 1);
    const span2025 = createDaySpan(2025, 1, 1);

    expect(span2024.start).toBe("2023-12-31T21:00:00.000Z");
    expect(span2025.start).toBe("2024-12-31T21:00:00.000Z");
  });

  it("should maintain same day throughout span", () => {
    const span = createDaySpan(2025, 7, 15);
    const startDate = new Date(span.start);
    const endDate = new Date(span.end);

    // Both should be for July 15th MSK, but UTC date may differ
    expect(startDate.getUTCDate()).toBe(14); // 00:00 MSK == 21:00 UTC previous day
    expect(endDate.getUTCDate()).toBe(15); // 20:59 UTC is still July 15th UTC
    expect(startDate.getUTCMonth()).toBe(endDate.getUTCMonth());
    expect(startDate.getUTCFullYear()).toBe(endDate.getUTCFullYear());
  });
});
