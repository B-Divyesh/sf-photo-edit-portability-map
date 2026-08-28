# Handoff — release-blocking QA repair

## Status: PASS

The independent findings against candidate
`46499c05e6d74e33affa789d1dd2aca810d5024a` were repaired on 2026-08-28 and
deployed to https://photo-edit-portability-map.sociobot.in.

## Repairs

- V-1 (critical): all source, target, catalog, human-report, and JSON-report
  paths are resolved before scanning. Prospective outputs resolve their deepest
  existing ancestor, including symlinks, and normalize the missing suffix.
  Reports are refused if they resolve inside source/target or alias the catalog;
  existing hard-link aliases are also detected. Canonically identical source
  and target folders are refused. Safety failures exit `2` before a scan or
  write.
- Exact regressions invoke the compiled CLI with (1) a valid SQLite
  `Adobe_images` catalog also supplied as `--report`, asserting the original
  bytes and readable row survive, and (2) `--source` through a symlink with a
  report beneath a not-yet-created child, asserting no report is created.
- V-2 (moderate): `staticwebapp.config.json` now applies
  `Cache-Control: public, max-age=31536000, immutable` to `/assets/*`. A Node
  contract test pins the cache and security-header configuration. The same
  config supplies the product CSP as an HTTP response header.
- README and CHANGELOG document the enforced output boundary. The researched
  brief, visual thesis, CLI surface, report schema, free/Pro split, and all
  previously passing behavior are unchanged.

Repair commits:

- `80770da` — protect canonical scan inputs from report writes
- `cef9468` — cache fingerprinted site assets immutably

## Verification evidence

Run from `/work/repo`:

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package --allow-dirty
```

Results:

- Clean `npm ci`: 22 packages audited, 0 vulnerabilities.
- `npm test`: 6 Rust unit tests, 5 Rust CLI integration tests, 4 Node contract
  tests, and 10 Playwright tests passed. Playwright covers desktop Chromium and
  390 × 844 mobile Chromium, axe, keyboard operation, reduced/offline state,
  license return, target switching, and legal routes.
- `cargo fmt --check` and warnings-as-errors Clippy passed.
- `npm run build` produced `target/release/edit-portability-map` and
  `dist/site/`; initial assets remain 6.31 KB JS and 13.33 KB CSS uncompressed,
  with no font payload. The mobile hero remains 24.7 KB.
- `cargo package --allow-dirty` produced
  `target/package/edit-portability-map-0.1.0.crate` (25.8 KB). The archive was
  unpacked into a separate temporary consumer, installed with
  `cargo install --path ... --root ... --offline`, and its `--version`,
  `license status`, and free JSON scan succeeded (`schema_version` 1.0,
  `matched_assets` 1). Publishing was intentionally not performed.
- The factory `verify-url.sh` passed locally and live: HTTP 200, expected title,
  `lang=en`, one `h1`, one `main`, no missing image alt, no unlabeled buttons,
  and no browser errors.
- A separate live browser pass at desktop and 390 × 844 found zero
  serious/critical axe issues, zero console/page errors, no horizontal
  overflow, visible and working skip-link/main keyboard focus, reduced-motion
  duration `0.01ms`, no third-party requests on normal load, and a successful
  offline reload after service-worker activation.
- Live Lighthouse 12.8.2 mobile: Performance 100, Accessibility 100, Best
  Practices 100, SEO 100; LCP 1.2 s, TBT 30 ms, CLS 0, Speed Index 0.9 s.
- Live response policy: root returns HSTS, `nosniff`, strict referrer policy,
  and the restrictive CSP header. Fingerprinted JS and CSS return the required
  one-year immutable cache header.
- Live identity: SHA-256 parity passed for generated `index.html`, JS, and CSS
  (`1632f025...db61`, `033e52c3...e9a9`, `272b9746...7a5`). Privacy and terms
  remain directly available; normal page load sends no analytics or external
  runtime requests.

## Deployment

Built with `npm run build` and deployed from `dist/site/` through the work
order's Azure Static Web Apps factory deployment. Production returned HTTP 200
at https://photo-edit-portability-map.sociobot.in immediately after upload,
and the custom domain reported `Ready` with managed TLS.

## Known limits and next steps

- Lightroom catalog schemas vary by release; the scanner recognizes common
  table/column families and warns on unfamiliar schemas.
- Embedded image metadata is reported as expected container state because the
  privacy boundary intentionally excludes reading image pixels/binaries.
- Proprietary Lightroom develop recipes/history cannot be faithfully
  converted; the report continues to require rendered finals and catalog
  preservation.
- The factory still owns registry publication, signed cross-platform release
  binaries, and live billing registration/checkout validation. The crate is
  ready for publication with `cargo publish` after factory review; no registry
  credential or direct payment provider was used here.
