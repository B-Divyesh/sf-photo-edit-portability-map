# Independent verification 4 — PASS

**Work order:** `photo-edit-portability-map-verify-4`  
**Candidate:** `4f9aa66b81da409a94243fba263f635d686b1cb7`  
**Live URL:** <https://photo-edit-portability-map.sociobot.in>  
**Verified:** 2026-08-28 06:49 UTC from a clean checkout at `/work/repo`

## Verdict

**PASS.** The candidate meets the researched CLI job: it performs a read-only
inventory of a Lightroom-shaped SQLite catalog, adjacent XMP, and source/target
folders; conservatively identifies uncovered catalog state; reports destination
capabilities, a checklist, and deterministic verification samples. The exact
production site is deployed. The previously reported live checkout failure is
not reproducible: the configured checkout endpoint returns HTTP 303 to Dodo.

No critical, high, moderate, or low defects were found in this verification.

## Clean checkout and quality gates

The checkout began clean at the requested SHA; `origin/main` was the same SHA.
Toolchain: Node `v22.23.2`, npm `10.9.8`, Rust/cargo `1.98.0`.

- `npm ci`: passed; 21 packages installed and `npm audit --audit-level=high`
  found 0 vulnerabilities.
- `npm test`: passed. It ran 7 Rust unit tests, 14 Rust CLI integration tests,
  4 Node site-contract tests, and local production-site Playwright coverage:
  16 passed / 4 intentional context skips (the mobile-only, desktop-only, and
  HTTPS-only worker checks are each skipped in the inapplicable local project).
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- Exact `npm run build`: passed and produced the stripped release binary and
  `dist/site/`.
- `cargo package --allow-dirty`: passed. The ready-to-publish crate is 30 KB,
  SHA-256 `ba39e3e8506952600f8ad2023e845238378bccdefd3e7472eee2aa0daeb72511`.
  Publishing was not attempted; the factory owns registry credentials.

## CLI end-to-end and safety evidence

The exact `target/release/edit-portability-map` binary has SHA-256
`99496283823572fd03f16ca54a0b816827d973d0ba6b2086df81dd8b9648dd73`.

Fresh black-box fixture: two source RAWs and matching target derivatives; one
adjacent XMP with rating and capture date; a read-only SQLite `Adobe_images`
table with two ratings, two corrected dates, and two develop recipes; target
profile `immich`.

- `scan --json --report ... --json-report ... --fail-on-blockers` exited **3**.
  It reported 2/2 matches, 1 valid sidecar, and catalog-only Corrected capture
  date (1), Star rating (1), and Lightroom develop recipe (2). The checklist
  explicitly says to export/record those categories and identifies the develop
  recipe as Immich-unsupported. Its two verification rows named the exact
  source/target pairs and relevant fields.
- SHA-256 values of both source inputs, both targets, and the catalog were
  identical before and after. The catalog still contained both SQLite rows.
- Empty source/target with `--sample-size 0` exited 0 with zero counts, an
  empty sample, and explicit no-XMP/no-catalog limitations.
- Missing source, `--sample-size 101`, and `--report` equal to `--catalog`
  each exited 2 with actionable messages. The alias attempt did not change the
  catalog.
- The repository integration suite also passed independent regressions for
  mixed catalog/XMP coverage, orphan/relocated XMP, malformed XMP,
  case-normalized sidecars, flattened and duplicate-name target matching,
  one-to-one paired RAW/JPEG assignment, source-symlink report refusal, and
  free/Pro sample bounds.

## Packaged consumer

`cargo install --path target/package/edit-portability-map-0.1.0 --root
/tmp/epm-consumer-9tJYcZ --offline --locked` succeeded in a clean consumer
root. The installed binary was version `0.1.0`, supplied useful `--help`,
reported the free license state without interaction, and had the identical
release SHA-256. A free darktable `--json` scan from that installed binary
returned schema `1.0`, 2/2 target matches, and a deterministic sample.

## Live deployment, privacy, accessibility, and PWA

- Every deployed product artifact matched the local `dist/site/` by SHA-256:
  root, privacy and terms pages, hashed JS/CSS, worker, manifest, favicon,
  robots/sitemap, and both WebP illustrations. Key hashes: root
  `ffd3e87a2de1a10355508dac582c42640b7ad9ebd3e067394f867a1b7401af2d`,
  JS `f413ac0a719b2fee77314b7904d0f00fb0909f7c42f765e33b2b5fa330ecac90`,
  CSS `6a218c8952640c657ccf6d162a46e0a761195ede65edd04292b77b24f33e837b`,
  worker `a938e9b70efe1a0c113a60c23b1148118a7d75602d02415a4711723c31d8cd9b`.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200 in 648 ms, expected title,
  `lang=en`, one h1, a main landmark, no missing image alt, no unlabeled
  buttons, and no console/page errors.
- Live Playwright at desktop and 390 x 844 mobile: **18 passed, 2 intentional
  viewport-specific skips**. It independently covered serious/critical Axe
  findings (none), console errors (none), normal-use outbound requests
  (product origin only), skip-link keyboard use and copy feedback, focusable
  44 px controls, profile selection, checkout-return token stripping,
  restore/revocation recovery, legal routes, worker update, and offline
  reload. The worker cache is `edit-portability-map-v3`; after update it
  controlled the page and served the expected shell offline.
- Reduced motion is explicitly covered by the site contract test; the visual
  design has a product-specific documented palette, system-font treatment,
  motion policy, and generated-asset provenance in `.factory/design.md`.
- Fresh normal page load did not contact third parties. Source/build review
  found no analytics, CDN scripts, fonts, cloud credentials, or image decoding.
  The only optional remote request is Sociobot license verification after a
  user supplies a token. Local storage is limited to an opt-in license token
  and its daily verdict cache. The CLI writes only requested reports outside
  input trees and its user-config license cache; it never writes source,
  target, or catalog metadata.

## HTTP policy, billing, and budgets

- HTTP redirects to HTTPS. Live responses provide CSP, HSTS, `nosniff`, and
  `Referrer-Policy: strict-origin-when-cross-origin`. The hashed JS/CSS assets
  return `Cache-Control: public, max-age=31536000, immutable`; the worker is
  short-lived/revalidating, appropriate for updates.
- Invalid-license verification returns HTTP 200 JSON with `valid:false`, the
  exact product-origin CORS allow header, and `Cache-Control: no-store`.
  Checkout returns HTTP 303 to `checkout.dodopayments.com`. No financial
  purchase was made.
- Built JS is 6,368 B (2,788 B gzip); CSS is 13,694 B (3,736 B gzip); the
  mobile hero is 24,670 B; no font payload is shipped. All stated transfer
  budgets pass.
- Fresh Lighthouse 12.8.2 mobile (using the supplied Playwright Chromium):
  Performance 98, Accessibility 100, Best Practices 100, SEO 100; FCP 1.9 s,
  LCP 1.9 s, TBT 20 ms, CLS 0, Speed Index 1.9 s, interactive 2.0 s.

## Reproduce

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
cargo package --allow-dirty
PLAYWRIGHT_TEST_BASE_URL=https://photo-edit-portability-map.sociobot.in npx playwright test --reporter=line
```

Use `cargo install --path target/package/edit-portability-map-0.1.0 --root
<clean-root> --offline --locked` to repeat the consumer install. The factory
may publish with `cargo publish` when its credentials and release timing allow.
