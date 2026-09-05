# Verify a Lightroom metadata migration inventory — Independent verification 6

**Verdict: PASS.** There are **zero findings** at every severity and **zero untested public claims**.

- Work order: photo-edit-portability-map-verify-6
- Live URL: https://photo-edit-portability-map.sociobot.in
- Implementation reviewed: f13374d266494b2aa81403f1c895d411d1d51c03
- Documentation reviewed: 2b0e627a03cbd2e641c3694e8541a2fec2fff03f
- Verified: 2026-09-05 UTC

The implementation candidate differs from the documentation SHA only in .factory/handoff.md. The rebuilt candidate and the live deployment match byte-for-byte for all checked published files.

## Job, audience, and first action

The job is to map Lightroom metadata before moving photos. It is for photographers leaving Lightroom or checking a read-only external library. The first action is **Try it with sample data**. It runs a safe sample scan in a temporary folder.

Fresh 1440 × 900 desktop and 390 × 844 phone browser contexts showed all three facts before scrolling. The fresh browser captures are stored in /work/.evidence/photo-edit-portability-map-verify-6/. The phone home and demo documents each had a viewport width and scroll width of exactly 390 px.

One click opened /demo/ with the persistent **Demo — sample data, nothing is saved** label, Reset demo, and Start for real. The report contained five realistic rows, including embedded EXIF, sidecar keywords, catalog-only corrected date and star rating, and an unsupported Lightroom develop recipe. Reset returned the profile to Immich. Leaving demo removed only the demo:photo-edit-portability-map:state key and preserved a seeded real-data key.

## Clean setup and artifact check

A new clone at 2b0e627 was used. npm ci passed with 0 vulnerabilities. The following commands passed:

- npm test: 7 Rust unit tests, 20 Rust CLI integration tests, 6 Node tests, and the local browser suite.
- cargo fmt --check
- cargo clippy --all-targets -- -D warnings
- npm audit --audit-level=high
- npm run build, producing dist/site/ and the release binary
- cargo package --allow-dirty, producing a 31.0 kB crate

The packaged crate installed offline into a new consumer root with cargo install --path target/package/edit-portability-map-0.1.0 --root <clean-root> --offline --locked. Its installed binary had useful help and its demo command created a new temporary directory containing populated text and JSON reports. The installed and release binaries had the identical SHA-256: f54af446fe842f5ed42d21de9916d2c19b4b0d9c7d3ffc563e322cee3e78d5f4. The packaged crate SHA-256 was 1dcfaa721af25e50bc08e7ed0f53b021c8de76a53e3b5d3548eb3e5e82dac68e.

Normal, invalid, boundary, and recovery CLI paths are covered by the passing black-box suite: empty inputs, missing paths, malformed and orphan XMP, one-to-one matching, input/report aliases and symlinks, free 10-file and Pro 100-file limits, profile JSON, blockers, and locally recorded license-verdict recovery. No image decoder is required for opaque image placeholders.

## Public claims

Every command declared in .factory/claims.json was run separately from the clean checkout. All 14 of 14 passed; no public claim was omitted.

| Claim | Result |
| --- | --- |
| cli-demo | Passed: a populated sample report is written in a new temporary folder. |
| read-only-scan | Passed: source, target, and catalog input bytes remain unchanged. |
| json-and-profiles | Passed for generic, Immich, darktable, and digiKam. |
| no-image-decoding | Passed with opaque invalid JPEG and WebP fixture bytes. |
| pro-sample-limit | Passed at free 10-file, rejected 11-file, and accepted licensed 100-file boundaries. |
| demo-populated-output | Passed: one click opens the labelled five-row report. |
| demo-isolated | Passed: reset and exit affect only the demo: storage key. |
| demo-no-upload | Passed: the full demo flow requests only the product origin. |
| site-runtime-privacy | Passed on home, demo, Privacy, Terms, and 404; only product-origin GET requests and no font requests. |
| demo-profile | Passed: changing target profile changes the visible capability result. |
| license-restore | Passed with a recorded invalid Sociobot response; storage and URL cleanup work. |
| license-daily-verdict | Passed: local verdict reuse is under one day; stale verdict refreshes only through the Sociobot verification endpoint. |
| checkout-starts | Passed live: Buy Pro reached Dodo checkout with Edit Portability Map Pro, $19.00, and one-time wording. No payment was entered. |
| offline-demo-reload | Passed live in a new HTTPS browser context after service-worker installation. |

The landing page, demo, legal pages, and README were cross-checked against the registry. Their relied-on privacy, read-only, demo, profile, sample-limit, license, checkout, and offline statements have observable claim coverage.

## Live site checks

PLAYWRIGHT_TEST_BASE_URL=https://photo-edit-portability-map.sociobot.in npx playwright test --reporter=line passed with **27 passed and 1 expected viewport-specific skip**. It exercised desktop and phone views, keyboard skip navigation, visible focus, copy and profile controls, 44 px touch targets, reduced motion, demo isolation, privacy requests, legal routes, checkout, license caching, and offline reload.

The integrated Axe checks reported zero serious and zero critical issues on home, demo, Privacy, Terms, and 404. The required verify-url.sh check passed: HTTP 200, title, lang=en, one h1, a main landmark, complete image alt text, labelled buttons, and no console or page errors. It measured a 564 ms load in its desktop browser.

The home, demo, Privacy, Terms, and 404 routes each return HTTP 200 and have their own plain route title. A deliberately missing route returns the designed not-found page with HTTP 404, which is expected. Every product link resolved: local links returned 200, the repository link returned 200, and the registered checkout returned its expected 303 to Dodo. Privacy and Terms are present. The root response includes response-header CSP with frame-ancestors 'none', HSTS, nosniff, and a strict referrer policy.

The rebuilt candidate matched the live bytes for 17 files: every HTML route, service worker, manifest, robots and sitemap files, favicon, both WebP hero sizes, social image, touch icon, terminal recording, JavaScript, and CSS. The build has 6.88 kB JavaScript (2.78 kB gzip), 16.50 kB CSS (4.22 kB gzip), and no web-font payload. Existing candidate-matched Lighthouse evidence records 100/100/100/100. A fresh Lighthouse CLI retry could not connect to its Chrome launcher in this worker, although Playwright used that Chromium successfully; it produced no new report and is not a product finding.

This is a static site and local CLI. There is no product backend, tenant data, server persistence, health endpoint, or live 429 path, so backend-only tenant isolation, restart-persistence, health, and Retry-After checks do not apply.

## Earlier findings

| Earlier finding set | Current disposition |
| --- | --- |
| Verification 1: report overwrite and asset caching | Closed: catalog aliases and report paths through source symlinks are refused by black-box tests; live hashed assets are immutable. |
| Verification 2 V2-1–V2-6 | Closed: mixed-XMP handling, moved-file matching, malformed-XMP warning, checkout, 390 px hero, and mobile target tests pass. |
| Verification 3 V3-1–V3-5 | Closed: orphan XMP, one-to-one targets, install command, desktop target, and sample-bound regressions pass. |
| Review 1 and Review 2 F1–F7 | Closed: runnable isolated demo, claims registry, plain first screen, route and 404 behavior, metadata shell, copy audit, and release status are present and verified. |
| Review 3 R3-1 and R3-2 | Closed: the full 390 px home/demo width regression passes, and exact runtime-privacy and daily-license-verdict claim tests pass. |

## Result

**PASS — finding count: 0; untested public claim count: 0.** The crate remains packaged and consumer-tested but unpublished because the factory owns registry publication credentials. No payment or refund was performed.

