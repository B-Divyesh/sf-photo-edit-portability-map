# Handoff — review 2

## Status: FAIL

The independent review on 2026-09-05 found seven open findings and 74 untested
public claim units. Do not mark this product released or accepted. Full evidence
is in `.factory/review-2.md`.

Implementation reviewed:
`b0aee02872d477f380598cd604c0703cfe5ab73e`. Documentation state reviewed:
`44a95dfe7c15b0f4bb55cb574f63075b662f26c4`. All deployed product files match
the clean build of that unchanged implementation.

## What this review did

- Opened the live site in fresh desktop and 390 px phone browsers.
- Checked the first screen, static sample, `/demo`, legal pages, unknown routes,
  links, keyboard focus, reduced motion, accessibility, privacy requests,
  service-worker update, and offline reload.
- Ran every project quality gate from a clean checkout.
- Installed the packaged CLI into a clean consumer root and exercised normal,
  invalid, boundary, and recovery paths.
- Rechecked every earlier verification and review finding.
- Changed only review and handoff documentation; product code was not modified.

## Verification summary

Passed: `npm ci`, `npm test`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `npm audit --audit-level=high`,
`npm run build`, `cargo package --allow-dirty`, live Playwright, factory
`verify-url.sh`, Axe CLI, installed-artifact scans, live-file SHA-256 parity,
and Lighthouse.

Failed acceptance gates:

- No runnable isolated CLI sample demo, sample fixtures, demo documentation,
  persistent demo label, reset, or start-for-real path.
- No `.factory/claims.json`, no `@claim:` tests, and 74 untested public claim
  units.
- The first-screen headline and action do not name the job or start a sample.
- `/demo` and unknown paths return the home page; hash navigation does not move
  focus.
- Metadata, shared legal-page structure, and plain copy remain incomplete.
- The prior handoff retained a top-level PASS after review 1 failed.

## How to verify

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

For the installed-artifact check, install
`target/package/edit-portability-map-0.1.0` into a new Cargo root with
`--offline --locked`. The CLI currently has no demo command to verify.

## Next steps

Implement every open item in `.factory/review-2.md`, add exact claim tests and
the required sample sandbox, then run a new independent review. A passing test
suite alone does not change this FAIL verdict.
