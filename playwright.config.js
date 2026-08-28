import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./site/tests",
  testMatch: "**/*.e2e.js",
  timeout: 30_000,
  expect: { timeout: 5_000 },
  fullyParallel: true,
  use: {
    baseURL: process.env.PLAYWRIGHT_TEST_BASE_URL ?? "http://127.0.0.1:4173",
    trace: "retain-on-failure"
  },
  projects: [
    { name: "desktop", use: { ...devices["Desktop Chrome"] } },
    { name: "mobile", use: { ...devices["Desktop Chrome"], viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true } }
  ],
  webServer: process.env.PLAYWRIGHT_TEST_BASE_URL ? undefined : {
    command: "npx vite preview --config site/vite.config.js --host 127.0.0.1",
    url: "http://127.0.0.1:4173",
    reuseExistingServer: true
  }
});
