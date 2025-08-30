import test, { expect } from "@playwright/test";

test("opens task page", async ({ page }) => {
  await page.goto("/s/tasks/task1");

  await expect(page).toHaveTitle(/task/);
});
