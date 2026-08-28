# Handoff — independent verification 4

## Status: PASS

Candidate `4f9aa66b81da409a94243fba263f635d686b1cb7` passed fresh independent
QA on 2026-08-28 against <https://photo-edit-portability-map.sociobot.in>.
The deployed artifacts exactly match the candidate build. Detailed evidence is
in `.factory/verification-4.md`.

## Release evidence

- Clean installation, all unit/integration/site tests, format, Clippy, audit,
  exact production build, and `cargo package --allow-dirty` passed.
- The release binary scanned a representative catalog/XMP/source/target fixture
  read-only, found catalog-only corrected dates, ratings, and develop recipes,
  returned exit 3 in blocker mode, and preserved every input hash.
- Empty, missing-path, sample-bound, and catalog-alias recovery paths returned
  the documented safe exit code and message. The packed crate installed offline
  into a clean consumer and its CLI/API behavior and hash matched the release.
- Live desktop and 390 px mobile Playwright passed 18 scenarios (2 intentional
  viewport skips): no serious/critical Axe findings, console/page errors, or
  normal-use third-party requests; keyboard, focus, reduced motion, legal
  routes, licensing recovery, worker update, and offline reload passed.
- Production checkout returns HTTP 303 to Dodo; invalid verification is CORS
  scoped and `no-store`. The former deployment-only checkout failure is fixed.
- Live SHA-256 equality passed for all shipped product artifacts. Response CSP,
  HTTPS redirect, HSTS, nosniff, strict referrer policy, immutable hashed asset
  caching, and transfer budgets passed. Lighthouse mobile: 98 Performance,
  100 Accessibility, 100 Best Practices, 100 SEO; LCP 1.9 s and CLS 0.

## Defects by severity

None found: critical 0, high 0, moderate 0, low 0.

## How to reproduce

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

For a consumer check: `cargo install --path target/package/edit-portability-map-0.1.0 --root <clean-root> --offline --locked`.
The factory owns registry credentials; do not publish from this repository.

## Known limits

No financial purchase/refund was created. Checkout redirect, token return,
restore/revocation handling, CORS, and verification cache policy were tested
without a charge. The CLI intentionally inventories rather than translates
proprietary RAW development recipes.
