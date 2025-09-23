import { expect, test } from "@playwright/test";

test("sign up", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Sign up" }).click();
  await page.getByRole("textbox", { name: "Name", exact: true }).click();
  await page.getByRole("textbox", { name: "Name", exact: true }).fill("Danila");
  await page.getByRole("textbox", { name: "Name", exact: true }).press("Tab");
  await page.getByRole("textbox", { name: "Username" }).fill("noxlovette");
  await page.getByRole("textbox", { name: "Username" }).press("Tab");
  await page.getByRole("combobox").selectOption("teacher");
  await page.getByRole("combobox").press("Tab");
  await page.getByRole("textbox", { name: "Email" }).fill("nox@icllud.com");
  await page.getByRole("textbox", { name: "Email" }).press("Tab");
  await page.getByRole("textbox", { name: "Password" }).fill("h$lloW0rld");
  await page.getByRole("button").filter({ hasText: /^$/ }).first().click();
  await page.getByRole("textbox", { name: "Password" }).click();
  await page.getByRole("textbox", { name: "Password" }).click();
  await page
    .getByRole("textbox", { name: "Password" })
    .press("ControlOrMeta+a");
  await page
    .getByRole("textbox", { name: "Password" })
    .press("ControlOrMeta+c");
  await page.getByRole("textbox", { name: "Again" }).click();
  await page.getByRole("textbox", { name: "Again" }).fill("h$lloW0rld");
  await page.getByRole("button", { name: "Sign up" }).click();
  await expect(page.getByRole("button", { name: "Log In" })).toBeVisible();
});
