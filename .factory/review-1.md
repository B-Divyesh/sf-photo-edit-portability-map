# Adversarial first-read review 1

**Verdict: FAIL.** Four blocking findings prevent a first-time visitor from trying or verifying this CLI safely.

Reviewed 2026-08-28 against `https://photo-edit-portability-map.sociobot.in/` in fresh Chromium contexts at 390 × 844 and 1440 × 900, plus a clean local clone of commit `8afec01`.

## Cold first screen

My initial reading on both screen sizes: this appears to be a Lightroom migration utility that tells photographers where edits and metadata live. It is apparently for someone leaving Lightroom. I cannot state what to click first to **try the product**: the prominent action, **“Run your pre-flight,”** only changes the URL to `#start`; it does not run a scan, show sample data, or say what will happen. The other option, **“Read a sample map,”** only reaches a static page section.

This fails the required first-screen answer for the first action. The headline, **“See what your edits are actually attached to.”**, is a metaphor rather than the job; without the 19-word supporting sentence it does not say that this is a CLI inventory, which files are inspected, or what the result is.

## Findings, ordered by severity

### BLOCKING — No runnable, isolated sample-data demo

- **Quote:** “Run your pre-flight”; “Read a sample map”; “Recorded local run / alpine archive.”
- **Check:** Neither hero action is “Try it with sample data.” The latter only scrolls to an HTML table. `https://photo-edit-portability-map.sociobot.in/demo?demo=1` returns the landing page (HTTP 200) with no demo banner, no **Reset demo**, and no **Start for real** control. It creates no demo storage namespace. The repository has no `examples/`, `.factory/demo.md`, `--demo`, or `demo` command. In a temporary directory, both `edit-portability-map --demo` and `edit-portability-map demo` exit 2 as unrecognised.
- **Why this loses or misleads a visitor:** A static, unlabelled report can look like a result from the visitor's files, yet it cannot establish that the binary performs the shown work. There is no safe path to see the real CLI run before installation or to reset sample state.
- **Concrete fix:** Ship realistic source, target, XMP, and catalog fixtures in `examples/`; implement `edit-portability-map demo` (or `--demo`) to copy them to a new temporary directory, run the real scan, print the output directory, and leave user paths untouched. Add a first-screen **“Try it with sample data”** link to `/demo` and a self-hosted terminal recording of that exact command. `/demo` must show **“Demo — sample data, nothing is saved”**, **Reset demo**, and **Start for real**. Document the command, fixture set, reset behaviour, and isolation in `.factory/demo.md`; test it in a new temp directory.

### BLOCKING — The required claims registry and claim tests are absent

- **Quote:** “No image pixels read”; “No catalog writes”; “No telemetry”; “17 metadata categories”; “The complete inventory, safety checklist, and JSON export stay free”; “$19 one time.” README also makes claims including “It never changes the source tree or catalog,” “The documented examples are covered by integration tests,” and “The site contains a browser-local recorded demo; it does not upload files.”
- **Check:** `.factory/claims.json` does not exist, so there are zero claim entries, zero `@claim:` tests, and no listed test commands to run from the clean clone. Every claim-like live-page and README statement indexed with `†` in the audit below is therefore an unlisted claim. The browser's existing `normal use stays on the product origin` test is not a demo-flow privacy test and cannot substitute for a claim entry.
- **Why this loses or misleads a visitor:** Privacy, read-only behaviour, support counts, pricing entitlement, and demo isolation are decisions a migrating photographer may rely on. They cannot be independently verified in the stipulated sandbox.
- **Concrete fix:** Add `.factory/claims.json` with one entry and one clean-sandbox observable test per claim. At minimum cover no source/catalog writes, no image-pixel decoding, no telemetry/network outside explicitly permitted endpoints, JSON/report output, the supported profile set, free 10-file and Pro 100-file limits, price/checkout response if advertised, and offline demo reload if “Offline” is retained. Remove any sentence that cannot be tested. Tag the tests `@claim:<id>` and list the exact command and sandbox in the registry.

### BLOCKING — A bad or demo URL is served as the landing page, not its own state or 404

