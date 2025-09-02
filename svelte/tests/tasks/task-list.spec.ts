import test, { expect } from "@playwright/test";

test.describe("task list", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/t/tasks");
  });

  test("has the right title", async ({ page }) => {
    await expect(page).toHaveTitle(/Tasks/);
  });

  test("displays empty state when no tasks", async ({ page }) => {
    await expect(page.getByText("No tasks found")).toBeVisible();
  });

  test("task click navigates to detail page", async ({ page }) => {
    await page.getByRole("link").filter({ hasText: "Task 1" }).click();
    await expect(page).toHaveURL(`/t/tasks/`);
  });
});
