import test, { expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/t/dashboard");
  await page.getByRole("link", { name: "Tasks" }).click();
});

test("task list title", async ({ page }) => {
  await expect(page).toHaveTitle(/Tasks/);
});

test.beforeEach(async ({ page }) => {
  await page
    .getByRole("button", { name: /View details/ })
    .first()
    .click();
});
test("navigates to detail view", async ({ page }) => {
  await expect(page).toHaveTitle(/Task From/);
});

test("edit task", async ({ page }) => {
  await page.locator("div:nth-child(3) > .flex").first().click();
  await page
    .locator("form")
    .filter({ hasText: "Editing... title assignee" })
    .locator("#assignee")
    .selectOption({ index: 0 });
  await page.getByPlaceholder("Due Date").fill("2025-09-17");
  await page
    .getByRole("button", { name: "Set priority to High Priority" })
    .click();
  await page.getByRole("textbox", { name: "Title" }).click();
  await page.getByRole("textbox", { name: "Title" }).press("ControlOrMeta+a");
  await page.getByRole("textbox", { name: "Title" }).fill("Hello!");
  await page
    .locator("form")
    .filter({ hasText: "Editing... title assignee" })
    .locator("#btn")
    .nth(2)
    .click();
});
