import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const read = (path) => readFile(new URL(path, import.meta.url), "utf8");

test("landing page has the required semantic shell", async () => {
  const html = await read("../index.html");
  assert.match(html, /<html lang="en">/);
  assert.equal((html.match(/<h1[ >]/g) ?? []).length, 1);
  assert.match(html, /<main id="main"[^>]*>/);
  assert.match(html, /alt="Translucent archive layers/);
  assert.match(html, /href="\/privacy\/"/);
  assert.match(html, /href="\/terms\/"/);
});

test("paid unlock follows the Sociobot storage and verification contract", async () => {
  const script = await read("../src/app.js");
  assert.match(script, /sb_license:/);
  assert.match(script, /\/verify\?license=/);
  assert.match(script, /history\.replaceState/);
  assert.match(script, /86_400_000/);
});

test("motion and focus have explicit accessible treatments", async () => {
  const css = await read("../src/style.css");
  assert.match(css, /:focus-visible/);
  assert.match(css, /prefers-reduced-motion: reduce/);
  assert.match(css, /min-height: 44px/);
});

test("deployment gives hashed assets immutable caching and a response CSP", async () => {
  const config = JSON.parse(await read("../public/staticwebapp.config.json"));
  const assets = config.routes.find((route) => route.route === "/assets/*");
  assert.equal(assets.headers["Cache-Control"], "public, max-age=31536000, immutable");
  assert.match(config.globalHeaders["Content-Security-Policy"], /default-src 'self'/);
  assert.equal(config.globalHeaders["X-Content-Type-Options"], "nosniff");
});
