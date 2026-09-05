# Review 4 — Map Lightroom metadata before moving

## Verdict

**PASS — 0 findings; 0 untested public claims.**

- Live URL: <https://photo-edit-portability-map.sociobot.in>
- Review date: 2026-09-05 UTC
- Implementation candidate: `f13374d266494b2aa81403f1c895d411d1d51c03`
- Documentation state reviewed: `641a42592c0f882a32522b1146403313b20529cd`
- Clean checkout: `/tmp/epm-review4.EZ7koo/repo`

Only `.factory/handoff.md` and this report are changed by review 4. The two
commits after the implementation candidate change reports only. The rebuilt
candidate and the live site match across all 17 public files checked.

## Job, audience, and first action

Before scrolling in fresh 1440 × 900 desktop and 390 × 844 phone contexts:

- Job: **Map Lightroom metadata before you move.**
- Audience: photographers leaving Lightroom who need to check which ratings,
  dates, and edits stay with each photo.
- First action: **Try it with sample data.** The adjacent line says it runs a
  safe sample scan in a temporary folder.

All four lines were visible at scroll position zero on both screens. The phone
viewport and document were both exactly 390 px wide. Screenshots are
`/work/.evidence/review-4-desktop-first-screen.png` and
`/work/.evidence/review-4-phone-first-screen.png`.

## Sample and real-data isolation

One click opened `/demo/` with the persistent **Demo — sample data, nothing is
saved** label, **Reset demo**, and **Start for real**. The populated report had
five realistic rows: embedded EXIF, sidecar keywords, catalog-only corrected
dates and ratings, and an unsupported Lightroom develop recipe.

Changing the profile changed the capability result. Reset restored Immich.
Starting for real removed only `demo:photo-edit-portability-map:state`; a
seeded non-demo key stayed unchanged. Request recording over the full flow
showed only the product origin. The fresh-context offline reload also restored
the labelled five-row sample.

## Public claims

Every command in `.factory/claims.json` ran separately from the clean checkout.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `cli-demo` | Pass | Real binary created a new temporary folder with three matched photos, three catalog-only fields, and text and JSON reports. |
| `read-only-scan` | Pass | Source, target, and catalog bytes remained unchanged. |
| `json-and-profiles` | Pass | Versioned JSON completed for generic, Immich, darktable, and digiKam. |
| `no-image-decoding` | Pass | Invalid JPEG/WebP bytes still produced the inventory. |
| `pro-sample-limit` | Pass | Free 10 passed, free 11 was refused, and recorded-valid Pro 100 passed. |
| `demo-populated-output` | Pass | First-screen action opened the labelled five-row report. |
| `demo-isolated` | Pass | Reset and exit changed only the demo key. |
| `demo-no-upload` | Pass | Full demo flow made no third-party request. |
| `site-runtime-privacy` | Pass | Home, demo, legal, and 404 pages used product-origin GET requests only, with no font request. |
| `demo-profile` | Pass | Selecting darktable changed the rendered result and status. |
| `license-restore` | Pass | Recorded invalid response proved token storage, URL removal, and inactive status. |
| `license-daily-verdict` | Pass | Recorded valid response proved token-bound daily reuse and refresh only through the Sociobot verification URL. |
| `checkout-starts` | Pass | Live action opened Edit Portability Map Pro at $19.00 as a one-time purchase. No payment was entered. |
| `offline-demo-reload` | Pass | A new HTTPS context reloaded the populated demo offline after worker control. |

Result: **14 of 14 claim commands passed.** The live pages, README, legal copy,
demo documentation, CLI help, and package metadata were cross-checked against
the registry. No unlisted or untested public claim remains.

## Clean checkout and installed CLI

The documented setup and gates passed:

- `npm ci`: pass, 0 vulnerabilities.
- `npm test`: pass — 7 Rust unit tests, 20 CLI integration tests, 6 Node
  tests, and 23 applicable local browser tests; 5 live-only or viewport skips
  were expected.
- `cargo fmt --check`: pass.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `npm audit --audit-level=high`: pass, 0 vulnerabilities.
- `npm run build`: pass; produced `dist/site/` and the release binary.
- `cargo package --allow-dirty`: pass; 26 files, 31.0 KiB compressed.

The packaged crate installed offline into a new consumer root. Its installed
binary showed useful help and ran the populated demo. A normal darktable JSON
scan returned three sources, three matches, three catalog-only fields, and one
unsupported field. Invalid missing and identical folders returned exit 2 with
clear messages. Sample sizes 0 and 10 passed; 11 returned exit 2 with the Pro
recovery command. Trying to write a report over the catalog returned exit 2,
and the catalog digest stayed unchanged.

The public `cargo install --git` command also installed commit `641a425` into a
second clean consumer root. The installed `0.1.0` binary ran the same populated
demo and printed its temporary output path.

## Live site, accessibility, privacy, and routes

Fresh live Playwright ran 28 cases: **27 passed and 1 expected desktop-only
skip**. It covered both viewports, the sample, reset/isolation, keyboard use,
visible focus, reduced motion, all route shells, integrated Axe checks,
privacy requests, license state, checkout, and offline reload.

