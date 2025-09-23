import { describe, it, expect, vi } from "vitest";
import { formatFileSize, getFileExtension, shuffleArray, getUrgency } from "./ui";
import type { TaskFull } from "$lib/types";

describe("formatFileSize", () => {
  it("should format 0 bytes correctly", () => {
    expect(formatFileSize(0)).toBe("0 Bytes");
  });

  it("should format bytes correctly", () => {
    expect(formatFileSize(100)).toBe("100 Bytes");
    expect(formatFileSize(1023)).toBe("1023 Bytes");
  });

  it("should format kilobytes correctly", () => {
    expect(formatFileSize(1024)).toBe("1 KB");
    expect(formatFileSize(1536)).toBe("1.5 KB");
    expect(formatFileSize(2048)).toBe("2 KB");
  });

  it("should format megabytes correctly", () => {
    expect(formatFileSize(1024 * 1024)).toBe("1 MB");
    expect(formatFileSize(1024 * 1024 * 2.5)).toBe("2.5 MB");
  });

  it("should format gigabytes correctly", () => {
    expect(formatFileSize(1024 * 1024 * 1024)).toBe("1 GB");
    expect(formatFileSize(1024 * 1024 * 1024 * 1.75)).toBe("1.75 GB");
  });

  it("should handle decimal places correctly", () => {
    expect(formatFileSize(1536)).toBe("1.5 KB");
    expect(formatFileSize(1075)).toBe("1.05 KB");
  });
});

describe("getFileExtension", () => {
  it("should extract extension from filename", () => {
    expect(getFileExtension("document.pdf")).toBe("PDF");
    expect(getFileExtension("image.jpg")).toBe("JPG");
    expect(getFileExtension("archive.tar.gz")).toBe("GZ");
  });

  it("should handle files without extensions", () => {
    expect(getFileExtension("README")).toBe("README");
    expect(getFileExtension("dockerfile")).toBe("DOCKERFILE");
  });

  it("should handle files with multiple dots", () => {
    expect(getFileExtension("my.file.name.txt")).toBe("TXT");
    expect(getFileExtension("backup.2023.sql")).toBe("SQL");
  });

  it("should handle edge cases", () => {
    expect(getFileExtension("")).toBe("");
    expect(getFileExtension(".")).toBe("");
    expect(getFileExtension(".hidden")).toBe("HIDDEN");
    expect(getFileExtension("file.")).toBe("");
  });

  it("should convert to uppercase", () => {
    expect(getFileExtension("file.html")).toBe("HTML");
    expect(getFileExtension("script.js")).toBe("JS");
    expect(getFileExtension("data.JSON")).toBe("JSON");
  });
});

describe("shuffleArray", () => {
  it("should return array with same length", () => {
    const input = [1, 2, 3, 4, 5];
    const result = shuffleArray(input);
    expect(result).toHaveLength(input.length);
  });

  it("should return array with same elements", () => {
    const input = [1, 2, 3, 4, 5];
    const result = shuffleArray(input);
    expect(result.sort()).toEqual(input.sort());
  });

  it("should not modify original array", () => {
    const input = [1, 2, 3, 4, 5];
    const original = [...input];
    shuffleArray(input);
    expect(input).toEqual(original);
  });

  it("should handle empty array", () => {
    const result = shuffleArray([]);
    expect(result).toEqual([]);
  });

  it("should handle single element array", () => {
    const result = shuffleArray([42]);
    expect(result).toEqual([42]);
  });

  it("should shuffle array differently on multiple calls", () => {
    const input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const result1 = shuffleArray(input);
    const result2 = shuffleArray(input);
    
    expect(result1).not.toEqual(result2);
  });

  it("should work with different data types", () => {
    const strings = ["a", "b", "c"];
    const objects = [{ id: 1 }, { id: 2 }, { id: 3 }];
    
    expect(shuffleArray(strings)).toHaveLength(3);
    expect(shuffleArray(objects)).toHaveLength(3);
  });
});

describe("getUrgency", () => {
  const createTask = (dueDate: string): TaskFull => ({
    dueDate,
    id: "1",
    title: "Test Task",
    description: "Test Description",
    status: "pending",
    createdAt: "2025-01-01T00:00:00Z",
    updatedAt: "2025-01-01T00:00:00Z",
  } as TaskFull);

  it("should return 'overdue' for past due dates", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task = createTask("2025-01-14T12:00:00Z");
    expect(getUrgency(task)).toBe("overdue");
    
    vi.useRealTimers();
  });

  it("should return 'urgent' for tasks due within 1 day", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task = createTask("2025-01-16T12:00:00Z");
    expect(getUrgency(task)).toBe("urgent");
    
    vi.useRealTimers();
  });

  it("should return 'soon' for tasks due within 3 days", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task2Days = createTask("2025-01-17T12:00:00Z");
    const task3Days = createTask("2025-01-18T12:00:00Z");
    
    expect(getUrgency(task2Days)).toBe("soon");
    expect(getUrgency(task3Days)).toBe("soon");
    
    vi.useRealTimers();
  });

  it("should return 'normal' for tasks due more than 3 days away", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task = createTask("2025-01-20T12:00:00Z");
    expect(getUrgency(task)).toBe("normal");
    
    vi.useRealTimers();
  });

  it("should handle tasks without due dates", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task = createTask("");
    expect(getUrgency(task)).toBe("normal");
    
    vi.useRealTimers();
  });

  it("should handle invalid due dates", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2025-01-15T12:00:00Z"));
    
    const task = createTask("invalid-date");
    expect(getUrgency(task)).toBe("normal");
    
    vi.useRealTimers();
  });
});