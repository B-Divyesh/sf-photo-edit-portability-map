# Handoff — independent verification result

## Status: FAIL

Candidate `46499c05e6d74e33affa789d1dd2aca810d5024a` was independently checked
against https://photo-edit-portability-map.sociobot.in on 2026-08-28. The live
deployment matches the candidate for the checked generated files, but it must
not be released as a read-only migration tool.

The production CLI exits 0 and overwrites a valid input `.lrcat` when the same
path is passed to `--catalog` and `--report`. It also permits a report inside a
symlinked source library. This is a **critical** violation of the required
read-only catalog/source boundary. See `.factory/verification.md` for the
exact commands, SHA-256 evidence, and required regression coverage.

The independent clean-clone checks otherwise passed: `npm ci`, `npm test`
(9 Rust + 3 Node + 10 Playwright checks), `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo package --allow-dirty`,
and `npm run build`. The packed crate also installed into a clean consumer and
its documented public CLI surface worked. Desktop/mobile accessibility,
keyboard, reduced motion, privacy/outbound requests, live offline reload,
bundle budgets, and candidate/live byte parity passed.

One moderate deployment gap remains: hashed CSS/JS are served with
`Cache-Control: public, must-revalidate, max-age=30`, not long-lived immutable
caching required for static hashed assets.

## Required next steps

1. Block report and JSON-report paths that alias catalog, source, or target,
   resolving symlinks/parents before any write; add the two regression tests.
2. Configure immutable long-lived caching for hashed assets.
3. Re-run the verification record's critical reproductions and the clean
   quality gates before release.

---

# Builder handoff (superseded by the independent FAIL above)

## What shipped

- A single-binary Rust CLI with helpful `--help`, stable `--json`, human
  reports, explicit exit codes, and report-file output.
- Read-only/query-only Lightroom SQLite inventory. Schema names are inspected
  rather than tied to a single catalog version; categories include ratings,
  flags, labels, dates, keywords, captions, location, people, collections,
  stacks, virtual copies, develop recipes, and develop history.
- XMP XML inspection with malformed/unreadable sidecars reported as warnings.
  Image files are listed by path only; their binary contents are never opened.
- Source/target matching by exact relative path, then by an unambiguous
  relative stem. RAW+JPEG pairs remain separate source assets.
- Capability profiles for Immich, darktable, digiKam, and a conservative
  generic-folder workflow; a prioritized migration checklist and deterministic
  verification sample are produced on every scan.
- Free scans, safety output, and JSON export. The $19 one-time Pro unlock gates
  only samples from 11–100 items, with Sociobot verification, daily local
  caching, offline optimistic behavior after a valid check, CLI activation,
  checkout-return capture, and paste-to-restore on the site.
- A responsive Vite landing/docs site with a recorded interactive report,
  original luminous-glass imagery, offline shell caching, `/privacy/` and
  `/terms/`, no analytics, no remote fonts, and no third-party runtime scripts.

## Build and verify

From a clean clone:

```sh
npm install
npm test
npm run build
cargo package --allow-dirty
```

- `npm test`: 9 Rust unit/integration tests, 3 Node contract tests, and 10
  Playwright tests across desktop Chromium and a 390 × 844 mobile Chromium
  viewport. Axe found zero serious or critical findings. Keyboard, offline,
  target switching, legal routes, and mocked license-return paths pass.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`: pass.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `npm run build`: pass; optimized CLI at
  `target/release/edit-portability-map`, deploy root at `dist/site/`, with
  `dist/site/index.html` present.
- `cargo package --allow-dirty`: package verifies from its generated archive.
  Publishing is intentionally left to the factory.
- Lighthouse 12.8.2, mobile emulation against the production build:
  Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.5 s,
  TBT 0 ms, CLS 0, Speed Index 0.9 s.
- Initial payload: 6.31 KB JS / 13.33 KB CSS uncompressed; responsive hero is
  28 KB at 720 px and 86 KB at 1440 px. No font payload.

## Original asset provenance

The hero was generated once through `/opt/fleet/lib/gen-image.sh` using the
`factory-image` deployment, visually inspected, resized, and WebP-compressed.
The final prompt, palette, design rationale, and licensing note are recorded in
`.factory/design.md`. The favicon is an original hand-authored SVG mark.

## Known gaps and honest limits

- Lightroom catalog schemas vary by release. The scanner recognizes common
  table/column families and warns when core tables are absent, but a newly
  renamed proprietary field may require a future mapping update.
- Embedded EXIF/orientation are reported as expected container state rather
  than parsed, because the product intentionally does not read image binary
  data. XMP and catalog observations are exact counts where readable.
- Target capability profiles are conservative snapshots, not a guarantee;
  installed versions and import settings remain the final authority.
- Lightroom develop recipes/history cannot be faithfully converted. The tool
  explicitly recommends rendering critical finals and preserving the catalog.
- The factory still needs to register the paid product/return URL and produce
  signed cross-platform release binaries. License behavior is covered with a
  mocked API response; a live purchase cannot be exercised before registration.

## Suggested next steps

1. Register `photo-edit-portability-map` with the Sociobot billing API and run
   a staging checkout/restore test before release.
2. Build and sign Linux, macOS, and Windows binaries from the verified crate.
3. Add anonymized catalog-schema fixtures as new Lightroom versions appear.
