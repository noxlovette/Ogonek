import { describe, expect, it, vi } from "vitest";
import { delay } from "./simulators";

describe("delay", () => {
  it("should return a promise that resolves after the specified time", async () => {
    vi.useFakeTimers();

    const promise = delay(1000);

    let resolved = false;
    promise.then(() => {
      resolved = true;
    });

    expect(resolved).toBe(false);

    vi.advanceTimersByTime(999);
    await Promise.resolve();
    expect(resolved).toBe(false);

    vi.advanceTimersByTime(1);
    await Promise.resolve();
    expect(resolved).toBe(true);

    vi.useRealTimers();
  });
});
