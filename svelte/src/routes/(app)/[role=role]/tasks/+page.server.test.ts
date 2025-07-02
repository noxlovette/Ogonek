import { describe, expect, it, vi } from "vitest";
import { load } from "./+page.server";

const mockTasks = {
  data: [{ id: 1, name: "Task A" }],
  total: 1,
  page: 1,
  per_page: 50,
};

describe("load", () => {
  const fetch = vi.fn();
  const depends = vi.fn();

  const createUrl = (query: Record<string, string>) => {
    const url = new URL("http://localhost");
    for (const [key, value] of Object.entries(query)) {
      url.searchParams.set(key, value);
    }
    return url;
  };

  it("fetches tasks with default parameters", async () => {
    fetch.mockResolvedValueOnce({
      json: () => Promise.resolve(mockTasks),
    });

    const result = await load({
      fetch,
      url: createUrl({}),
      depends,
    } as any);

    expect(fetch).toHaveBeenCalledWith("/axum/task?page=1&per_page=50");
    expect(result).toEqual({ tasksPaginated: mockTasks });
    expect(depends).toHaveBeenCalledWith("tasks:completed");
  });

  it("fetches tasks with custom query parameters", async () => {
    fetch.mockResolvedValueOnce({
      json: () => Promise.resolve(mockTasks),
    });

    const result = await load({
      fetch,
      url: createUrl({
        page: "2",
        per_page: "20",
        search: "test",
        assignee: "john",
        completed: "true",
      }),
      depends,
    } as any);

    expect(fetch).toHaveBeenCalledWith(
      "/axum/task?page=2&per_page=20&search=test&assignee=john&completed=true",
    );
    expect(result).toEqual({ tasksPaginated: mockTasks });
  });

  it("handles fetch error gracefully", async () => {
    const errorLogger = vi.spyOn(console, "error").mockImplementation(() => {});
    fetch.mockRejectedValueOnce(new Error("Fetch failed"));

    const result = await load({
      fetch,
      url: createUrl({}),
      depends,
    } as any);

    expect(result.status).toBe(500);
    errorLogger.mockRestore();
  });
});