- **Quote:** `/demo` and `/does-not-exist` both respond `200` with title “Edit Portability Map — Lightroom migration pre-flight CLI” and the landing `<h1>` “See what your edits are actually attached to.”
- **Check:** `/demo` is not a demo route and `/does-not-exist` is not a designed 404. The fallback disguises a missing route as a real page. The hash action also leaves focus on `BODY`, not a destination heading; Back restores `BODY` focus.
- **Why this loses or misleads a visitor:** A catalog link to the promised demo opens an unrelated landing state, and a mistyped URL cannot be identified as missing. Keyboard and screen-reader users receive no route-change focus or announcement.
- **Concrete fix:** Add a real `/demo` document/state with title **“Demo — Edit Portability Map”**, its own plain `<h1>`, canonical URL, and focus/live announcement on navigation. Add a styled 404 with a return-home link and HTTP 404 status. Test direct load, reload, Back/Forward, heading focus, and the 404 response.

### BLOCKING — The primary copy does not name the job or an outcome-naming first action

- **Quote:** “See what your edits are actually attached to.” and “Run your pre-flight.”
- **Check:** The headline is eight words but does not say “inventory,” “migration,” “metadata,” or “CLI.” The action is neither a result-naming verb nor an explanation of the next screen; it is only an anchor. The first descriptive sentence is 19 words, but its jargon (“XMP,” “develop state,” “catalog”) makes it do too much work.
- **Why this loses or misleads a visitor:** A visitor can infer a broad purpose but not whether it scans a catalog, converts edits, creates a backup, or uploads photos. The required first click remains ambiguous.
- **Concrete fix:** Use: **“Map Lightroom metadata before you move.”** Supporting sentence: **“For photographers leaving Lightroom, see which ratings, keywords, and edits stay with each photo.”** Put **“Try it with sample data”** first, with **“Runs a safe sample scan in a temporary folder”** alongside it. Make the real secondary action **“Copy install command.”**

### MAJOR — Metadata and page skeleton are incomplete

- **Quote/check:** The home page has a valid title, description, canonical link, favicon, one `<h1>`, `lang`, `main`, robots and sitemap. It has no Open Graph or Twitter metadata and no 180 px Apple touch icon. Privacy and Terms have no canonical, Open Graph/Twitter metadata, manifest, header/navigation, or footer. The home header omits Privacy; its footer omits Privacy/Terms from the global footer pattern, “Built by Param Factory,” and a version/build identifier.
- **Why this loses or misleads a visitor:** Shared/previewed links have no controlled product image or description, and legal pages look detached from the product and give less consistent navigation.
- **Concrete fix:** Provide per-route canonical, OG, Twitter and description metadata; make and reference a 1200 × 630 product-derived image and 180 px touch icon. Render the same header, skip link, and footer on legal/demo/404 routes, including visible Privacy, Terms, Param Factory attribution, and build identifier.

### MAJOR — Copy audit identifies long, jargon-heavy, and contextless copy

- **Quote:** “It inventories file names, XMP sidecars, and a Lightroom SQLite catalog; classifies visible state as embedded, sidecar, catalog-only, or unsupported by the selected destination; and produces a migration checklist plus a deterministic verification sample.” (34 words)
- **Why this loses or misleads a visitor:** This asks a cold reader to learn several technical labels before understanding the output.
- **Concrete fix:** “It checks your catalog and XMP sidecars. It lists what travels with a photo and what stays in Lightroom.”

Other required rewrites are recorded in the audit flags below: replace contextless headings (“A larger proof set, once.”), marketing/adjectival wording (“honest checklist,” “useful free inventory”), terms that change level of abstraction (“pre-flight,” “survey,” “field map,” “map,” “inventory,” “report”), and buttons that do not name a result (“Copy,” “Verify,” “Run your pre-flight,” “Read a sample map”).

## Demo and privacy sandbox result

