# Independent verification 3 — FAIL

**Work order:** `photo-edit-portability-map-verify-3`

**Candidate:** `2482bd61858f3efedad4e672e34fca88748fac12`

**Live URL:** <https://photo-edit-portability-map.sociobot.in>

**Verified:** 2026-08-28 UTC from the clean candidate checkout at `/work/repo`

## Verdict

**FAIL.** The candidate installs, tests, builds, packages, and is deployed
byte-for-byte. The previously reported deployment-only purchase failure is not
present: the live checkout now returns HTTP 303 to Dodo. However, two fresh
production-binary cases still make the migration inventory unsafe:

1. an XMP file with no corresponding source image can hide a catalog-only field;
2. one target file can be counted as the match for two distinct same-stem source
   assets.

Both cases can produce exit 0 with `--fail-on-blockers`, contrary to the brief's
requirement to identify every catalog-only field and safely account for source
assets before migration. No critical input-write or data-loss defect was
reproduced. Two high, one moderate, and two low defects were found.

## Release blockers

### V3-1 — An orphan XMP can hide catalog-only metadata

**Severity: High.** The scanner treats every `.xmp` anywhere below `--source` as
sidecar coverage; it does not require an adjacent source image with the same
path/stem. That contradicts the product's own promise to parse adjacent XMP and
can cancel the conservative catalog/XMP count protection added after V2.

Fresh release-binary fixture:

- source: `A.CR3`, `B.CR3`, and `orphan.xmp` (there is no `orphan` image);
- target: `A.jpg`, `B.jpg`;
- read-only Lightroom-shaped catalog: one populated `Adobe_images.rating`;
- `orphan.xmp` contains `xmp:Rating="5"`;
- command includes `--json --fail-on-blockers`.

Observed exit: **0**. Relevant JSON:

```json
{
  "summary": {
    "source_assets": 2,
    "matched_assets": 2,
    "xmp_sidecars": 1,
    "catalog_records": 1,
    "catalog_only_fields": 0
  },
  "rating": {
    "location": "sidecar",
    "records": 1,
    "detail": "Observed in 1 XMP sidecar(s). Also present in up to 1 catalog record(s)."
  },
  "blockers": [],
  "warnings": []
}
```

The unrelated XMP value is therefore treated as coverage for the catalog value,
and the checklist omits the rating export blocker. This is a direct failure of
the success measure to identify every catalog-only field.

**Required fix:** associate XMP documents with actual source assets by normalized
relative path/stem before counting their fields. Ignore and clearly warn about
orphan sidecars. Add exact orphan, relocated-orphan, case-variant, RAW+JPEG, and
mixed valid/orphan regression fixtures.

### V3-2 — One target file satisfies two source assets

**Severity: High.** Matching is computed independently per source item without
reserving a target. An exact target match can also be reused as another source's
same-stem fallback.

Fresh release-binary fixture:

```text
source/photo.CR3
source/photo.JPG
target/photo.JPG
```

Observed exit: **0**. Relevant JSON:

```json
{
  "summary": {
    "source_assets": 2,
    "target_assets": 1,
    "matched_assets": 2,
    "missing_assets": 0
  },
  "verification_sample": [
    { "source": "photo.CR3", "target": "photo.JPG", "status": "matched" },
    { "source": "photo.JPG", "target": "photo.JPG", "status": "matched" }
  ]
}
```

The checklist contains no missing-asset blocker, even though one target path
cannot account for two source files. The implementation already documents a
paired RAW as a separate asset, so silently reusing the JPEG target is internally
inconsistent as well as unsafe for migration planning.

**Required fix:** make source/target pairing one-to-one. Reserve exact matches
first, then apply relative-stem and unique-basename fallbacks only among unmatched
items; unresolved same-stem groups must remain missing/ambiguous and be explained
in the report. Add paired RAW+JPEG permutations to integration tests.

## Other defects

### V3-3 — The primary visible install command is invalid

**Severity: Moderate.** The hero visibly shows:

```sh
cargo install --git github.com/B-Divyesh/sf-photo-edit-portability-map
```

Running that exact command returned exit 101:

