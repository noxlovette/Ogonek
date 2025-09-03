import { expect, test } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/t/dashboard");
  await page.getByRole("link", { name: "Lessons" }).click();
});

test("lesson list title", async ({ page }) => {
  await expect(page).toHaveTitle(/Lessons/);
});
test.beforeEach(async ({ page }) => {
  await page
    .getByRole("button", { name: /View details/ })
    .first()
    .click();
});
test("lesson edit", async ({ page }) => {
  await page.getByRole("link", { name: "Edit" }).click();
  await page.getByRole("textbox", { name: "Topic" }).click();
  await page.getByRole("textbox", { name: "Topic" }).fill("Hello");
  await page
    .locator("form")
    .filter({ hasText: "Editing... title topic" })
    .locator("#btn")
    .nth(2)
    .click();
});

test("lesson delete", async ({ page }) => {
  await page.locator("div:nth-child(3) > .flex").click();
  await page
    .locator("form")
    .filter({ hasText: "Editing... title topic" })
    .locator("#btn")
    .nth(1)
    .click();
  await page
    .locator("form")
    .filter({ hasText: "Editing... You are deleting" })
    .locator("#btn")
    .nth(3)
    .click();
  await page.getByRole("alert", { name: "Notification" }).click();
});
