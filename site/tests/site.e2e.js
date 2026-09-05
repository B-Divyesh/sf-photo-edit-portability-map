import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("home gives a first-time photographer the job, audience, and first action", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle("Edit Portability Map — Map Lightroom metadata");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Map Lightroom metadata before you move.");
  await expect(page.getByText("For photographers leaving Lightroom", { exact: false })).toBeVisible();
  const demoLink = page.getByRole("link", { name: "Try it with sample data" });
  await expect(demoLink).toBeVisible();
  await expect(page.getByText("Runs a safe sample scan in a temporary folder.")).toBeVisible();
  await demoLink.click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page).toHaveTitle("Demo — Edit Portability Map");
});

test("@claim:demo-populated-output one click opens a labelled, populated sample report", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.getByRole("button", { name: "Reset demo" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Start for real" })).toBeVisible();
  await expect(page.locator("#demo-report-body tr")).toHaveCount(5);
  await expect(page.locator("#demo-report-body")).toContainText("Lightroom develop recipe");
  await expect(page.getByText("Catalog-only", { exact: true }).first()).toBeVisible();
});

test("@claim:demo-isolated reset and leaving demo only change the demo storage key", async ({ page }) => {
  await page.addInitScript(() => localStorage.setItem("real:photo-library", "untouched"));
  await page.goto("/demo/");
  await page.getByLabel("Target profile").selectOption("darktable");
  expect(await page.evaluate(() => localStorage.getItem("demo:photo-edit-portability-map:state"))).toContain("darktable");
  await page.getByRole("button", { name: "Reset demo" }).click();
  expect(await page.evaluate(() => ({ real: localStorage.getItem("real:photo-library"), demo: localStorage.getItem("demo:photo-edit-portability-map:state") }))).toEqual({ real: "untouched", demo: JSON.stringify({ profile: "immich" }) });
  await page.getByRole("link", { name: "Start for real" }).click();
  await expect(page).toHaveURL(/\/$/);
  expect(await page.evaluate(() => ({ real: localStorage.getItem("real:photo-library"), demo: localStorage.getItem("demo:photo-edit-portability-map:state") }))).toEqual({ real: "untouched", demo: null });
});

test("@claim:demo-no-upload the full demo flow uses only the product origin", async ({ page, baseURL }) => {
  const unexpected = [];
  const productOrigin = new URL(baseURL).origin;
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== productOrigin) unexpected.push(request.url());
  });
  await page.goto("/demo/");
  await page.getByLabel("Target profile").selectOption("digikam");
  await page.getByRole("button", { name: "Reset demo" }).click();
  await page.waitForLoadState("networkidle");
  expect(unexpected).toEqual([]);
});

test("@claim:demo-profile the sample report changes when the selected target changes", async ({ page }) => {
  await page.goto("/demo/");
  await page.getByLabel("Target profile").selectOption("darktable");
  await expect(page.locator("#profile-note")).toContainText("darktable");
  await expect(page.locator("#demo-report-body")).toContainText("Supported");
});

test("@claim:license-restore a returned license is stored, verified, and removed from the address", async ({ page }) => {
  await page.route("https://api.sociobot.in/**", (route) => route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ valid: false, reason: "invalid", expires_at: null }) }));
  await page.goto("/?license=invalid_test_token_12345#license");
  await expect(page).toHaveURL(/\/#license$/);
  await expect(page.locator("#license-status")).toContainText("not active");
  expect(await page.evaluate(() => localStorage.getItem("sb_license:photo-edit-portability-map"))).toBe("invalid_test_token_12345");
});

test("@claim:site-runtime-privacy every published page loads only product assets", async ({ page, baseURL }) => {
  const productOrigin = new URL(baseURL).origin;
  const requests = [];
  page.on("request", (request) => requests.push({
    method: request.method(),
    resourceType: request.resourceType(),
    url: request.url()
  }));

  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
    await page.goto(path);
    await page.waitForLoadState("networkidle");
  }

  expect(requests.length).toBeGreaterThan(5);
  expect(requests.filter(({ url }) => new URL(url).origin !== productOrigin)).toEqual([]);
  expect(requests.filter(({ resourceType }) => resourceType === "font")).toEqual([]);
  expect(requests.filter(({ method }) => method !== "GET")).toEqual([]);
});

