# Review 2 — Lightroom migration inventory CLI

## Verdict: FAIL

Seven findings remain open: four blocking and three major. The product cannot
pass while any finding or untested public claim remains. There are **74
untested claim-bearing copy units** and no declared claim commands.

- Live URL: <https://photo-edit-portability-map.sociobot.in>
- Review date: 2026-09-05 UTC
- Implementation reviewed: `b0aee02872d477f380598cd604c0703cfe5ab73e`
- Documentation state reviewed: `44a95dfe7c15b0f4bb55cb574f63075b662f26c4`
- Work order base: `44a95dfe7c15b0f4bb55cb574f63075b662f26c4`

The commits after `b0aee02` change only `.factory` reports and handoff text.
A clean build from the work order base matched all 12 deployed product files
by SHA-256. The live runtime is therefore the last implementation candidate;
later report-only commits do not require a different product image.

## First screen before scrolling

Fresh Chromium contexts were opened at 1440 × 900 and 390 × 844. Both loaded
at scroll position zero with empty local storage and no console or page errors.

- Job I could infer: check a Lightroom catalog, XMP sidecars, source files, and
  a target folder to find metadata that may not move.
- Audience I could infer: photographers leaving Lightroom.
- First action shown: **Run your pre-flight**. It only changes the URL to
  `#start`; it does not run a scan or load sample data.

The headline is **“See what your edits are actually attached to.”** It does
not name the inventory job. The supporting sentence is needed to infer the
job and uses several specialist terms. There is no **Try it with sample data**
action on the first screen.

## Open findings

### F1 — Blocking — No runnable, isolated sample-data demo

The CLI has no `demo` command or `--demo` option. Both probes exit 2 as
unrecognised. The repository still has no `examples/` directory,
`.factory/demo.md`, or bundled sample catalog/XMP/source/target set.

The live `/demo?demo=1` response is the home page with seven hard-coded table
rows. It has no demo banner, **Reset demo**, **Start for real**, separate demo
storage, or terminal recording of the real binary. Its title is the home title.
The page calls the table a “Recorded local run,” but it is data defined in
`site/src/app.js`; it is not a run of the installed artifact.

The static table is populated and target selection changes its support labels.
It does not accept user data and left local storage empty. That does not prove
demo isolation because no demo operation exists. Reset and protection from
real-data changes cannot be exercised.

This is review-1 finding 1, unchanged.

### F2 — Blocking — Claims registry and claim tests are absent

`.factory/claims.json` is missing and the repository contains zero
`@claim:` tags. There are no declared claim commands to run. A missing registry
is a failed claims gate, not zero claims.

Review 1 enumerated 74 claim-bearing landing-page and README copy units. The
public copy and implementation are unchanged, so all 74 remain unlisted and
untested under the claims contract. Repeated legal-page claims are not counted
again. Examples include read-only behavior, no image-pixel reads, no telemetry,
17 categories, target support, report safety, JSON output, sample limits,
license behavior, pricing, and the claimed browser-local recorded demo.

Some behavior passes ordinary integration tests. Those tests do not have a
claim ID, do not run from the required declared sandbox, and cannot replace the
missing registry. The README claim that the site contains a browser-local
recorded demo is also inaccurate: the site contains a hard-coded table.

This is review-1 finding 2, unchanged. Untested claim count: **74**.

### F3 — Blocking — The headline and primary action do not name the job

The headline remains metaphorical and does not say that the product inventories
Lightroom metadata before a move. **Run your pre-flight** is also vague and
only scrolls to an install example. It gives no adjacent explanation of what
happens. **Read a sample map** scrolls to the static table.

The mandatory first-screen shape is still absent: a job-naming headline, a
plain audience sentence, a one-click sample action, and an explanation beside
that action.

This is review-1 finding 4, unchanged.

### F4 — Blocking — Demo routing, missing-page handling, and focus are wrong

`/demo`, `/demo?demo=1`, and an unknown path all return HTTP 200 with the home
title, home canonical URL, and home heading. There is no demo route and no
designed 404 page. A deliberate HTTP 404 would be expected; serving the home
page as a successful response is the defect.

Activating **Run your pre-flight** changes the address to `#start`, but focus
remains on `BODY`. Back returns to `/` with focus still on `BODY`. There is no
route-change announcement or destination-heading focus.

This is review-1 finding 3, unchanged.

### F5 — Major — Page metadata and the shared site structure are incomplete

The home page still has no Open Graph metadata, Twitter card, or Apple touch
icon. Privacy and Terms have no canonical, Open Graph/Twitter metadata,
manifest, header, navigation, or footer. The home header omits Privacy. The
home footer omits the required Privacy and Terms links, Param Factory credit,
and build ID. The sitemap has only home, Privacy, and Terms because no real demo
or 404 route exists.

The Privacy page tells a person to open a public repository issue for a privacy
request but does not provide a direct contact link. The legal pages are
readable and return HTTP 200, but they do not use the required shared skeleton.

This is review-1 finding 5, unchanged.

### F6 — Major — Public copy still breaks the plain-words contract

The unchanged copy still uses mood or metaphor headings, including **“A field
map, not a magic converter,” “Point at a copy. Keep the evidence,” “A larger
proof set, once,” “Privacy, in plain sight,”** and **“A map, not a guarantee.”**
It also changes terms among survey, pre-flight, field map, map, inventory, and
report. Buttons such as **Copy** and **Verify** do not name their result.

The README still contains sentences over 22 words and dense specialist terms.
The exact review-1 copy audit remains current. The required
`.factory/copy-audit.md` file is absent.

This is review-1 finding 6, unchanged.

### F7 — Major — Release status did not follow the failed review

