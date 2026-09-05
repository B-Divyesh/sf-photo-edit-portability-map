import assert from "node:assert/strict";
import { access, stat } from "node:fs/promises";
import { resolve } from "node:path";
import test from "node:test";

const dist = resolve(import.meta.dirname, "../../dist/site");

test("the production site build contains each published route and required visual assets", async () => {
  const files = [
    "index.html",
    "demo/index.html",
    "privacy/index.html",
    "terms/index.html",
    "404.html",
    "portability-map-card.webp",
    "apple-touch-icon.png",
    "demo-terminal.svg",
    "staticwebapp.config.json"
  ];
  for (const file of files) await access(resolve(dist, file));
  assert.equal((await stat(resolve(dist, "portability-map-card.webp"))).size > 10_000, true);
  assert.equal((await stat(resolve(dist, "apple-touch-icon.png"))).size > 10_000, true);
});
