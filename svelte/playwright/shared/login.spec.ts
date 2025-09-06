import { expect, test } from "@playwright/test";

test("sign in", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Log In" }).click();
  await page.getByRole("textbox", { name: "Username" }).click();
  await page.getByRole("textbox", { name: "Username" }).fill("dev_teacher1");
  await page.getByRole("textbox", { name: "Username" }).press("Tab");
  await page.getByRole("textbox", { name: "Password" }).fill("blablabla");
  await page.getByRole("button", { name: "Log In" }).click();

  await expect(page).toHaveTitle(/Dashboard/);
});
