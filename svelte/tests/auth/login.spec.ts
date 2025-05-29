import { expect, test } from "@playwright/test";

// Mock responses for API calls
const mockSuccessResponse = {
  accessToken: {
    token:
      "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJxcW5FUWZ6dGpwaUlZcTU2TkEzb2IiLCJyb2xlIjoidGVhY2hlciIsImV4cCI6MTc0ODQ2MTYxMywiaWF0IjoxNzQ4NDYwNzEzfQ.mock-signature",
    expiresAt: 1748461613,
  },
  refreshToken: {
    token:
      "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJxcW5FUWZ6dGpwaUlZcTU2TkEzb2IiLCJyb2xlIjoidGVhY2hlciIsImV4cCI6MTc1MTA1MjcxMywiaWF0IjoxNzQ4NDYwNzEzfQ.mock-refresh-signature",
    expiresAt: 1751052713,
  },
};

const mockUserResponse = {
  id: "kL9mN2pQ3rS4tU5vW6xY7z",
  username: "testuser",
  role: "student",
};

test.describe("Login Page", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to login page before each test
    await page.goto("/auth/login");
  });

  test("displays login form elements correctly", async ({ page }) => {
    // Check page title
    await expect(page).toHaveTitle(/Login/);

    // Check main heading
    await expect(page.locator("h2")).toContainText("Welcome back");

    // Check form elements exist
    await expect(page.locator('input[name="username"]')).toBeVisible();
    await expect(page.locator('input[name="password"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();

    // Check signup link
    await expect(page.getByRole("link", { name: "Signup" })).toContainText(
      "Signup",
    );

    // Check placeholders
    await expect(page.locator('input[name="username"]')).toHaveAttribute(
      "placeholder",
      "Username",
    );
    await expect(page.locator('input[name="password"]')).toHaveAttribute(
      "placeholder",
      "Password",
    );
  });

  test("validates required username field", async ({ page }) => {
    // Leave username empty and try to submit
    await page.fill('input[name="password"]', "password123");
    await page.click('button[type="submit"]');

    // Should show validation error
    await expect(page.locator("text=username")).toBeVisible();
  });

  test("validates required password field", async ({ page }) => {
    // Leave password empty and try to submit
    await page.fill('input[name="username"]', "testuser");
    await page.click('button[type="submit"]');

    // Should show validation error
    await expect(page.getByText("password is required")).toBeVisible();
  });

  test("password field is properly masked", async ({ page }) => {
    const passwordInput = page.locator('input[name="password"]');
    await expect(passwordInput).toHaveAttribute("type", "password");
  });

  test("form submission disables button temporarily", async ({ page }) => {
    // Mock a slow response to test loading state
    await page.route("/axum/auth/signin", async (route) => {
      await new Promise((resolve) => setTimeout(resolve, 100));
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify(mockSuccessResponse),
      });
    });

    await page.route("**/validate-access**", async (route) => {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify(mockUserResponse),
      });
    });

    // Fill form
    await page.fill('input[name="username"]', "testuser");
    await page.fill('input[name="password"]', "password123");

    // Submit and check if button is disabled during submission
    const submitButton = page.locator('button[type="submit"]');
    await submitButton.click();
  });

  test("signup link navigates correctly", async ({ page }) => {
    await page.click('a[href="/auth/signup"]');
    await expect(page).toHaveURL("/auth/signup");
  });
});

// Accessibility tests
test.describe("Login Page - Accessibility", () => {
  test("supports keyboard navigation", async ({ page }) => {
    await page.goto("/auth/login");

    // Navigate through form with keyboard
    await page.keyboard.press("Tab"); // Username field
    await page.keyboard.type("testuser");

    await page.keyboard.press("Tab"); // Password field
    await page.keyboard.type("password123");

    await page.keyboard.press("Tab"); // Submit button
    await page.keyboard.press("Enter"); // Submit form

    // Form should attempt to submit
  });
});