```text
error: invalid url `github.com/B-Divyesh/sf-photo-edit-portability-map`: relative URL without a base
```

The Copy button and README include the required `https://`, so there is a
recovery path, but the landing page's primary displayed installation instruction
does not work as written.

### V3-4 — Desktop “Pro” navigation target is narrower than 44 px

**Severity: Low.** At 1440 × 900 the visible header `Pro` link measured
25.30 × 44 CSS px. This misses the supplied 44 × 44 click/touch target baseline.
At 390 × 844, every visible link/button/input/select met the target size.

### V3-5 — `--sample-size 101` reports a license problem before the real bound

**Severity: Low.** In a clean free configuration, `--sample-size 101` exits 2
with “a Pro license is required for samples above 10,” even though the documented
and implemented hard maximum is 100. Range validation happens only after the Pro
gate, so invalid input gives the wrong recovery instruction unless the user first
has a valid license.

## Clean checkout and local quality gates

The checkout began clean and exactly at the candidate; `origin/main` also pointed
to the candidate. Toolchain: Node 22.23.2, npm 10.9.8, rustc/cargo 1.98.0.

- `npm ci`: passed; 21 packages installed, 22 audited, 0 vulnerabilities.
- `npm test`: passed.
  - Rust: 7 unit tests and 9 CLI integration tests, 0 failures.
  - Node site contracts: 4 tests, 0 failures.
  - Local Playwright: 15 passes and 3 intentional context skips across desktop
    and 390 × 844 mobile (desktop-only mobile geometry skip and HTTPS-only
    service-worker skips on local HTTP).
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed with 0 vulnerabilities.
- Exact `npm run build`: passed and produced the stripped release CLI plus
  `dist/site/`.
- `cargo package --allow-dirty`: passed; 16 files, 98.7 KiB unpacked / 27.2 KiB
  compressed.
- Crate SHA-256: `9506b4d4ecd9e30b2840e1cd705a1b52c5bcb271eae169dca17461dbda77f0d2`.
- Release binary SHA-256:
  `9d7b77c6a09d0e4fc6703cca0237167754776035a7f4cb85b0c27c802dd57203`.

## Package consumer and CLI behavior

The generated crate was installed into a fresh root with:

```sh
cargo install \
  --path /work/repo/target/package/edit-portability-map-0.1.0 \
  --root /tmp/epm-consumer-v3.Z78hTt --offline --locked
```

The installed binary reported version 0.1.0, exposed the documented `scan` and
`license` commands, reported the free license state, and had the same SHA-256 as
the exact production binary. A packed-consumer mixed-coverage scan returned
schema 1.0, conservatively reported one catalog-only rating, and exited 3 with
`--fail-on-blockers`. Registry publication was not attempted.

Independent release-binary evidence:

| Case | Result |
| --- | --- |
| Representative 4-source catalog/XMP/target scan | Exit 3; wrote human and JSON reports outside inputs; catalog, source, and target hashes unchanged |
| Exact mixed XMP/catalog rating coverage from V2 | Exit 3; one catalog-only rating and explicit export blocker |
| Unique flattened filename | `2024/Trip/DSC_0042.NEF` matched `DSC_0042.jpg` |
| Duplicate flattened filenames | Both remained missing; ambiguous fallback was refused |
| Truncated XMP | Counted as discovered, contributed no fields, and emitted the exact malformed-EOF warning |
| Empty source/target, sample 0 | Exit 0; zero summary/sample and explicit no-XMP/no-catalog warnings |
| Missing source or file used as source | Exit 2 with resolvable messages |
| Source and target aliases through a symlink | Exit 2 |
| Invalid target or negative sample | Clap exit 2 with valid choices/parser error |
| Free sample 11 | Exit 2 with activation/environment recovery text |
| Invalid SQLite catalog | Exit 2, “file is not a database” |
| Report under symlinked source | Exit 2; no report created |
| Report hard-linked to catalog | Exit 2; catalog remained queryable with both rows |
| Invalid live license activation | Exit 2; mode-600 cache recorded an inactive verdict; free scans remained available |

