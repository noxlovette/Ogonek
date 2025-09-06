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
  await page.getByRole("button", { name: "Save" }).click();

  expect(page).toHaveTitle(/Lesson From/);
});

test("lesson delete", async ({ page }) => {
  await page.getByRole("link", { name: "Edit" }).click();
  await page.getByRole("button", { name: "Delete" }).click();
  await page.getByRole("button", { name: "Confirm" }).click();
});
