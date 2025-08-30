import test, { expect } from "@playwright/test";

test("opens tasks page", async ({ page }) => {
  await page.goto("/t/tasks");

  await expect(page).toHaveTitle(/Tasks/);
});

test("opens single task page", async ({ page }) => {
  await page.goto("/t/tasks/task1");

  await expect(page).toHaveTitle(/Task/);
});