No demo could be entered because no demo exists. A fresh page at `/demo?demo=1` contained zero matching demo banners, zero Reset controls, and zero Start-for-real controls. It had empty local storage before interaction, but that is not evidence of demo isolation because no demo operation occurred. Network interception on this route showed no console errors or failed responses, but cannot exercise the advertised CLI privacy/read-only claims. The existing site uses an external license verification endpoint when a token is submitted, so an eventual privacy test must assert the allowed request list over the whole demo flow.

## Structure and link checks

| Check | Result |
| --- | --- |
| Fresh mobile/desktop load | HTTP 200; no console errors observed; 390 px document width remained 390 px. |
| `/`, `/privacy/`, `/terms/` | HTTP 200; titles present; each page has one `<h1>` and `main`. |
| Internal/external destinations | Privacy, Terms, GitHub, and checkout destinations resolved HTTP 200 (checkout redirects to Dodo). Hash targets are present. |
| `/demo`, unknown route | Incorrect: both return the landing page HTTP 200; no designed 404. |
| Visual identity | Distinct luminous-glass/inspection imagery and dark field-sheet treatment are present; this is not a generic gradient SaaS surface. |

## Copy audit

Word counts treat command lines, headings, controls, and visible generated report text as copy units. `!` marks a plain-words issue. `†` marks a claim-like statement with no claims registry entry (all such claims are unlisted because the registry is absent).

### Landing page

| ID | Words | Copy |
| --- | ---: | --- |
| L01 | 4 | Skip to main content |
| L02 | 4 | Edit Portability Map |
| L03 | 3 | How it works |
| L04 | 2 | Sample report |
| L05 | 1 | Pro |
| L06 | 1 | Source |
| L07 | 6 | Read-only migration survey v0.1.0 ! inconsistent level/term |
| L08 | 8 | See what your edits are actually attached to. ! metaphor headline |
| L09 | 19 | Before leaving Lightroom, map ratings, dates, keywords, virtual copies, and develop state across the file, XMP, catalog, and destination. ! jargon |
| L10 | 8 | cargo install --git https://github.com/B-Divyesh/sf-photo-edit-portability-map |
| L11 | 1 | Copy ! button does not name result |
| L12 | 3 | Run your pre-flight ! anchor, not result-naming |
| L13 | 4 | Read a sample map ! does not say static/demo |
| L14 | 9 | No image pixels read; No catalog writes; No telemetry † |
| L15 | 3 | 4 storage states † |
| L16 | 3 | 17 metadata categories † |
| L17 | 3 | 0 source writes † |
| L18 | 3 | 1 honest checklist ! marketing adjective; † |
| L19 | 4 | Survey method / 01–03 ! “survey” inconsistent |
| L20 | 7 | A field map, not a magic converter. ! contextless/metaphor |
| L21 | 5 | The tool makes ownership visible. † |
| L22 | 13 | It never promises that one application's look can be translated into another's recipe. † |
| L23 | 10 | Open a Lightroom catalog in SQLite read-only mode. ! jargon; † |
| L24 | 5 | Parse adjacent XMP as text. ! jargon; † |
| L25 | 8 | List source and target names without decoding photos. † |
| L26 | 19 | Classify each observed category by its real home and compare it with Immich, darktable, digiKam, or a generic file workflow. ! jargon; † |
| L27 | 10 | Work blockers first, then import a deterministic cross-section. ! jargon; † |
| L28 | 8 | Keep every original until the destination passes inspection. |
| L29 | 5 | Recorded local run / alpine archive ! unsubstantiated demo label |
| L30 | 7 | Read the risk before moving a file. ! metaphor/contextless |
| L31 | 2 | Target profile |
| L32 | 3 | Source 18,420 † |
| L33 | 3 | Matched 18,412 † |
| L34 | 2 | Missing 8 † |
| L35 | 2 | Catalog-only 4 † |
| L36 | 7 | Sample portability inventory for the selected target ! “portability inventory” jargon |
| L37 | 1 | State |
| L38 | 1 | Field |
| L39 | 1 | Records |
| L40 | 2 | Target result |
| L41 | 3 | Before you move |
| L42 | 4 | Run it / local terminal ! contextless |
| L43 | 4 | Point at a copy. ! unclear verb |
| L44 | 3 | Keep the evidence. ! unclear evidence |
| L45 | 7 | A catalog is optional for XMP-only workflows. ! jargon; † |
| L46 | 8 | Reports must live outside the folders being scanned. † |
| L47 | 2 | Copy command |
| L48 | 9 | Lightroom's develop recipe and history are proprietary application state. ! jargon; † |
| L49 | 10 | This tool flags them; it does not fake a conversion. † |
| L50 | 11 | Render critical finished versions, preserve originals, and keep a catalog backup. |
| L51 | 3 | Optional Pro license |
| L52 | 5 | A larger proof set, once. ! contextless |
| L53 | 10 | The complete inventory, safety checklist, and JSON export stay free. † |
| L54 | 16 | Pro expands the verification evidence pack from 10 to 100 files for larger, more varied libraries. † |
| L55 | 4 | One-time purchase; no subscription † |
| L56 | 6 | License works across your own devices † |
| L57 | 7 | Refunds and receipts handled by Sociobot/Dodo † |
| L58 | 3 | $19 one time † |
| L59 | 4 | Buy Pro through Sociobot |
| L60 | 6 | Checkout opens on the merchant-of-record site. † |
| L61 | 3 | Already purchased? |
| L62 | 3 | Paste your license |
| L63 | 1 | Verify ! button does not name result |
| L64 | 3 | Free edition active. † |
| L65 | 4 | Copy CLI activation command |
| L66 | 2 | Privacy; Terms |
| L67 | 8 | A small instrument for a consequential move. ! metaphor |
| L68 | 6 | MIT licensed; No telemetry; View source † |
| L69 | 8 | Camera and lens EXIF; 18,420; Supported † |
| L70 | 15 | Confirm camera, lens, ISO, and orientation on three files. † |
| L71 | 12 | Corrected capture date; 614; Supported; Check a corrected video date after the destination rescan. † |
| L72 | 14 | Star rating; 7,382; Partial — verify; Confirm 0, 3, and 5-star examples. † |
| L73 | 19 | Keywords and hierarchy; 12,110; Partial — verify; Check one nested keyword branch; hierarchy may flatten. † |
| L74 | 14 | Collections; 86; Unsupported; Export collection membership or recreate it as tags. † |
| L75 | 14 | Virtual copies; 223; Unsupported; Render or duplicate every version you intend to keep. † |
| L76 | 17 | Lightroom develop recipe; 15,906; Unsupported; Render critical finals; preserve RAW files and the catalog. † |
| L77 | 13 | Showing the conservative Immich capability profile. Your installed version is the final authority. † |

