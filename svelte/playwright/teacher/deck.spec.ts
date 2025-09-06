import test, { expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/t/dashboard");
  await page.getByRole("link", { name: "Flashcards" }).click();
});

test("flashcards list title", async ({ page }) => {
  await expect(page).toHaveTitle(/Flashcards/);
});

test.beforeEach(async ({ page }) => {
  await page
    .getByRole("button", { name: /View details/ })
    .first()
    .click();
});
test("deck edit", async ({ page }) => {
  await page.getByRole("link", { name: "Edit" }).click();
  await page.getByRole("button", { name: "Save" }).click();
  await page.getByRole("alert", { name: "Notification" }).click();
});
