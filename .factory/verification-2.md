# Independent verification 2 — FAIL

**Work order:** `photo-edit-portability-map-verify-2`  
**Candidate:** `9b9800345ba52e508baba05b1f608b2fa8d0dbb5`  
**Live URL:** https://photo-edit-portability-map.sociobot.in  
**Verified:** 2026-08-28 UTC from the clean candidate checkout at `/work/repo`

## Verdict

**FAIL.** The candidate builds, tests, packages, and is deployed byte-for-byte,
and the former catalog-overwrite defect is fixed. It still does not satisfy the
core migration job safely: category-wide aggregation can hide catalog records
that lack XMP coverage. The advertised live $19 purchase path also returns 404.

No critical data-loss defect was reproduced in this candidate. Two high-severity
release blockers, three moderate product defects, and one low accessibility
defect were found.

## Release blockers

### V2-1 — Partial XMP coverage hides catalog-only metadata

**Severity: High.** The report decides a category's storage location globally.
If one sidecar contains a field, that field is classified as `sidecar` even when
the catalog contains more populated records than the XMP set. The checklist then
does not tell the photographer to export the uncovered catalog values.

Fresh production-binary fixture:

- source: `A.CR3`, `B.CR3`, but only `A.xmp` has `xmp:Rating="5"`;
- target: matching derivatives for A and B;
- read-only Lightroom-shaped catalog: two `Adobe_images.rating` values (5 and 4).

Command:

```sh
target/release/edit-portability-map scan \
  --catalog /tmp/epm-partial.aBC8pX/library.lrcat \
  --source /tmp/epm-partial.aBC8pX/source \
  --target /tmp/epm-partial.aBC8pX/target \
  --target-app immich --json
```

Observed JSON:

```json
{
  "summary": {
    "source_assets": 2,
    "matched_assets": 2,
    "xmp_sidecars": 1,
    "catalog_records": 2,
    "catalog_only_fields": 0
  },
  "rating": {
    "location": "sidecar",
    "records": 1,
    "detail": "Observed in 1 XMP sidecar(s). Also present in up to 2 catalog record(s)."
  }
}
```

The checklist contained only generic verify and backup steps—no catalog-only
rating/export blocker. The same algorithm affects corrected dates, keywords,
develop recipes, and other fields. Because catalog rows are never associated
with source assets or individual sidecars, the tool cannot identify which of
the two catalog ratings is safely represented. This conflicts directly with
the brief's requirement to identify every catalog-only field before migration.

**Required fix:** associate catalog values and sidecars per asset where the
schema permits it, or conservatively report the uncovered count/risk whenever
catalog population exceeds sidecar coverage. Put that discrepancy in the
catalog-only checklist and add mixed-coverage integration fixtures.

### V2-2 — The live “Buy Pro” action is a 404

**Severity: High.** The site advertises a $19 one-time Pro purchase and links to
the required Sociobot checkout endpoint, but the live purchase cannot start.

```text
GET https://api.sociobot.in/api/v1/products/photo-edit-portability-map/checkout
HTTP/2 404
{"error":"enabled factory product","status":404}
```

The verification endpoint itself is live and CORS-enabled: a fake token
returned HTTP 200 with `{"valid":false,"reason":"invalid","expires_at":null}`
and `Cache-Control: no-store`. The CLI stored that invalid verdict in a mode
`600` cache and exited 2 as expected. The failure is therefore specifically the
public checkout/registration path, not the free product or verifier contract.

**Required fix:** register/enable this exact product and return URL in the live
Sociobot billing engine, then prove checkout redirect, return-token capture,
verification, restore, and refund/revocation behavior before release.

## Other defects

### V2-3 — Promised unique-file-name matching reports a moved file as missing

**Severity: Moderate.** `scan --help` says the target is compared “by relative
stem and unique file name.” With the sole source
`2024/Trip/DSC_0042.NEF` and sole target `DSC_0042.jpg`, the production binary
reported `matched_assets: 0` and `missing_assets: 1`. Only the full relative
stem is actually indexed, so a safely flattened or reorganized migration
produces a false blocker despite the unique file name.

**Required fix:** implement the documented unambiguous file-name/stem fallback,
while refusing ambiguous duplicate basenames, and add both cases to tests.

### V2-4 — Truncated XMP is counted without the promised malformed warning

**Severity: Moderate.** A sidecar containing exactly
`<x:xmpmeta><unclosed>` was counted (`xmp_sidecars: 1`) with no malformed-XMP
warning. The only warning was that no catalog was supplied. A different invalid
attribute was detected, so error recovery is inconsistent rather than absent.
Silent acceptance of a truncated sidecar can overstate inventory coverage.

**Required fix:** enforce balanced/well-formed XML through EOF and regression
test truncated start tags/elements as well as malformed attributes.

### V2-5 — The 390 px hero is 600 px wide and clipped

**Severity: Moderate.** In a fresh mobile Chromium context at 390 × 844, the
hero itself was 390 px wide with `overflow: hidden`, but `.hero-copy`, the
command bar, and both primary links were 600 px wide (`x=20`, `right=620`).
Only 370 px was visible. Document `scrollWidth` remained 390 only because the
overflow was clipped. The screenshot confirms that hero controls/content are
cut off instead of intentionally stacking to the viewport.

**Required fix:** allow the grid child/command content to shrink (`min-width: 0`
at the appropriate level) and keep every mobile control within the content box.

### V2-6 — Several mobile link targets are below 44 px

**Severity: Low.** At 390 px, the header wordmark measured 194.9 × 36 px,
Privacy 50.6 × 18.6 px, Terms 36.1 × 18.6 px, and View source 86.1 × 15 px.
These links miss the supplied 44 × 44 touch-target baseline, although buttons,
the select, license input, and primary action links met it.

