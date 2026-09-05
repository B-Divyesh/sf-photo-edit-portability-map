# Review 3 handoff — Edit Portability Map

## Status

**FAIL.** Review 3 found 2 findings and 2 untested public claims. No product
code was changed.

The implementation reviewed is
`66d40f3a2fb529dc014f2122dbe002c5e2cf26a1`. Documentation before this review
was `306109652407d535c9223a6140fd4cdd850093d8`; it differs from the
implementation only in `.factory` reports.

## What was verified

- A clean checkout passed install, tests, format, clippy, audit, build, and
  package checks. The packaged crate installed offline in a new consumer root;
  `edit-portability-map demo` produced populated text and JSON reports in a
  temporary folder.
- All 12 commands in `.factory/claims.json` passed individually.
- Fresh live desktop and phone checks confirmed the job, audience, and
  **Try it with sample data** first action. The one-click demo is populated,
  visibly labelled, resettable, isolated from real browser data, and removed
  by Start for real.
- Full live Playwright checks passed 23 tests with one expected skip. Live Axe
  checks found 0 serious/critical findings on every route. The live deployment
  byte-matches the reviewed implementation.

## Remaining work

1. Fix the 9 px horizontal overflow on the live home page at a 390 px phone
   viewport and add a home-page mobile-width regression.
2. Register and test the two public privacy/license promises described in
   `.factory/review-3.md`: whole-site no-third-party-runtime behavior, and
   daily license-verdict/Sociobot-only request behavior.

Do not declare the product PASS until those repairs and their exact claim
commands pass in a clean checkout and on the live deployment.

## Reproduce

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
cargo package --allow-dirty
```

Run every command in `.factory/claims.json`, then install the packaged crate
into a fresh consumer root and run `edit-portability-map demo`. See
`.factory/review-3.md` for full evidence and finding disposition.