### README

| ID | Words | Copy |
| --- | ---: | --- |
| R01 | 3 | Edit Portability Map |
| R02 | 18 | Edit Portability Map is a read-only pre-flight CLI for photographers leaving Lightroom or auditing an external photo library. ! “pre-flight” |
| R03 | 34 | It inventories file names, XMP sidecars, and a Lightroom SQLite catalog; classifies visible state as embedded, sidecar, catalog-only, or unsupported by the selected destination; and produces a migration checklist plus a deterministic verification sample. ! >22; jargon; † |
| R04 | 16 | It never changes the source tree or catalog, never decodes image pixels, and has no telemetry. † |
| R05 | 20 | It also does not claim to translate proprietary Lightroom development recipes: those must be rendered or rebuilt in the destination. † |
| R06 | 11 | Build the single binary with stable Rust 1.85 or newer: † |
| R07 | 9 | Release artifacts can be built with cargo build --release. † |
| R08 | 6 | The binary is target/release/edit-portability-map. |
| R09 | 14 | Run a useful free inventory against a Lightroom catalog, source library, and destination folder: ! “useful” |
| R10 | 8 | Write stable JSON for scripts or later comparison: † |
| R11 | 10 | Or save both human and machine reports without shell redirection: |
| R12 | 12 | --catalog is optional for XMP-only workflows. --source and --target are always required. † |
| R13 | 9 | Supported target profiles are generic, immich, darktable, and digikam. † |
| R14 | 18 | Use --sample-size 0 to omit the verification sample, or a value up to 10 in the free edition. † |
| R15 | 24 | Exit code 0 means the scan completed, 2 means an input/usage problem, and 3 means the inventory found migration blockers (only with --fail-on-blockers). ! >22; † |
| R16 | 16 | Saved reports must resolve outside the source and target trees and cannot alias the input catalog. † |
| R17 | 19 | The CLI resolves symlinked paths and existing parent directories before scanning, then refuses unsafe output with exit code 2. † |
| R18 | 8 | The JSON shape is versioned with schema_version. † |
| R19 | 8 | The documented examples are covered by integration tests. † |
| R20 | 7 | Embedded — normally travels inside the photo container. † |
| R21 | 13 | The CLI reports this as an expected location without opening binary image data. † |
| R22 | 11 | Sidecar — observed in an .xmp file next to a source asset. † |
| R23 | 22 | Adjacency is matched by case-normalized relative path and stem; relocated or orphan XMP files are warned about and cannot mask catalog-only state. ! jargon; † |
| R24 | 10 | Catalog-only — observed in Lightroom tables without equivalent sampled sidecar coverage. ! jargon; † |
| R25 | 26 | If a field appears in more populated catalog rows than XMP sidecars, the uncovered difference is conservatively reported here; export, render, or recreate it before leaving. ! >22; jargon; † |
| R26 | 10 | Target-unsupported — the selected profile cannot faithfully consume the source representation. † |
| R27 | 9 | It is a capability warning, not a conversion error. † |
| R28 | 6 | The generated checklist is intentionally conservative. † |
| R29 | 12 | Verify the sample in the actual destination before deleting or cancelling anything. |
| R30 | 19 | Target matching is one-to-one: exact relative paths are reserved first, then only unambiguous relative-stem or unique-name fallbacks are used. † |
| R31 | 11 | The free scan, safety warnings, and JSON export are never gated. † |
| R32 | 13 | A one-time $19 Pro license unlocks verification evidence packs up to 100 files. † |
| R33 | 15 | Purchase and restore are handled by Sociobot, the merchant of record, on the project site. † |
| R34 | 11 | Activate the returned token once, or supply it through the environment: |
| R35 | 14 | The verdict is cached in the user config directory and checked at most daily. † |
| R36 | 11 | A previously valid license remains optimistic during a temporary network failure. † |
| R37 | 8 | The token is never written into a report. † |
| R38 | 17 | npm run build runs the Rust release build and the Vite documentation site; the static deployment output is exactly dist/site/. ! two ideas; † |
| R39 | 4 | npm run build:site builds only the site. † |
| R40 | 12 | The site contains a browser-local recorded demo; it does not upload files. ! false/unsupported demo description; † |
| R41 | 8 | There is no analytics or runtime third-party script. † |
| R42 | 16 | The website stores only a license token and daily verification cache when a buyer opts in. † |
| R43 | 24 | The CLI reads SQLite with read-only/query-only flags and reads XMP text; it does not access cloud credentials, image pixels, or write source metadata. ! >22; jargon; † |
| R44 | 15 | A copied catalog is still recommended because Lightroom may itself be using a live catalog. |
| R45 | 17 | See the hosted privacy and terms pages. |
| R46 | 1 | MIT. |

The command blocks, option spellings, section labels, and headings are also visible README copy; they were checked as controls/technical references rather than counted as prose sentences. No prohibited marketing word from the supplied banned list was found, but “honest,” “useful,” and “conservative” make untested quality assertions.

## Verification record

- Fresh live browser contexts: complete for mobile and desktop; screenshots reviewed; no console errors observed.
- Fresh local clone: `npm ci`, `npm test`, and `npm run build` completed successfully. The Playwright `test-results/.last-run.json` records `passed`; the subsequent release/LTO build produced `dist/site/` before the CLI demo probes ran.
- Claims: no commands run because `.factory/claims.json` is absent; this is the blocking result, not a pass.
- CLI demo probe: `edit-portability-map --demo` → exit 2, “unexpected argument '--demo'”; `edit-portability-map demo` → exit 2, “unrecognized subcommand 'demo'”.
