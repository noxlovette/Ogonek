import type { EventSmall } from "$lib/types/api/calendar";
import { describe, expect, it } from "vitest";
import {
  createDateKey,
  generateMonthDays,
  groupEventsByDate,
  isSameDay,
} from "./calendar";

describe("createDateKey", () => {
  it("should create correct date key", () => {
    const date = new Date(2025, 0, 15); // January 15, 2025
    expect(createDateKey(date)).toBe("2025-0-15");
  });

  it("should handle different months", () => {
    const date1 = new Date(2025, 11, 31); // December 31, 2025
    const date2 = new Date(2025, 0, 1); // January 1, 2025

    expect(createDateKey(date1)).toBe("2025-11-31");
    expect(createDateKey(date2)).toBe("2025-0-1");
  });
});

describe("groupEventsByDate", () => {
  const mockEvents: EventSmall[] = [
    {
      id: "1",
      title: "Event 1",
      dtstartTime: "2025-01-15T10:00:00Z",
      dtendTime: "2025-01-15T11:00:00Z",
      isAllDay: false,
    },
    {
      id: "2",
      title: "Event 2",
      dtstartTime: "2025-01-15T14:00:00Z",
      dtendTime: "2025-01-15T15:00:00Z",
      isAllDay: false,
    },
    {
      id: "3",
      title: "Event 3",
      dtstartTime: "2025-01-16T09:00:00Z",
      dtendTime: "2025-01-16T10:00:00Z",
      isAllDay: false,
    },
  ];

  it("should group events by date correctly", () => {
    const result = groupEventsByDate(mockEvents);

    const jan15Key = createDateKey(new Date("2025-01-15T10:00:00Z"));
    const jan16Key = createDateKey(new Date("2025-01-16T09:00:00Z"));

    expect(result[jan15Key]).toHaveLength(2);
    expect(result[jan16Key]).toHaveLength(1);
    expect(result[jan15Key]).toContain(mockEvents[0]);
    expect(result[jan15Key]).toContain(mockEvents[1]);
    expect(result[jan16Key]).toContain(mockEvents[2]);
  });

  it("should handle empty events array", () => {
    const result = groupEventsByDate([]);
    expect(result).toEqual({});
  });

  it("should handle single event", () => {
    const singleEvent = mockEvents[0];
    const result = groupEventsByDate([singleEvent]);

    const dateKey = createDateKey(new Date(singleEvent.dtstartTime));
    expect(result[dateKey]).toHaveLength(1);
    expect(result[dateKey][0]).toBe(singleEvent);
  });
});

describe("isSameDay", () => {
  it("should return true for same day", () => {
    const date1 = new Date(2025, 0, 15, 10, 30);
    const date2 = new Date(2025, 0, 15, 20, 45);

    expect(isSameDay(date1, date2)).toBe(true);
  });

  it("should return false for different days", () => {
    const date1 = new Date(2025, 0, 15);
    const date2 = new Date(2025, 0, 16);

    expect(isSameDay(date1, date2)).toBe(false);
  });

  it("should return false for different months", () => {
    const date1 = new Date(2025, 0, 15);
    const date2 = new Date(2025, 1, 15);

    expect(isSameDay(date1, date2)).toBe(false);
  });

  it("should return false for different years", () => {
    const date1 = new Date(2025, 0, 15);
    const date2 = new Date(2024, 0, 15);

    expect(isSameDay(date1, date2)).toBe(false);
  });
});

describe("generateMonthDays", () => {
  const mockEvents: EventSmall[] = [
    {
      id: "1",
      title: "Event 1",
      dtstartTime: "2025-01-15T10:00:00Z",
      dtendTime: "2025-01-15T11:00:00Z",
      isAllDay: false,
    },
  ];

  it("should generate 42 days (6 weeks)", () => {
    const referenceDate = new Date(2025, 0, 15); // January 15, 2025
    const result = generateMonthDays(referenceDate, mockEvents);

    expect(result.monthDays).toHaveLength(42);
  });

  it("should include previous month overflow days", () => {
    const referenceDate = new Date(2025, 0, 1); // January 1, 2025 (Wednesday)
    const result = generateMonthDays(referenceDate, mockEvents);

    const firstDay = result.monthDays[0];
    expect(firstDay.isCurrentMonth).toBe(false);
    expect(firstDay.day).toBeGreaterThan(25); // Should be from previous month
  });

  it("should mark current month days correctly", () => {
    const referenceDate = new Date(2025, 0, 15); // January 15, 2025
    const result = generateMonthDays(referenceDate, mockEvents);

    const currentMonthDays = result.monthDays.filter(
      (day) => day.isCurrentMonth,
    );
    expect(currentMonthDays).toHaveLength(31); // January has 31 days

    const day15 = currentMonthDays.find((day) => day.day === 15);
    expect(day15?.isCurrentMonth).toBe(true);
  });

  it("should mark today correctly", () => {
    const today = new Date();
    const referenceDate = new Date(today.getFullYear(), today.getMonth(), 1);
    const result = generateMonthDays(referenceDate, []);

    const todayDay = result.monthDays.find((day) => day.isToday);
    expect(todayDay).toBeDefined();
    expect(todayDay?.day).toBe(today.getDate());
  });

  it("should include next month overflow days", () => {
    const referenceDate = new Date(2025, 0, 1); // January 2025
    const result = generateMonthDays(referenceDate, mockEvents);

    const lastDay = result.monthDays[41];
    expect(lastDay.isCurrentMonth).toBe(false);
    expect(lastDay.day).toBeLessThan(15); // Should be from next month
  });

  it("should return correct month name and year", () => {
    const referenceDate = new Date(2025, 0, 15); // January 15, 2025
    const result = generateMonthDays(referenceDate, mockEvents, "en-US");

    expect(result.year).toBe(2025);
    expect(result.monthName.toLowerCase()).toContain("january");
  });

  it("should assign events to correct days", () => {
    const referenceDate = new Date(2025, 0, 1); // January 2025
    const events: EventSmall[] = [
      {
        id: "1",
        title: "Event on 15th",
        dtstartTime: "2025-01-15T10:00:00Z",
        dtendTime: "2025-01-15T11:00:00Z",
        isAllDay: false,
      },
    ];

    const result = generateMonthDays(referenceDate, events);
    const day15 = result.monthDays.find(
      (day) => day.day === 15 && day.isCurrentMonth,
    );

    expect(day15?.events).toHaveLength(1);
    expect(day15?.events[0].title).toBe("Event on 15th");
  });

  it("should handle different locales", () => {
    const referenceDate = new Date(2025, 0, 15); // January 15, 2025
    const resultRu = generateMonthDays(referenceDate, [], "ru-RU");
    const resultEn = generateMonthDays(referenceDate, [], "en-US");

    expect(resultRu.monthName).not.toBe(resultEn.monthName);
  });

  it("should set correct actualDate for each day", () => {
    const referenceDate = new Date(2025, 0, 15); // January 15, 2025
    const result = generateMonthDays(referenceDate, []);

    const day15 = result.monthDays.find(
      (day) => day.day === 15 && day.isCurrentMonth,
    );
    expect(day15?.actualDate.getDate()).toBe(15);
    expect(day15?.actualDate.getMonth()).toBe(0); // January
    expect(day15?.actualDate.getFullYear()).toBe(2025);
  });
});
