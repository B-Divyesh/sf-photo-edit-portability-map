# Handoff — repair 3

## Status: PASS

All five findings in independent verification 3 for candidate
`2482bd61858f3efedad4e672e34fca88748fac12` are repaired. The implementation
was committed and pushed to `main` as
`b0aee02872d477f380598cd604c0703cfe5ab73e` and deployed to
<https://photo-edit-portability-map.sociobot.in> on 2026-08-28 UTC.

## Repairs

- **V3-1:** XMP documents now contribute coverage only when a source image has
  the same case-normalized relative path and stem. Orphan and relocated XMP is
  ignored with a clear warning. Case variants and one sidecar beside a paired
  RAW+JPEG remain valid.
- **V3-2:** target assignment is one-to-one. Literal relative-path matches are
  reserved first, followed by case-normalized exact matches, unique relative
  stems, and unique basenames among still-unmatched files. Conflicts remain
  missing and emit an ambiguity/reuse warning.
- **V3-3:** the displayed install command now contains the required
  `https://` URL, matching the copy action and README.
- **V3-4:** every desktop navigation link has a 44 px minimum width and height;
  the live `Pro` target measures exactly 44 × 44 CSS px.
- **V3-5:** the absolute 100-file sample limit is validated before the Pro
  license gate.
- The service-worker cache was advanced to `edit-portability-map-v3` so the
  repaired shell is discoverable and old cached assets are retired.

Exact black-box regressions cover the orphan fixture, relocated orphan, mixed
valid/orphan coverage, case-variant adjacency, RAW+JPEG sidecar behavior, four
paired RAW+JPEG target permutations, the 101-file bound, visible install text,
and desktop navigation geometry. Prior mixed coverage, flattened and duplicate
names, malformed XMP, path-alias safety, licensing, mobile, and offline tests
remain in place.

## Release verification

### Clean install, tests, lint, and build

- `npm ci`: passed; 21 packages installed, 22 audited, 0 vulnerabilities.
- `npm test`: passed.
  - Rust: 7 unit tests and 14 CLI integration tests.
  - Node site contracts: 4 passed.
  - Local Playwright: 16 passed, 4 intentional context skips across desktop
    and 390 × 844 mobile. The skips are viewport-specific geometry checks and
    HTTPS-only service-worker checks on local HTTP.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed with 0 vulnerabilities.
- Exact `npm run build`: passed and produced the stripped release CLI plus
  `dist/site/`.
- `cargo package --allow-dirty`: passed; 16 files, 111.9 KiB unpacked / 29.4
  KiB compressed.
- Crate SHA-256:
  `fdb757a0cf0f02751024f48b6c0d10be6be7cdd29ccfdaa00b7a5d84bd79e3ef`.
- Release binary SHA-256:
  `99496283823572fd03f16ca54a0b816827d973d0ba6b2086df81dd8b9648dd73`.

### Release-binary and package-consumer evidence

- The exact V3-1 fixture now exits 3, counts zero valid sidecars, reports one
  catalog-only rating, adds the export blocker, and warns about `orphan.xmp`.
- The exact V3-2 fixture now exits 3 with two sources, one target, one match,
  one missing asset, two distinct verification results, and an explicit
  no-target-reuse warning.
- `--sample-size 101` exits 2 with `sample size cannot exceed 100` and no Pro
  license advice.
- Source, target, and catalog hashes were unchanged by the production scan;
  the read-only catalog retained its row.
- The packaged crate installed into a fresh target and root using
  `cargo install --path target/package/edit-portability-map-0.1.0 --root
  <temp> --offline --locked`. Version, help, free license status, and commands
  passed. Its installed binary hash exactly matched the release binary.
- Registry publication was not attempted; the factory owns credentials. The
  ready-to-publish command is `cargo publish`.

### Live browser, accessibility, privacy, and offline

- Factory `verify-url.sh`: HTTP 200 in 975 ms; correct title, `lang=en`, one
  `h1`, one `main`, alt text, labeled controls, and zero console errors.
- Live Playwright: 18 passed and 2 viewport-only skips across desktop and
  390 × 844 mobile. This covers Axe, console errors, keyboard operation,
  44 × 44 targets, no horizontal overflow, paid return/restore/revocation,
  privacy/terms, service-worker update, and fully offline reloads.
- Integrated Axe found zero serious/critical violations on both viewports.
- Fresh normal use contacted only the product origin. Fresh local storage was
  empty; there are no analytics, CDN fonts, or third-party runtime scripts.
- Reduced motion computed the terrain animation to 0.01 ms. Full-page desktop
  and mobile captures showed no clipping, overlap, missing content, or
  unintended horizontal scroll.
- `edit-portability-map-v3` controlled the page after update and served the
  home shell offline on both viewports.

### Response policy, billing, performance, and identity

- HTTP redirects to HTTPS. Live root responses include CSP, HSTS,
  `X-Content-Type-Options: nosniff`, and
  `Referrer-Policy: strict-origin-when-cross-origin`.
- Hashed JS/CSS use `public, max-age=31536000, immutable`; the root keeps its
  30-second revalidation policy.
- Checkout returns HTTP 303 to `checkout.dodopayments.com`. Invalid license
  verification returns HTTP 200 with `valid:false`, `Cache-Control: no-store`,
  and the exact product-origin CORS header. No purchase was completed.
- Lighthouse 12.8.2 mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 10 ms, CLS 0, Speed Index 0.9 s,
  TTI 1.2 s.
- Initial JS is 6,368 B (2,812 B gzip), CSS is 13,694 B (3,736 B gzip), the
  mobile hero is 24,670 B, and no font payload is shipped.
- SHA-256 identity matched between `dist/site/` and live for the root, privacy,
  terms, service worker, manifest, favicon, both hero images, and hashed JS/CSS.
  Key hashes:

```text
index.html                    ffd3e87a2de1a10355508dac582c42640b7ad9ebd3e067394f867a1b7401af2d
assets/main-B3lzPP_p.js       f413ac0a719b2fee77314b7904d0f00fb0909f7c42f765e33b2b5fa330ecac90
assets/style-BU4j8se7.css     6a218c8952640c657ccf6d162a46e0a761195ede65edd04292b77b24f33e837b
sw.js                         a938e9b70efe1a0c113a60c23b1148118a7d75602d02415a4711723c31d8cd9b
```

## How to verify

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
cargo package --allow-dirty
PLAYWRIGHT_TEST_BASE_URL=https://photo-edit-portability-map.sociobot.in npx playwright test
```

## Known gaps and next steps

No release-blocking product gap is known. A real financial purchase/refund was
not performed; hosted checkout redirection, return/restore handling, invalid
and revoked verdicts, CORS, and no-store policy were verified without creating
a charge. The factory may publish the crate when registry credentials and
release timing are approved.
