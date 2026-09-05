# Independent verification 5 — PASS

**Work order:** `photo-edit-portability-map-verify-5`  
**Implementation reviewed:** `66d40f3a2fb529dc014f2122dbe002c5e2cf26a1`  
**Documentation reviewed:** `fbcad4f6fb18fb99a8894e04b5201ace73e45a8b`  
**Live URL:** <https://photo-edit-portability-map.sociobot.in>  
**Verified:** 2026-09-05 UTC

## Verdict

**PASS.** There are **zero findings** at every severity and **zero untested
public claims**. The live deployment is the implementation reviewed; the only
commit after that implementation is the separate handoff update.

The job is to map Lightroom metadata before moving photos. It is for
photographers leaving Lightroom or checking an external library. The first
action is **Try it with sample data**; it opens a populated, labelled demo.

## Clean setup and artifact checks

A new clone of the requested branch was used at
`/tmp/photo-edit-portability-map-verify-5-McWaNT`.

- `npm ci` passed with 0 high-or-higher audit findings.
- `npm test` passed: 7 Rust unit tests, 19 Rust CLI integration tests, 6 site
  contract tests, and 19 applicable local Playwright tests (5 expected
  environment/viewport skips).
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `npm audit --audit-level=high`, `npm run build`, and
  `cargo package --allow-dirty` all passed.
- `npm run build` produced `dist/site/`. Built JavaScript is 6.88 kB
  (2.78 kB gzip) and CSS is 16.44 kB (4.21 kB gzip), below the static-product
  budgets.
- The release binary SHA-256 is
  `f54af446fe842f5ed42d21de9916d2c19b4b0d9c7d3ffc563e322cee3e78d5f4`.
  The packaged crate SHA-256 is
  `5e35fafed3afd771482397df488f82ffbb760dc0e8e176428fefcc5622fc202b`.
- A fresh offline consumer install from `target/package/` succeeded. The
  installed `edit-portability-map --help` is useful and its `demo` command
  created an isolated temporary folder with populated text and JSON reports.

The latest valid Lighthouse result remains the one recorded for this exact
implementation in the prior handoff: 99 Performance, 100 Accessibility, 100
Best Practices, and 100 SEO. A fresh Lighthouse CLI attempt in this worker
could not connect to its Chrome launcher and produced no report; that is a
worker-tool limitation, not a product result. Fresh Playwright and Axe checks
below completed normally.

## Claims

Every command declared in `.factory/claims.json` was run from the clean setup
and passed. There are 12 declared claims and 12 tested claims.

| Claim | Result |
| --- | --- |
| `cli-demo` | Passed: a new temporary directory contains populated demo reports. |
| `read-only-scan` | Passed: source, target, and catalog input bytes are unchanged. |
| `json-and-profiles` | Passed for generic, Immich, darktable, and digiKam. |
| `no-image-decoding` | Passed with opaque invalid image bytes. |
| `pro-sample-limit` | Passed at free size 10 and Pro-required size 11. |
| `demo-populated-output` | Passed: one click opens five realistic rows and the sample banner. |
| `demo-isolated` | Passed: reset and exit only change the `demo:` key. |
| `demo-no-upload` | Passed: the full demo flow uses only the product origin. |
| `demo-profile` | Passed: changing profile changes the shown capability result. |
| `license-restore` | Passed against a recorded invalid response; storage and URL cleanup work. |
| `checkout-starts` | Passed live: the Pro action reaches the registered Sociobot checkout path. |
| `offline-demo-reload` | Passed live in a new browser context after worker installation. |

The landing page, demo, privacy page, terms, and README were cross-checked
against the registry. Their relied-on read-only, local demo, profile, sample
limit, checkout, and privacy statements are covered by those observable
tests; no unlisted public claim was found.

## CLI paths

The packaged consumer command produced a report for three source photos,
three matched targets, and three catalog-only fields: corrected capture date,
star rating, and Lightroom develop recipe. It identifies the develop recipe
as unsupported rather than pretending to convert it.

The clean test suite also exercises normal, invalid, boundary, and recovery
paths: empty inputs, bad paths, report paths that alias a catalog or enter a
source tree through a symlink, malformed and orphan XMP, one-to-one target
matching, sample bounds, blockers, profile JSON, and local license recovery.
The product has no backend, tenant data, health endpoint, or server rate-limit
path, so backend-only tenant/restart/429 checks do not apply.

## Live website checks

Fresh 1440 x 900 desktop and 390 x 844 phone contexts both showed, before
scrolling, the title “Map Lightroom metadata before you move,” the Lightroom
audience sentence, and the **Try it with sample data** action with its
temporary-folder explanation. Screenshots are in
`/work/.evidence/photo-edit-portability-map-verify-5/`.

Following that action on a fresh phone context opened `/demo/` with title
“Demo — Edit Portability Map,” the persistent “Demo — sample data, nothing is
saved” label, Reset demo, and Start for real. The populated five-row report
showed embedded EXIF, sidecar keywords, catalog-only date and rating, and an
unsupported develop recipe. Reset restored Immich; Start for real removed the
demo storage key and returned home. No real browser key was changed.

- Full live Playwright suite: 23 passed, 1 expected viewport-specific skip.
  It covers desktop and phone, keyboard skip/copy/profile controls, focus,
  touch target dimensions, reduced motion, accessibility, route titles,
  checkout redirect, demo isolation, privacy requests, and offline reload.
- Axe integration on `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html`:
  0 serious and 0 critical violations. `verify-url.sh` passed for the home
  page: title, `lang`, h1, main landmark, image alt text, labelled buttons,
  and no console/page errors.
- Fresh requests on home, demo, privacy, and terms stayed on the product
  origin. There are no analytics, CDN fonts, or third-party runtime scripts.
  Sociobot is contacted only after an explicit license action.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200. An
  intentionally absent URL returns HTTP 404 and the designed not-found page;
  this is expected behavior, not a defect.
- Live home response headers include response-header CSP with
  `frame-ancestors 'none'`, HSTS, `X-Content-Type-Options: nosniff`, and
  `Referrer-Policy: strict-origin-when-cross-origin`.

## Candidate and live identity

`git diff --name-only 66d40f3..fbcad4f` contains only `.factory/handoff.md`.
The rebuilt files and live files have identical SHA-256 digests for home,
demo, privacy, terms, 404, service worker, manifest, favicon, both landscape
images, social image, JavaScript, and CSS. This confirms that live behavior is
from implementation `66d40f3`, not a stale deployment.

## Earlier finding disposition

| Earlier finding set | Current disposition |
| --- | --- |
| Verification 1 `V-1` unsafe report overwrite and `V-2` asset caching | Covered by alias/symlink refusal tests; live hashed assets remain immutable. |
| Verification 2 `V2-1` through `V2-6` | Covered by mixed-XMP, matching, malformed-XMP, checkout, and mobile regression tests; all passed. |
| Verification 3 `V3-1` through `V3-5` | Covered by orphan-XMP, one-to-one target, install command, touch-target, and sample-bound regressions; all passed. |
| Review 2 `F1` through `F7` | The runnable isolated demo, 12-claim registry, plain first screen, routes/404, shared metadata shell, copy audit, and corrected status are present and verified above. |

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

Run each command listed in `.factory/claims.json` for claim-level evidence.
For the consumer artifact check:

```sh
cargo install --path target/package/edit-portability-map-0.1.0 --root /tmp/epm-consumer --offline --locked
/tmp/epm-consumer/bin/edit-portability-map demo
```
