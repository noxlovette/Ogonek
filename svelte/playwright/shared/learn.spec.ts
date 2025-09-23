import { expect, test } from "@playwright/test";

test("learn mode buttons", async ({ page }) => {
  await page.goto("/t/dashboard");
  await page.getByRole("link", { name: /Due for Review/ }).click();
  await page.getByRole("button", { name: "Flip Enter" }).click();
  await page.getByRole("button", { name: "Disaster" }).click();
  await page.getByRole("button", { name: "Flip Enter" }).click();
  await page.getByRole("button", { name: "Ok" }).click();
  await page.getByRole("button", { name: "Flip Enter" }).click();
  await page.getByRole("button", { name: "Breeze" }).click();
  await page.getByRole("button", { name: "Back" }).click();

  expect(page).toHaveTitle(/Flashcards/);
});
