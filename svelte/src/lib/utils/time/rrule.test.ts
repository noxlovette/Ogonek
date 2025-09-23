import { describe, expect, it } from "vitest";
import { parseRRuleDays, WEEKDAYS } from "./rrule";

describe("WEEKDAYS constant", () => {
  it("should have correct structure for all weekdays", () => {
    expect(WEEKDAYS).toHaveLength(7);

    WEEKDAYS.forEach((day, index) => {
      expect(day).toHaveProperty("label");
      expect(day).toHaveProperty("full");
      expect(day).toHaveProperty("ical");
      expect(day).toHaveProperty("index");
      expect(day.index).toBe(index);
    });
  });

  it("should have correct Russian labels", () => {
    expect(WEEKDAYS[0].full).toBe("Воскресенье");
    expect(WEEKDAYS[1].full).toBe("Понедельник");
    expect(WEEKDAYS[2].full).toBe("Вторник");
    expect(WEEKDAYS[3].full).toBe("Среда");
    expect(WEEKDAYS[4].full).toBe("Четверг");
    expect(WEEKDAYS[5].full).toBe("Пятница");
    expect(WEEKDAYS[6].full).toBe("Суббота");
  });

  it("should have correct iCal day codes", () => {
    expect(WEEKDAYS[0].ical).toBe("SU");
    expect(WEEKDAYS[1].ical).toBe("MO");
    expect(WEEKDAYS[2].ical).toBe("TU");
    expect(WEEKDAYS[3].ical).toBe("WE");
    expect(WEEKDAYS[4].ical).toBe("TH");
    expect(WEEKDAYS[5].ical).toBe("FR");
    expect(WEEKDAYS[6].ical).toBe("SA");
  });

  it("should have correct indices", () => {
    WEEKDAYS.forEach((day, index) => {
      expect(day.index).toBe(index);
    });
  });
});

describe("parseRRuleDays", () => {
  it("should return empty array for null rrule", () => {
    expect(parseRRuleDays(null)).toEqual([]);
  });

  it("should return empty array for undefined rrule", () => {
    expect(parseRRuleDays(undefined)).toEqual([]);
  });

  it("should return empty array for empty string rrule", () => {
    expect(parseRRuleDays("")).toEqual([]);
  });

  it("should return empty array when no BYDAY found", () => {
    const rrule = "FREQ=WEEKLY;INTERVAL=1;COUNT=10";
    expect(parseRRuleDays(rrule)).toEqual([]);
  });

  it("should parse single day correctly", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=MO;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1]); // Monday index
  });

  it("should parse multiple days correctly", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=MO,WE,FR;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 3, 5]); // Monday, Wednesday, Friday indices
  });

  it("should parse all weekdays", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=SU,MO,TU,WE,TH,FR,SA;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([0, 1, 2, 3, 4, 5, 6]);
  });

  it("should handle days in different order", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=FR,MO,WE;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 3, 5]); // Should maintain original weekday order
  });

  it("should handle BYDAY with other parameters before", () => {
    const rrule = "FREQ=WEEKLY;INTERVAL=2;BYDAY=TU,TH;COUNT=5";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([2, 4]); // Tuesday, Thursday
  });

  it("should handle BYDAY with other parameters after", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=SA,SU;INTERVAL=1;UNTIL=20251231";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([0, 6]); // Sunday, Saturday
  });

  it("should ignore invalid day codes", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=MO,XX,WE,YY;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 3]); // Only Monday, Wednesday
  });

  it("should handle duplicate day codes", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=MO,MO,WE;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 3]); // Will not include duplicates based on input
  });

  it("should work with custom day mapping", () => {
    const customDays = [
      { label: "D", full: "Domingo", ical: "SU", index: 10 },
      { label: "L", full: "Lunes", ical: "MO", index: 20 },
      { label: "M", full: "Martes", ical: "TU", index: 30 },
    ];

    const rrule = "FREQ=WEEKLY;BYDAY=MO,TU;INTERVAL=1";
    const result = parseRRuleDays(rrule, customDays);
    expect(result).toEqual([20, 30]); // Custom indices for Monday, Tuesday
  });

  it("should handle case sensitivity", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=mo,WE,fr;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([3]); // Only WE matches (case sensitive)
  });

  it("should handle whitespace in BYDAY value", () => {
    const rrule = "FREQ=WEEKLY;BYDAY=MO, WE ,FR;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 5]); // MO and FR, WE doesn't match due to space
  });

  it("should handle complex rrule with multiple semicolons", () => {
    const rrule =
      "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,WE,FR;COUNT=10;UNTIL=20251231";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([1, 3, 5]);
  });

  it("should handle BYDAY at the end of rrule", () => {
    const rrule = "FREQ=WEEKLY;INTERVAL=1;COUNT=10;BYDAY=TH";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([4]); // Thursday
  });

  it("should handle BYDAY at the beginning of rrule", () => {
    const rrule = "BYDAY=SA;FREQ=WEEKLY;INTERVAL=1";
    const result = parseRRuleDays(rrule);
    expect(result).toEqual([6]); // Saturday
  });
});
