import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("loads without console errors and meets the serious accessibility baseline", async ({ page }) => {
  const errors = [];
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  await page.goto("/");
  await expect(page).toHaveTitle(/Edit Portability Map/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("img")).toHaveAttribute("alt", /archive layers/i);
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter((item) => ["serious", "critical"].includes(item.impact));
  expect(serious).toEqual([]);
  expect(errors).toEqual([]);
});

test("keyboard path reaches the primary command", async ({ page }) => {
  await page.goto("/");
  await page.keyboard.press("Tab");
  await expect(page.getByRole("link", { name: "Skip to main content" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.locator("main")).toBeFocused();
  await page.getByRole("button", { name: "Copy install command" }).focus();
  await page.keyboard.press("Enter");
  await expect(page.getByRole("status").filter({ hasText: "Command copied" })).toBeVisible();
});

test("profile switching and offline state remain understandable", async ({ page, context }) => {
  await page.goto("/");
  await page.getByLabel("Target profile").selectOption("darktable");
  await expect(page.locator("#profile-note")).toContainText("darktable");
  await context.setOffline(true);
  await expect(page.locator("#connection")).toContainText("Offline");
});

test("checkout return stores and strips a license, then exposes CLI activation", async ({ page }) => {
  await page.route("https://api.sociobot.in/**", (route) => route.fulfill({
    status: 200,
    contentType: "application/json",
    body: JSON.stringify({ valid: true, reason: "ok", expires_at: null })
  }));
  await page.goto("/?license=valid_test_token_12345#license");
  await expect(page).toHaveURL(/\/#license$/);
  await expect(page.locator("#license-status")).toContainText("Pro active");
  await expect(page.getByRole("button", { name: "Copy CLI activation command" })).toBeVisible();
  const stored = await page.evaluate(() => localStorage.getItem("sb_license:photo-edit-portability-map"));
  expect(stored).toBe("valid_test_token_12345");
});

test("privacy and terms pages are direct and semantic", async ({ page }) => {
  for (const path of ["/privacy/", "/terms/"]) {
    await page.goto(path);
    await expect(page.locator("h1")).toHaveCount(1);
    await expect(page.locator("main")).toHaveCount(1);
  }
});