Normal scanning performs no network call; source inspection and the isolated
browser request audit found no telemetry. The only CLI network path is explicit
license verification. Reports do not contain the license token.

## Live deployment, accessibility, privacy, and PWA

- Factory `verify-url.sh`: HTTP 200; 823 ms measured load; title, `lang=en`, one
  `h1`, one `main`, alt text, labeled buttons, and zero console errors.
- Live Playwright: 17 passes and one intentional desktop skip across desktop and
  390 × 844 mobile. This includes service-worker update/cache assertions and
  fully offline reloads in both contexts.
- Axe: zero serious/critical findings on both live viewports; the integrated run
  found zero violations at any impact.
- Independent browser instrumentation found zero console errors, page errors,
  or failed requests on either viewport. A fresh normal load contacted only
  `https://photo-edit-portability-map.sociobot.in`.
- Keyboard traversal reached the skip link first and every visible interactive
  control without a trap. Focus-visible computed to a 3 px cyan solid outline
  with 3 px offset; Enter activated skip and copy actions; the profile, license,
  and restore/error paths were operable.
- At 390 px, client and document widths were both 390 px, hero/control content
  stayed inside the viewport, and no visible interactive target was below 44 ×
  44 CSS px.
- Reduced motion computed the terrain animation as 0.01 ms. No animation loops.
- Fresh-page local storage was empty. There are no analytics, CDN fonts, or
  third-party runtime scripts. Sociobot is contacted only for an explicit
  license flow. `/privacy/` and `/terms/` return semantic pages.
- Service worker `registration.update()` resolved, cache
  `edit-portability-map-v2` was present, the worker controlled the reload, and
  offline home reloads retained the expected heading and offline status.
- Visual inspection of full-page desktop and mobile captures found no clipping,
  overlap, missing content, or unintended horizontal scroll.

## Live response policy, billing, and performance

- HTTP redirects to HTTPS. Root responses include response-header CSP, HSTS,
  `X-Content-Type-Options: nosniff`, and
  `Referrer-Policy: strict-origin-when-cross-origin`.
- Hashed JS/CSS return `Cache-Control: public, max-age=31536000, immutable`.
  The root and service worker use a 30-second revalidation policy suitable for
  update discovery.
- Live checkout returns HTTP 303 to an HTTPS
  `checkout.dodopayments.com/session/...` URL. No purchase was completed.
- Invalid-token verification returns HTTP 200 with
  `{"valid":false,"reason":"invalid","expires_at":null}`,
  `Cache-Control: no-store`, and the exact product-origin CORS header.
- Lighthouse 12.8.2 mobile: Performance 99, Accessibility 100, Best Practices
  100, SEO 100. FCP 1.0 s, LCP 1.2 s, TBT 100 ms, CLS 0, Speed Index 1.0 s,
  TTI 1.3 s.
- Initial JS is 6,368 B (2,812 B gzip); CSS is 13,656 B (3,737 B gzip); the
  mobile hero is 24,670 B; no font payload is shipped. All supplied static
  budgets pass.

## Candidate-to-live identity

SHA-256 equality passed between `dist/site/` and the live deployment for the
root, privacy, terms, service worker, both hero images, favicon, manifest, and
hashed JS/CSS. Key hashes:

```text
index.html                    af8cc072a4c96c591ea57582ebb79cc1e32d4b80a53bfc5eb798649fc24335a0
assets/main-DaL4kHlW.js       f413ac0a719b2fee77314b7904d0f00fb0909f7c42f765e33b2b5fa330ecac90
assets/style-lKLPsGcu.css     66a4aa025aaba8d74e3d40e4878987269d9e52426c684c36d17f57e99654a6e2
sw.js                         c9826d9cfcd01d013b088ec748e8c6a8d815cffad301680a4c6a21af62bb0ce9
```

This rules out stale deployment or the former checkout failure as the cause of
the release decision.

## Release decision

Do not publish or market this candidate as migration-safe. Correct V3-1 and
V3-2 and add exact black-box regressions before rerunning verification. Also fix
the displayed install URL, desktop navigation target, and sample-bound validation
order. The healthy live deployment and billing route do not mitigate incorrect
CLI inventory results.
