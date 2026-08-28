# Independent verification — FAIL

**Work order:** `photo-edit-portability-map-verify-1`  
**Candidate:** `46499c05e6d74e33affa789d1dd2aca810d5024a`  
**Live URL:** https://photo-edit-portability-map.sociobot.in  
**Verified:** 2026-08-28, from a clean clone at `/tmp/photo-portability-qa.GAqVnx`

## Verdict

**FAIL.** The candidate violates the product's non-negotiable read-only
catalog constraint. A user can cause `scan` to overwrite the supplied
Lightroom catalog by using the catalog path as `--report`. This is data loss
on the core input, despite the CLI reporting success.

## Critical defect

### V-1 — `--report` can overwrite the input Lightroom catalog

**Severity: Critical.** `ensure_output_is_safe` rejects output paths inside
`--source` and `--target`, but it never rejects the optional `--catalog` path.

Fresh reproduction using the production binary:

```sh
# `library.lrcat` is a valid SQLite file containing Adobe_images.
before=$(sha256sum "$qa_catalog/library.lrcat")
target/release/edit-portability-map scan \
  --catalog "$qa_catalog/library.lrcat" \
  --source "$qa_catalog/source" \
  --target "$qa_catalog/target" \
  --report "$qa_catalog/library.lrcat"
# exit 0
after=$(sha256sum "$qa_catalog/library.lrcat")
```

Observed result: the SHA-256 changed from
`bd540952b665483b4a3ab871cd76be345cd065f633596fb01d8af68dccdaec8b` to
`1f6e31b5edd20451aecfd3b79ba687442e980ba363baa176ba96b4cbdcf86b52`; the
file's new prefix was `EDIT PORTABILITY MAP / generic`. The valid SQLite
catalog was replaced by the human report, with no warning or non-zero exit.

The same incomplete guard also permits an output inside the scanned source
when `--source` is a symlink: a scan using `link-source` and
`--report link-source/symlink-report.txt` exited 0 and wrote to the real
source directory. This reinforces that the input-write safety boundary is not
reliable.

**Required fix before release:** resolve/canonicalize all input and output
paths (including existing parents/symlinks) and reject any report or JSON
report that aliases the catalog, source, or target. Add regression tests for
the exact catalog-alias and symlink cases.

## Additional finding

### V-2 — deployment cache policy misses the static-asset requirement

**Severity: Moderate.** The live hashed JS and CSS assets return
`Cache-Control: public, must-revalidate, max-age=30`, rather than the required
long-lived immutable policy for hashed static assets. This does not block
correctness, but fails the supplied performance/caching acceptance criterion
and causes needless revalidation.

## Checks that passed

### Clean install, quality gates, and package consumer

- Clean clone at the candidate commit: `npm ci` completed with 0 audit
  vulnerabilities.
- `npm test`: passed — 6 Rust unit tests, 3 Rust CLI integration tests, 3 Node
  contract tests, and 10 Playwright tests (five scenarios on each desktop and
  390 × 844 mobile Chromium viewport).
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo package --allow-dirty`, and the exact `npm run build` all passed.
  The build produced `target/release/edit-portability-map` and `dist/site/`.
- The generated `.crate` was unpacked and installed offline into a separate
  consumer root with `cargo install --path ... --root ... --offline`.
  Its public `--help`, `license status`, and free JSON `scan` command worked.
- Manual representative run with source/target files, XMP rating/date, and a
  Lightroom-shaped SQLite catalog correctly reported embedded, sidecar, and
  catalog-only categories; wrote a JSON report outside inputs; preserved the
  catalog SHA-256; and returned exit code 3 for blockers. Boundary/recovery
  checks also passed for missing source (2), identical source/target (2),
  source-local report refusal (2), sample size 0, and Pro sample 11 without a
  license (2 with clear recovery text).

### Browser, privacy, and deployment

- Local production build, desktop, and 390 px mobile: one `h1`, title/lang/main
  present; no console or page errors; axe reported zero serious/critical
  violations; no horizontal overflow at 390 px; reduced-motion animation
  duration was `0.01ms`.
- Keyboard-only smoke test on the live mobile viewport passed: the skip link
  received a visible 3 px cyan focus ring, Enter moved focus to `main`, and
  target-profile keyboard selection updated its live description.
- Normal load made no outbound requests. The static site has no analytics or
  remote fonts/scripts; the only product network endpoint in the bundle is the
  documented Sociobot license API, used only after a license is provided.
- The live service worker activated and controlled the page after reload;
  an offline reload of the live home page succeeded with the expected `h1` and
  no errors.
- Root response headers include HSTS, `nosniff`, and a strict referrer policy.
  The root document supplies its restrictive CSP by meta tag; the server does
  not send a CSP header. `/privacy/` and `/terms/` are available and semantic.
- Candidate-to-live parity was checked by SHA-256 for `index.html`, hashed JS,
  hashed CSS, `sw.js`, privacy/terms pages, and the 720 px WebP: all matched.
- Bundle sizes pass the stated transfer budgets: JS 6,314 B (2,788 B gzip),
  CSS 13,334 B (3,692 B gzip), mobile hero 24,670 B, no font payload.

## Handoff status

Do not release or publish this candidate until V-1 is corrected and
regression-tested. The factory can retest the exact overwrite and symlink
commands above, then recheck cache headers for V-2.
