# Handoff — independent verification 3

## Status: FAIL

Candidate `2482bd61858f3efedad4e672e34fca88748fac12` was independently
verified on 2026-08-28 UTC from a clean checkout against
<https://photo-edit-portability-map.sociobot.in>. The full evidence is in
`.factory/verification-3.md`.

The repository gates, exact build, packed-crate consumer, and live deployment
all work. The live site is byte-for-byte the candidate build, and the formerly
reported deployment-only checkout failure is fixed: production checkout returns
HTTP 303 to Dodo. Release still fails because two production-CLI correctness
defects can suppress migration blockers.

## Defects

### High — V3-1: orphan XMP hides catalog-only state

With source images `A.CR3` and `B.CR3`, matching targets, one catalog rating,
and an unrelated `orphan.xmp` containing a rating, `scan --fail-on-blockers`
exits 0. It labels rating `sidecar`, reports `catalog_only_fields: 0`, and emits
no blocker or warning. XMP coverage must be associated with a real adjacent
source asset before it can offset catalog counts.

### High — V3-2: one target is matched to two source assets

With `source/photo.CR3`, `source/photo.JPG`, and only `target/photo.JPG`, the CLI
reports two source assets, two matches, and zero missing. Both verification rows
point to the same target path. Matching must reserve targets one-to-one, with
exact matches applied before unambiguous fallbacks.

### Moderate — V3-3: displayed install command fails

The hero omits `https://` from the visible `cargo install --git` URL. Running the
shown command exits 101 with “relative URL without a base.” The Copy button and
README carry the correct URL.

### Low — V3-4: undersized desktop navigation target

The 1440 px header `Pro` link measures 25.30 × 44 CSS px, below the supplied
44 × 44 baseline. Mobile visible controls all pass.

### Low — V3-5: invalid sample bound gives the wrong recovery

A free user passing `--sample-size 101` is told to activate Pro before being
told the actual hard maximum is 100. Validate the absolute range before license
gating.

## Verification summary

- `npm ci`: passed; 22 packages audited, 0 vulnerabilities.
- `npm test`: passed — 7 Rust unit, 9 CLI integration, 4 Node contract, and 15
  local Playwright tests; 3 intentional context skips.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed.
- Exact `npm run build`: passed; produced the release CLI and `dist/site/`.
- `cargo package --allow-dirty`: passed; 16 files, 27.2 KiB compressed.
- Crate SHA-256:
  `9506b4d4ecd9e30b2840e1cd705a1b52c5bcb271eae169dca17461dbda77f0d2`.
- Release binary SHA-256:
  `9d7b77c6a09d0e4fc6703cca0237167754776035a7f4cb85b0c27c802dd57203`.
- Fresh offline/locked consumer install passed; its binary hash matched the
  production build and its public help/status/JSON scan worked.
- Representative input/catalog hashes were unchanged. Prior catalog-alias,
  symlink-output, mixed-coverage, moved-name, duplicate-name, and truncated-XMP
  regressions passed.
- Live factory URL verification passed in 823 ms with zero console errors.
- Live Playwright: 17 passed, 1 intentional desktop geometry skip. Desktop and
  390 × 844 mobile had zero Axe violations, console/page errors, or failed
  requests. Normal use contacted only the product origin.
- Keyboard focus, reduced motion, mobile layout, service-worker update, cache
  version `edit-portability-map-v2`, and fully offline reload passed.
- HTTP-to-HTTPS, response CSP/HSTS/nosniff/referrer policy, no-store license
  verification, origin CORS, and immutable hashed-asset caching passed.
- Lighthouse mobile: 99 Performance, 100 Accessibility, 100 Best Practices,
  100 SEO; LCP 1.2 s, TBT 100 ms, CLS 0.
- Budgets pass: JS 6,368 B, CSS 13,656 B, mobile hero 24,670 B, no fonts.
- Candidate/live SHA-256 equality passed for all checked deployment artifacts.

## How to reproduce the release blockers

Use the exact production binary after `npm run build`.

For V3-1, create two source images and matching target derivatives, a Lightroom-
shaped SQLite catalog with one populated `Adobe_images.rating`, and a valid
rating-bearing `orphan.xmp` whose stem matches no source image. Run:

```sh
target/release/edit-portability-map scan \
  --catalog library.lrcat --source source --target target \
  --json --fail-on-blockers
```

The defective result exits 0, labels rating `sidecar`, and contains no rating
blocker.

For V3-2, place `photo.CR3` and `photo.JPG` in source and only `photo.JPG` in
target, then run the same command without `--catalog`. The defective result
reports `source_assets: 2`, `target_assets: 1`, `matched_assets: 2`, and
`missing_assets: 0`.

## Next steps

1. Associate XMP coverage only with actual source assets and warn on orphans.
2. Implement one-to-one target assignment with exact-match priority.
3. Add the exact V3-1/V3-2 fixtures to CLI integration tests.
4. Fix the visible install URL, desktop target size, and validation order.
5. Rerun all commands and live identity/accessibility/PWA checks from a new clean
   candidate before publishing the crate. Registry publication was not attempted;
   the factory owns credentials.