The worker URL check passed with no console errors, one `h1`, `lang=en`, one
main landmark, complete image alt text, and labelled buttons. Axe found zero
serious or critical issues on home, demo, Privacy, Terms, and 404. The keyboard
test reached and operated the skip link, copy action, and profile select. The
reduced-motion rule reduced the entrance animation to at most 0.01 ms. The
single dark treatment is explicit in `.factory/design.md`; text and control
contrast pass the automated audit.

`/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200. A deliberately
missing URL returns the designed missing-page document with HTTP 404, its own
title and `h1`, and a home action. This 404 is expected evidence, not a defect.
Every discovered product and repository link resolved; checkout correctly
returned 303 to the hosted merchant page. Route titles and canonical URLs are
distinct and correct. Robots and sitemap files resolve and list every public
route.

The root response sends CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, and
a strict referrer policy. Hashed JavaScript and CSS use one-year immutable
caching. The built JavaScript is 6.88 kB, CSS is 16.50 kB, the mobile hero is
24.67 kB, and no web font is shipped. Verification 6 recorded Lighthouse
100/100/100/100; the byte-identical live runtime means its measured candidate
has not changed.

## Candidate and live identity

SHA-256 comparison passed for all 17 public files: five HTML documents, two
hashed assets, three product images, terminal SVG, favicon, touch icon,
manifest, service worker, robots, and sitemap. `staticwebapp.config.json` is
deployment configuration and is not a public file.

## Earlier finding disposition

| Earlier finding | Current proof and disposition |
| --- | --- |
| Verification 1 `V-1`: reports could overwrite the catalog or enter a symlinked source | Closed. Both dedicated CLI regressions pass; the installed binary refused a catalog alias and preserved its digest. |
| Verification 1 `V-2`: hashed assets lacked immutable caching | Closed. Live hashed JS/CSS return `max-age=31536000, immutable`. |
| Verification 2 `V2-1`: partial XMP coverage hid catalog-only values | Closed. The mixed-coverage blocker regression passes. |
| Verification 2 `V2-2`: Pro checkout returned 404 | Closed. The live claim opens the registered $19 one-time checkout. |
| Verification 2 `V2-3`: moved unique names stayed missing | Closed. Unique and duplicate fallback regressions pass. |
| Verification 2 `V2-4`: truncated XMP lacked a warning | Closed. The malformed-XMP regression passes with the warning and no extracted field. |
| Verification 2 `V2-5`: 390 px hero clipped | Closed. Full home and demo width checks equal 390 px. |
| Verification 2 `V2-6`: mobile targets were below 44 px | Closed. Every visible phone link, button, and select meets the target test. |
| Verification 3 `V3-1`: orphan XMP hid catalog-only state | Closed. Orphan, relocated, and mixed valid/orphan regressions pass. |
| Verification 3 `V3-2`: one target satisfied two sources | Closed. Paired RAW/JPEG one-to-one regressions pass. |
| Verification 3 `V3-3`: visible install command was invalid | Closed. The HTTPS command installed successfully in a clean consumer root. |
| Verification 3 `V3-4`: desktop Pro target was too narrow | Closed. The former narrow nav item is gone; the current desktop purchase control exceeds 44 px and the phone target suite passes. |
| Verification 3 `V3-5`: sample 101 showed the wrong error | Closed. The hard-limit regression passes before license handling. |
| Review 1 / Review 2 `F1`: no runnable isolated demo | Closed. CLI and browser demos are populated, labelled, resettable, and isolated. |
| Review 1 / Review 2 `F2`: no claim registry or claim tests | Closed. The registry has 14 independently passing commands. |
| Review 1 / Review 2 `F3`: first screen did not name the job/action | Closed. Job, audience, action, and next step are visible on phone and desktop. |
| Review 1 / Review 2 `F4`: demo and 404 routing/focus were wrong | Closed. Real demo and designed 404 routes, keyboard skip focus, direct loads, and reloads pass. |
| Review 1 / Review 2 `F5`: metadata and shared shell were incomplete | Closed. Each route has the required title, metadata, landmarks, header, and footer. |
| Review 1 / Review 2 `F6`: public words were long or unclear | Closed. Current copy audit has no over-22-word or banned-word flag. |
| Review 1 / Review 2 `F7`: release status ignored failed review | Closed. Handoff and verification history clearly record each later result. |
| Review 3 `R3-1` minor: 9 px mobile overflow | Closed. Fresh 390 px phone checks show no overflow on home or demo. |
| Review 3 `R3-2`: runtime privacy and daily license claims were incomplete | Closed. Both exact claim commands pass in desktop and phone projects. |

## Scope notes

This product is a static documentation/demo site and a local Rust CLI. It has
no product backend, tenant store, server restart persistence, health endpoint,
or live request allowance, so backend-only tenant, restart, health, and
429/`Retry-After` checks do not apply. SQLite is used read-only for the local
catalog; no shared database or product service was accessed.

An AI step would not improve the defined job. The scanner needs deterministic,
local evidence about files, XMP, and catalog state. Adding model inference
would weaken privacy and accuracy, so there is no missed AI feature finding.
The crate is ready to publish but remains unpublished because registry release
belongs to the factory. No purchase, refund, deployment, or product-code change
was made during this review.