test("@claim:license-daily-verdict caches a license verdict for one day and contacts only Sociobot", async ({ page, baseURL }) => {
  const productOrigin = new URL(baseURL).origin;
  const token = "valid_test_token_12345";
  const externalRequests = [];
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== productOrigin) externalRequests.push(request.url());
  });
  await page.route("https://api.sociobot.in/**", (route) => route.fulfill({
    status: 200,
    contentType: "application/json",
    body: JSON.stringify({ valid: true, reason: "ok", expires_at: null })
  }));

  const beforeVerification = Date.now();
  await page.goto(`/?license=${token}#license`);
  await expect(page.locator("#license-status")).toContainText("Pro is active");
  const stored = await page.evaluate(() => ({
    token: localStorage.getItem("sb_license:photo-edit-portability-map"),
    verdict: JSON.parse(localStorage.getItem("sb_license:photo-edit-portability-map:verdict"))
  }));
  expect(stored.token).toBe(token);
  expect(stored.verdict).toMatchObject({ token, valid: true });
  expect(stored.verdict.checkedAt).toBeGreaterThanOrEqual(beforeVerification);

  await page.evaluate(() => {
    const key = "sb_license:photo-edit-portability-map:verdict";
    const verdict = JSON.parse(localStorage.getItem(key));
    verdict.checkedAt = Date.now() - 86_400_000 + 60_000;
    localStorage.setItem(key, JSON.stringify(verdict));
  });
  await page.reload();
  await expect(page.locator("#license-status")).toContainText("recent verification");
  expect(externalRequests).toHaveLength(1);

  await page.evaluate(() => {
    const key = "sb_license:photo-edit-portability-map:verdict";
    const verdict = JSON.parse(localStorage.getItem(key));
    verdict.checkedAt = Date.now() - 86_400_000 - 60_000;
    localStorage.setItem(key, JSON.stringify(verdict));
  });
  await page.reload();
  await expect(page.locator("#license-status")).toContainText("Larger verification samples are available");
  expect(externalRequests).toHaveLength(2);
  for (const requestUrl of externalRequests) {
    const url = new URL(requestUrl);
    expect(url.origin).toBe("https://api.sociobot.in");
    expect(url.pathname).toBe("/api/v1/products/photo-edit-portability-map/verify");
    expect(url.searchParams.get("license")).toBe(token);
  }
});

test("@claim:checkout-starts the real Pro action redirects to Sociobot checkout", async ({ page, baseURL }) => {
  test.skip(!baseURL.startsWith("https://"), "uses the live registered checkout endpoint");
  await page.goto("/");
  await page.getByRole("link", { name: "Buy Pro through Sociobot" }).click();
  await expect.poll(() => page.url()).toMatch(/^https:\/\/checkout\.dodopayments\.com\//);
  await expect(page.getByText("Edit Portability Map Pro", { exact: true }).filter({ visible: true })).toBeVisible();
  await expect(page.getByText("$19.00", { exact: true }).filter({ visible: true }).first()).toBeVisible();
  await expect(page.getByText("One-time Pro unlock", { exact: false }).filter({ visible: true })).toBeVisible();
});

test("each published page has a semantic shell and no serious accessibility issues", async ({ page }) => {
  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/404.html"]) {
    const errors = [];
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    await page.goto(path);
    await expect(page.locator("html")).toHaveAttribute("lang", "en");
    await expect(page.locator("main")).toHaveCount(1);
    await expect(page.locator("h1")).toHaveCount(1);
    await expect(page.getByRole("link", { name: "Skip to main content" })).toHaveCount(1);
    await expect(page.locator("footer")).toContainText("Built by Param Factory");
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((item) => ["serious", "critical"].includes(item.impact))).toEqual([]);
    expect(errors).toEqual([]);
  }
});

test("keyboard users can skip content, copy commands, and operate the sample controls", async ({ page }) => {
  await page.goto("/demo/");
  await page.keyboard.press("Tab");
  await expect(page.getByRole("link", { name: "Skip to main content" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.locator("main")).toBeFocused();
  await page.getByRole("button", { name: "Copy demo command" }).focus();
  await page.keyboard.press("Enter");
  await expect(page.getByRole("status")).toContainText("Command copied");
  await page.getByLabel("Target profile").focus();
  await page.keyboard.press("ArrowDown");
  await expect(page.locator("#profile-note")).not.toHaveText("");
});

test("mobile home and demo fit and visible targets meet the touch target baseline", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "390px regression");
  for (const path of ["/", "/demo/"]) {
    await page.goto(path);
    const metrics = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, width: document.documentElement.scrollWidth }));
    expect(metrics).toEqual({ viewport: 390, width: 390 });
    const small = await page.locator("a:visible, button:visible, select:visible").evaluateAll((items) => items.map((item) => {
      const box = item.getBoundingClientRect();
      return { name: item.textContent.trim() || item.getAttribute("aria-label"), width: box.width, height: box.height };
    }).filter((item) => item.width < 44 || item.height < 44));
    expect(small).toEqual([]);
  }
});

test("reduced motion removes the entrance animation", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  await expect.poll(() => page.locator(".terrain").evaluate((node) => parseFloat(getComputedStyle(node).animationDuration) * 1000)).toBeLessThanOrEqual(0.01);
});

test("@claim:offline-demo-reload a fresh context reloads the demo offline after the first live visit", async ({ browser, baseURL }) => {
  test.skip(!baseURL.startsWith("https://"), "requires deployed HTTPS service worker");
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto("/demo/");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Check sample photo metadata before migration.");
  await expect(page.locator("#demo-report-body tr")).toHaveCount(5);
  await context.close();
});
