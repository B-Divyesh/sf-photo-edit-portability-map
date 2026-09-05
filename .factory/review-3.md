# Review 3 — Lightroom metadata migration CLI

## Verdict: FAIL

**FAIL.** There are **2 findings** and **2 untested public claims**. All 12
declared claim commands pass, but the product cannot be marked PASS while a
mobile path and public claims remain incomplete.

Reviewed 2026-09-05 against
`https://photo-edit-portability-map.sociobot.in` from a clean checkout of
documentation commit `306109652407d535c9223a6140fd4cdd850093d8`. The product
implementation reviewed is
`66d40f3a2fb529dc014f2122dbe002c5e2cf26a1`; the difference between those
commits is report documentation only.

## First screen before scrolling

On fresh desktop (1440 × 900) and phone (390 × 844) browser contexts, before
scrolling, the page states:

- **Job:** “Map Lightroom metadata before you move.”
- **Audience:** photographers leaving Lightroom.
- **First action:** **Try it with sample data**; it says that it runs a safe
  sample scan in a temporary folder.

The action opened `/demo/` in one click. It showed the persistent “Demo —
sample data, nothing is saved” label, Reset demo, Start for real, and five
populated finding rows. The visible report includes embedded EXIF, sidecar
keywords, catalog-only dates and ratings, and an unsupported Lightroom develop
recipe.

## Findings

### R3-1 — Minor — The live landing page horizontally scrolls at 390 px

On a fresh iPhone 13 browser context at 390 × 844, the live home page reports
`document.documentElement.clientWidth === 390` but
`document.documentElement.scrollWidth === 399`. The overflow occurs at the
lower Pro section: `.license-copy` and `.price-sheet` run from x=23.39 to
x=398.81. This creates a 9 px horizontal scroll on the landing page.

The existing mobile regression checks `/demo/` only, so it passes and does not
cover this home-page failure. The first screen remains readable, but the lower
page fails the mobile no-overflow expectation and makes the right edge of the
Pro section moveable.

Repair by avoiding viewport-unit horizontal padding that exceeds the document
client width on the landing page (or otherwise constraining the section), then
add the same `clientWidth === scrollWidth` assertion for `/` at 390 px.

### R3-2 — Major — Two privacy/license claims are not completely registered and tested

The public copy makes two claims beyond the observable scope of their listed
tests:

1. Privacy and README say the **site** has no analytics, advertising, remote
   fonts, or third-party runtime scripts. The only related registry entry is
   `demo-no-upload`; its exact test exercises `/demo/` only. It does not
   register or test the whole-site claim.
2. Privacy says the browser stores a returned license **and its daily verdict**
   and sends the token **only to Sociobot**. `license-restore` tests token
   storage, an intercepted invalid verification response, status, and URL
   cleanup. It does not assert the daily verdict cache or record all outgoing
   requests for that license flow.

These are public privacy/storage promises a visitor could rely on. Add exact
entries to `.factory/claims.json` and clean-context browser tests: one that
records requests across every published page and one that asserts the cached
daily verdict and Sociobot-only license request. Until then, the untested claim
count is **2**.

## What passed

- Clean setup and quality gates completed: `npm ci`, `npm test`, `cargo fmt
  --check`, `cargo clippy --all-targets -- -D warnings`, `npm audit
  --audit-level=high`, `npm run build`, and `cargo package --allow-dirty`.
  The build created `dist/site/`; package output was installed into a new,
  offline consumer root. Its installed `edit-portability-map demo` command
  created a populated temporary report with text and JSON output.
- Each declared claim command passed individually: `cli-demo`,
  `read-only-scan`, `json-and-profiles`, `no-image-decoding`,
  `pro-sample-limit`, `demo-populated-output`, `demo-isolated`,
  `demo-no-upload`, `demo-profile`, `license-restore`, `checkout-starts`, and
  `offline-demo-reload`.
- Live demo isolation was independently exercised: selecting darktable wrote
  only `demo:photo-edit-portability-map:state`; Reset restored Immich; Start
  for real removed that key while preserving a seeded `real:photo-library`
  value. A live demo request log contained only the product origin.
- Full live Playwright suite: 23 passed, 1 intentional desktop-only mobile
  skip. This covered normal, invalid, boundary, and recovery CLI/browser paths,
  keyboard skip/copy/profile controls, focus, reduced motion, reset, offline
  reload, checkout redirect, and the designed 404.
- Live Axe checks on `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html`
  found 0 serious and 0 critical violations. Each had `lang=en`, one `h1`, one
  `main`, a route-specific title, and no console errors.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200. A missing
  route returns the designed page with HTTP 404, which is expected behavior.
  All discovered links resolved successfully (including the checkout redirect
  and explicit mail link).
- The live home response has a response-header CSP including
  `frame-ancestors 'none'`, HSTS, `X-Content-Type-Options: nosniff`, and
  `Referrer-Policy: strict-origin-when-cross-origin`.
- Local rebuilt HTML, JavaScript, CSS, service worker, images, manifest, and
  404 assets match the live deployment byte-for-byte. The hashed JavaScript and
  CSS responses are immutable. This confirms live behavior is implementation
  `66d40f3`, not a stale deployment.

## Earlier finding disposition

| Earlier finding set | Current disposition |
| --- | --- |
| Verification 1 `V-1`, `V-2` | The report/catalog alias and symlink protections passed their CLI regressions; hashed live JS/CSS assets are immutable. |
| Verification 2 `V2-1`–`V2-6` | Mixed-XMP, moved-file matching, malformed XMP, checkout, hero sizing, and demo touch-target regressions passed. R3-1 is a separate landing-page overflow not covered by the prior demo-only mobile test. |
| Verification 3 `V3-1`–`V3-5` | Orphan-XMP, one-to-one target matching, install command, desktop target size, and sample upper-bound regressions passed. |
| Review 1 / Review 2 `F1`–`F7` | The runnable isolated demo, first-screen copy, routes/404, metadata shell, copy audit, report status repair, and 12 declared claims are present. R3-2 finds that two additional privacy/license promises still need exact registry coverage. |

## Not applicable

This is a static/local CLI product. It has no product backend, tenant store,
shared database, health endpoint, server restart state, or product-side
rate-limiting endpoint. Backend tenant-isolation and 429/Retry-After checks do
not apply.

## Evidence

Screenshots from the first-screen inspection are in
`/work/.evidence/photo-edit-portability-map-review-3/`. The required factory
copies are `/work/.evidence/qa-report.md` and
`/work/.evidence/qa-result.json`.
