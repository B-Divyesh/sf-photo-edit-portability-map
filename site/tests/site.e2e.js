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

test("normal use stays on the product origin", async ({ page, baseURL }) => {
  const thirdParty = [];
  const productOrigin = new URL(baseURL).origin;
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== productOrigin) thirdParty.push(request.url());
  });
  await page.goto("/");
  await page.waitForLoadState("networkidle");
  expect(thirdParty).toEqual([]);
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

test("mobile hero content fits and every visible link has a 44px touch target", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "390px regression");
  await page.goto("/");
  const heroMetrics = await page.locator(".hero").evaluate((hero) => {
    const box = hero.getBoundingClientRect();
    const descendants = [...hero.querySelectorAll(".hero-copy, .command-bar, .hero-actions a")];
    return {
      viewportWidth: document.documentElement.clientWidth,
      documentWidth: document.documentElement.scrollWidth,
      heroLeft: box.left,
      heroRight: box.right,
      descendants: descendants.map((element) => {
        const rect = element.getBoundingClientRect();
        return { className: element.className, left: rect.left, right: rect.right };
      })
    };
  });
  expect(heroMetrics.viewportWidth).toBe(390);
  expect(heroMetrics.documentWidth).toBe(390);
  for (const item of heroMetrics.descendants) {
    expect(item.left, item.className).toBeGreaterThanOrEqual(heroMetrics.heroLeft);
    expect(item.right, item.className).toBeLessThanOrEqual(heroMetrics.heroRight);
  }

  const undersizedLinks = await page.locator("a:visible").evaluateAll((links) => links
    .map((link) => {
      const rect = link.getBoundingClientRect();
      return { text: link.textContent.trim(), width: rect.width, height: rect.height };
    })
    .filter(({ width, height }) => width < 44 || height < 44));
  expect(undersizedLinks).toEqual([]);
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

test("restore and revocation replace a stale valid verdict without blocking free tools", async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem("sb_license:photo-edit-portability-map", "previous_valid_token");
    localStorage.setItem("sb_license:photo-edit-portability-map:verdict", JSON.stringify({
      token: "previous_valid_token",
      valid: true,
      checkedAt: Date.now()
    }));
  });
  await page.route("https://api.sociobot.in/**", (route) => route.fulfill({
    status: 200,
    contentType: "application/json",
    body: JSON.stringify({ valid: false, reason: "revoked", expires_at: null })
  }));
  await page.goto("/?license=revoked_test_token_12345#license");
  await expect(page).toHaveURL(/\/#license$/);
  await expect(page.locator("#license-status")).toContainText("no longer active");
  await expect(page.getByRole("link", { name: "Run your pre-flight" })).toBeVisible();
  const stored = await page.evaluate(() => ({
    token: localStorage.getItem("sb_license:photo-edit-portability-map"),
    verdict: JSON.parse(localStorage.getItem("sb_license:photo-edit-portability-map:verdict"))
  }));
  expect(stored.token).toBe("revoked_test_token_12345");
  expect(stored.verdict).toMatchObject({ token: "revoked_test_token_12345", valid: false });
});

test("privacy and terms pages are direct and semantic", async ({ page }) => {
  for (const path of ["/privacy/", "/terms/"]) {
    await page.goto(path);
    await expect(page.locator("h1")).toHaveCount(1);
    await expect(page.locator("main")).toHaveCount(1);
  }
});

test("the deployed service worker updates its cache and serves the shell offline", async ({ page, context, baseURL }) => {
  test.skip(!baseURL.startsWith("https://"), "live HTTPS service-worker check");
  await page.goto("/");
  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
  });
  await page.reload();
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  const caches = await page.evaluate(() => window.caches.keys());
  expect(caches).toContain("edit-portability-map-v2");
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await expect(page.locator("h1")).toContainText("See what your edits are");
  await expect(page.locator("#connection")).toContainText("Offline");
  await context.setOffline(false);
});
