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

test("task edit", async ({ page }) => {
  await page.getByRole("link", { name: "Edit" }).click();
  await page.locator("form").locator("#assignee").selectOption({ index: 0 });
  await page.getByPlaceholder("Due Date").fill("2025-09-17");
  await page
    .getByRole("button", { name: "Set priority to High Priority" })
    .click();
  await page.getByRole("textbox", { name: "Title" }).click();
  await page.getByRole("textbox", { name: "Title" }).press("ControlOrMeta+a");
  await page.getByRole("textbox", { name: "Title" }).fill("Hello!");
  await page.getByRole("button", { name: "Save" }).click();
});

test("task delete", async ({ page }) => {
  await page.getByRole("link", { name: "Edit" }).click();
  await page.getByRole("button", { name: "Delete" }).click();
  await page.getByRole("button", { name: "Confirm" }).click();
  await page.getByRole("alert", { name: "Notification" }).click();
});
