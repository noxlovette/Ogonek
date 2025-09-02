import test, { expect } from "@playwright/test";
test("opens single task page", async ({ page }) => {
  await page.goto("/t/tasks/task1");

  await expect(page).toHaveTitle(/Task/);
});