At the start of this review, `.factory/handoff.md` still began with
**Status: PASS** from verification 4, although its final section recorded the
later review-1 **FAIL**. The live product also remained public with the failed
review findings unchanged. The handoff update in this work order now makes the
current FAIL status explicit, but it does not undo the release-state error.

## Previous finding disposition

| Earlier finding | Current disposition | Evidence |
| --- | --- | --- |
| Verification 1: report could overwrite catalog or enter a symlinked source | Fixed | All 14 CLI integration tests passed, including catalog alias and symlink regressions. An installed binary refused a report under source with exit 2 and created no file. |
| Verification 1: hashed assets lacked immutable caching | Fixed | Live JS and CSS return `public, max-age=31536000, immutable`. |
| Verification 2: partial XMP coverage hid catalog state | Fixed | `mixed_catalog_and_xmp_coverage_is_a_catalog_only_blocker` passed. |
| Verification 2: checkout returned 404 | Fixed | The live checkout endpoint returns HTTP 303 to the hosted checkout. No purchase was made. |
| Verification 2: moved unique filename stayed missing | Fixed | Unique-name and duplicate-name regression tests passed. |
| Verification 2: truncated XMP lacked a warning | Fixed | The truncated-XMP regression passed. |
| Verification 2: mobile hero clipped | Fixed | At 390 px, document width stayed 390 px and hero controls remained within the viewport. |
| Verification 2: mobile links were under 44 px | Fixed | The mobile target regression passed. |
| Verification 3: orphan XMP hid catalog state | Fixed | Orphan, relocated-orphan, and mixed valid/orphan regressions passed. |
| Verification 3: one target matched two sources | Fixed | The paired RAW/JPEG one-to-one regression passed. |
| Verification 3: visible install command was invalid | Fixed | The live visible and copied command include `https://`. |
| Verification 3: desktop Pro target was too narrow | Fixed | It measured 44 × 44 CSS px; the desktop regression passed. |
| Verification 3: sample size 101 showed the wrong error | Fixed | Installed CLI returned exit 2 with “sample size cannot exceed 100.” |
| Review 1: runnable demo | Open | F1. |
| Review 1: claims registry | Open | F2. |
| Review 1: demo/404 routing | Open | F4. |
| Review 1: first-screen copy and action | Open | F3. |
| Review 1: metadata and shared skeleton | Open | F5. |
| Review 1: plain words | Open | F6. |

## Clean checkout and artifact verification

The independent checkout started clean at
`44a95dfe7c15b0f4bb55cb574f63075b662f26c4`.

- `npm ci`: passed; 21 packages installed and 0 audit vulnerabilities.
- `npm test`: passed; 7 Rust unit tests, 14 CLI integration tests, 4 Node
  contract tests, and 16 local Playwright tests passed with 4 expected
  environment/viewport skips.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed.
- `npm run build`: passed and produced `dist/site/` and the release binary.
- `cargo package --allow-dirty`: passed; the crate verified successfully.

The package was installed offline into a new consumer root. The installed
binary reported version 0.1.0 and useful `--help`. A populated source/target
scan returned one matched item, wrote human and JSON reports, and preserved
both input hashes. The JSON used schema 1.0 and included a deterministic sample.

Installed-artifact path checks:

| Path | Result |
| --- | --- |
| Normal matched scan | Exit 0; reports written outside inputs; input hashes unchanged. |
| Missing source | Exit 2 with the missing path. |
| Report inside source | Exit 2; no output created. |
| Sample size 0 | Exit 0 with no sample. |
| Free sample size 11 | Exit 2 with license recovery instructions. |
| Sample size 101 | Exit 2 with the hard maximum. |
| `--demo` and `demo` | Both exit 2 because neither exists. |

## Live browser, accessibility, privacy, and offline checks

- Factory `verify-url.sh`: passed; HTTP 200, title, `lang=en`, one `h1`, one
  `main`, image alt text, labeled buttons, and no console errors.
- Live Playwright suite: 18 passed and 2 expected viewport skips across fresh
  desktop and phone contexts.
- Axe CLI 4.10.3: zero violations on home, Privacy, Terms, and `/demo`.
- Keyboard: skip link, navigation, copy actions, profile control, purchase,
  restore field, and submit button were reachable with visible 3 px cyan focus.
  No trap was found. The hash-route focus defect is recorded in F4.
- Reduced motion: the terrain animation duration was `0.00001s`; scroll
  behavior was `auto`.
- Normal load contacted only the product origin. Submitting an invalid license
  made one allowed request to the Sociobot verification endpoint, stored only
  the token and verdict cache, showed a recovery message, and left free tools
  available.
- Service-worker update and offline home reload passed in both live contexts.
  The offline label says “local demo ready,” but there is no runnable demo.
- Home, Privacy, Terms, repository, robots, and sitemap links returned 200.
  Checkout returned 303. Invalid license verification returned 200 with
  `valid:false`, product-origin CORS, and `Cache-Control: no-store`.
- HTTP redirects to HTTPS. Live responses include CSP, HSTS, `nosniff`, and a
  strict referrer policy.
- Initial JS is 6,368 bytes, CSS is 13,694 bytes, and the mobile hero is
  24,670 bytes. There are no web-font files.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; FCP 895 ms, LCP 1,213 ms, TBT 24 ms, CLS 0.

This is a static site plus a local CLI. It has no product backend or tenant
state, so tenant isolation, restart persistence, health, and product 429 rules
do not apply. The external billing API was checked only through normal checkout
and invalid-license requests; it was not load-tested.

## Claim command result

No claim command exists to run because `.factory/claims.json` is missing. The
claim gate is **FAIL** with 74 untested public claim units.

## Final count

- Blocking findings: 4
- Major findings: 3
- Minor findings: 0
- Total findings: **7**
- Untested claims: **74**
- Final verdict: **FAIL**
