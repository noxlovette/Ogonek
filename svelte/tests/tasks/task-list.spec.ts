import test, { expect } from "@playwright/test";

test.describe("task list", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/t/tasks");
  });

  test("opens tasks page", async ({ page }) => {
    await expect(page).toHaveTitle(/Tasks/);
  });
});