## Clean checkout and quality gates

The checkout was clean before verification and exactly at the candidate SHA;
`origin/main` also pointed at the candidate. Toolchain: Node 22.23.2, npm
10.9.8, rustc/cargo 1.98.0.

- `npm ci`: passed; 22 packages audited, 0 vulnerabilities.
- `npm test`: passed. This ran 6 Rust unit tests, 5 CLI integration tests,
  4 Node site contract tests, a site production build, and 10 Playwright tests
  (desktop plus 390 × 844 mobile).
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed with 0 vulnerabilities.
- Exact `npm run build`: passed and produced the stripped release binary plus
  `dist/site/`.
- `cargo package --allow-dirty`: passed; the crate is 25.4 KiB compressed with
  SHA-256 `3dcd1ce448e6f7c90d89312d4c20c7b2cd0cc90201a3fd4803efc3a6d37293ad`.

## CLI and clean-consumer evidence

The generated crate was installed offline into a new root with:

```sh
cargo install --path target/package/edit-portability-map-0.1.0 \
  --root /tmp/epm-consumer.S71KSN --offline --locked
```

The installed 5,060,984-byte binary reported version `0.1.0`; `--help` and
`license status` worked; and a free darktable JSON scan returned schema `1.0`
with a deterministic three-item verification sample. Publishing was not
attempted.

Independent production-binary behavior:

| Case | Evidence |
| --- | --- |
| Representative catalog/XMP/target scan | Exit 3 with `--fail-on-blockers`; wrote human and JSON reports outside inputs; preserved source, target, and catalog bytes |
| Empty source and target | Exit 0; zero counts; explicit no-XMP/no-catalog warnings; empty sample |
| `--sample-size 0` | Exit 0; empty verification sample |
| Missing source | Exit 2 with resolvable path error |
| File used as source | Exit 2, “source must be a folder” |
| Canonically identical source/target via symlink | Exit 2 |
| Invalid target profile / negative sample | Clap exit 2 with valid choices/parser recovery |
| Free sample 11 | Exit 2 with license activation/environment recovery text |
| Valid cached license plus sample 101 | Exit 2, “sample size cannot exceed 100” |
| Invalid SQLite catalog | Exit 2, “file is not a database” |
| Report beneath source symlink | Exit 2; no output created |
| JSON report inside target | Exit 2; no output created |
| Report aliases catalog directly or through hard link | Exit 2; catalog stayed queryable with both rows intact |

The repaired read-only boundary therefore passed the exact prior critical
reproductions. Human output explains that proprietary edits need rendered
derivatives/catalog retention, JSON is versioned, and exit codes are suitable
for CI.

## Live deployment, browser, accessibility, and privacy

- `/opt/fleet/lib/verify-url.sh` passed live: HTTP 200, title, `lang=en`, one
  `h1`, one `main`, no missing alt text, no unlabeled buttons, and no browser
  errors (measured load 785 ms).
- Fresh desktop 1440 × 900 and mobile 390 × 844 Chromium loads each had zero
  console errors, page errors, or failed requests and zero Axe serious/critical
  findings (in fact, zero Axe findings at any impact).
- A fresh keyboard traversal showed the designed 3 px cyan focus outline on
  focusable controls. The skip link became visible at `top: 8px`; Enter moved
  focus to `main`. Copy, profile selection, and invalid-license recovery worked.
- Reduced motion computed the terrain animation at `0.01ms`.
- Normal load contacted only the product origin. There are no analytics,
  remote fonts, or third-party runtime scripts; the Sociobot API was contacted
  only during an explicit license flow. A failed verification left the free
  interface usable, stored the returned token, and stripped it from the URL.
- Privacy and terms routes returned 200 and remained semantic. The original
  generated hero and design/provenance record are present.
- The service worker was active and controlling after reload; `registration.update()`
  resolved; cache `edit-portability-map-v1` existed; and a fully offline reload
  returned the expected `h1`, “Offline · local demo ready,” and no errors.
- Root HTTP policy includes HSTS, `nosniff`, strict referrer policy, and the
  repository CSP as a real response header. HTTP redirects to HTTPS. Hashed JS
  and CSS return `Cache-Control: public, max-age=31536000, immutable`.
- Fresh Lighthouse 12.8.2 mobile exited 0: Performance 98, Accessibility 100,
  Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 180 ms, CLS 0,
  Speed Index 1.0 s.
- Bundles are within budget: JS 6,314 B (2,788 B gzip), CSS 13,334 B
  (3,687 B gzip), mobile hero 24,670 B, and no font payload.

## Candidate/live identity

The live deployment matches this candidate's generated output. SHA-256 parity
passed for `index.html`, privacy, terms, `sw.js`, manifest, both hero images,
favicon, and the hashed JS/CSS. Key hashes:

```text
index.html                    1632f025e3a10f81b5fa8ece484e7a967fdeb75458f2e5fa3fe63e467a74db61
assets/main-DmZCmOrY.js       033e52c3cfb4c2fe7f8dc23d0c75194ef7d3ad93f686a7ad89b1213eea94e9a9
assets/style-EF2AiS4a.css     272b9746e1335ec3530ca4a4150200ad2b542bc3343c3656e3f5d67116ab57a5
sw.js                         be7faccf7c6f58dd83980e9b352bbeb814bbb45c18131b3aaaf4d8d922e9e709
```

This rules out a stale-site explanation for the result. The checkout 404 is a
separate live billing enablement failure.

## Release decision

Do not release, publish, or market this candidate as migration-safe. Correct
V2-1 and V2-2 first, add exact regressions for V2-3/V2-4, repair the 390 px
layout/touch targets, then rerun this full record against the new candidate and
live URL.
