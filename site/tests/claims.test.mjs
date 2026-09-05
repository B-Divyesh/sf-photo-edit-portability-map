import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import test from "node:test";

function cargoTest(name) {
  return new Promise((resolve, reject) => {
    const child = spawn("cargo", ["test", "--test", "cli", name], { stdio: "pipe" });
    let stderr = "";
    child.stderr.on("data", (chunk) => { stderr += chunk; });
    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) resolve();
      else reject(new Error(stderr || `cargo test exited ${code}`));
    });
  });
}

test("@claim:cli-demo the demo command produces a populated report in its own temporary folder", async () => {
  await cargoTest("claim_demo_command_creates_a_populated_isolated_sample_report");
  assert.ok(true);
});

test("@claim:read-only-scan scanning preserves supplied source, target, and catalog bytes", async () => {
  await cargoTest("claim_readonly_scan_keeps_source_target_and_catalog_bytes_unchanged");
  assert.ok(true);
});

test("@claim:json-and-profiles JSON output works with every documented target profile", async () => {
  await cargoTest("claim_json_report_and_target_profiles_are_scriptable");
  assert.ok(true);
});

test("@claim:no-image-decoding opaque image placeholders still produce an inventory", async () => {
  await cargoTest("claim_scan_lists_opaque_image_placeholders_without_decoding_them");
  assert.ok(true);
});

test("@claim:pro-sample-limit the free CLI accepts ten files and requires Pro above ten", async () => {
  await cargoTest("claim_free_sample_limit_accepts_ten_without_a_license");
  await cargoTest("pro_sized_sample_requires_a_license");
  await cargoTest("claim_pro_license_accepts_the_hundred_file_sample_limit");
  assert.ok(true);
});
