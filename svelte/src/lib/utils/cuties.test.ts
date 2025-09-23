import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { getGreeting } from "./cuties";

describe("getGreeting", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should return 'morning' for hours 5-11", () => {
    vi.setSystemTime(new Date("2025-01-01 08:00:00"));
    expect(getGreeting()).toBe("morning");

    vi.setSystemTime(new Date("2025-01-01 05:00:00"));
    expect(getGreeting()).toBe("morning");

    vi.setSystemTime(new Date("2025-01-01 11:59:59"));
    expect(getGreeting()).toBe("morning");
  });

  it("should return 'afternoon' for hours 12-17", () => {
    vi.setSystemTime(new Date("2025-01-01 12:00:00"));
    expect(getGreeting()).toBe("afternoon");

    vi.setSystemTime(new Date("2025-01-01 15:30:00"));
    expect(getGreeting()).toBe("afternoon");

    vi.setSystemTime(new Date("2025-01-01 17:59:59"));
    expect(getGreeting()).toBe("afternoon");
  });

  it("should return 'evening' for hours 18-21", () => {
    vi.setSystemTime(new Date("2025-01-01 18:00:00"));
    expect(getGreeting()).toBe("evening");

    vi.setSystemTime(new Date("2025-01-01 20:30:00"));
    expect(getGreeting()).toBe("evening");

    vi.setSystemTime(new Date("2025-01-01 21:59:59"));
    expect(getGreeting()).toBe("evening");
  });

  it("should return 'night' for hours 22-4", () => {
    vi.setSystemTime(new Date("2025-01-01 22:00:00"));
    expect(getGreeting()).toBe("night");

    vi.setSystemTime(new Date("2025-01-01 23:30:00"));
    expect(getGreeting()).toBe("night");

    vi.setSystemTime(new Date("2025-01-01 02:00:00"));
    expect(getGreeting()).toBe("night");

    vi.setSystemTime(new Date("2025-01-01 04:59:59"));
    expect(getGreeting()).toBe("night");
  });
});
