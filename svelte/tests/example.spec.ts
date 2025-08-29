import { expect, test } from "@playwright/test";

test("has title", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/Ogonek/);
});

test("login link", async ({ page }) => {
  await page.goto("/");

  await page.getByRole("link", { name: "Log in" }).click();

  await expect(page.getByRole("heading", { name: "Log in" })).toBeVisible();
});
