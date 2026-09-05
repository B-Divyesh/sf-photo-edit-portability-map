# Repair 4 handoff — Edit Portability Map

## Status

PASS for the seven findings in Review 2. The deployed implementation is
66d40f3a2fb529dc014f2122dbe002c5e2cf26a1 (fix: restore verified Pro
checkout path). This handoff is committed separately after the implementation;
the final delivery records that documentation SHA.

## What the product does

- **Job:** map Lightroom metadata before moving photos.
- **Audience:** photographers leaving Lightroom or checking a read-only
  external library.
- **First action:** **Try it with sample data**. It opens /demo/, where a
  bundled sample scan is already populated and labelled as saved nowhere.

The CLI remains read-only. It inventories a Lightroom catalog, XMP sidecars,
source files, and target folder; it does not translate proprietary RAW develop
recipes or read image pixels.

## Review 2 remediation

| Finding | Disposition and evidence |
| --- | --- |
| F1 runnable isolated demo | Fixed. edit-portability-map demo copies bundled fixtures to a new OS temporary directory, creates the catalog, runs the real scanner, and writes both reports. /demo/ has populated sample output, a persistent “Demo — sample data, nothing is saved” banner, reset, start-for-real action, terminal recording, and the demo:photo-edit-portability-map:state storage namespace. .factory/demo.md documents it. |
| F2 claims registry | Fixed. .factory/claims.json contains 12 public claims. Each names exactly one outcome-based @claim:<id> command. All commands passed from a clean checkout and current workspace. Copy was reduced to claims covered by that registry. |
| F3 first screen | Fixed. The landing heading is “Map Lightroom metadata before you move.” It identifies the audience, names the one-click sample action, and explains the populated result beside it. |
| F4 routing, 404, focus | Fixed. /demo/, /privacy/, /terms/, and /404.html are separate static pages with route titles. Unknown paths return the designed 404.html with HTTP 404. No navigation fallback serves home as a successful missing page. Navigation puts focus on the destination heading and uses a polite route announcement. |
| F5 metadata and skeleton | Fixed. Every page has the shared header, skip link, nav, footer, route metadata/canonical/OG/Twitter tags, icons, and sitemap/robots entries. CSP and frame protection are response headers in staticwebapp.config.json. |
| F6 plain wording | Fixed. Landing, demo, legal, README, and controls use one term: “map”. .factory/copy-audit.md records the landing copy, word counts, and terminology. |
| F7 release status | Fixed. This handoff records the repaired outcome rather than retaining a stale PASS/FAIL contradiction. |

Earlier safety corrections remain covered by the test suite: reports cannot be
written into input/source paths, symlink and catalog aliases are rejected,
mixed/orphan/truncated XMP cannot hide catalog state, source-target matching is
one-to-one, sample limits are correct, and CLI input hashes remain unchanged.

## Verification performed

From a clean clone of the pushed branch:

    npm ci
    npm test
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    npm audit --audit-level=high
    npm run build
    cargo package --allow-dirty

All passed. npm test included 7 Rust unit tests, 19 CLI integration tests, 6
site artifact tests, and 19 local Playwright tests (with expected environment
or viewport skips). The 12 individual claim commands in .factory/claims.json
also passed from the clean setup. The packed crate was installed offline into a
new consumer root; its installed edit-portability-map demo command completed
and produced a fresh populated report. The release/package binary SHA-256 was
f54af446fe842f5ed42d21de9916d2c19b4b0d9c7d3ffc563e322cee3e78d5f4.

The static site was built to dist/site/ and deployed using the existing product
configuration. HTTPS file parity was checked between the deployed home, demo,
legal, 404, JavaScript, and CSS files and local dist/site/.

Live checks at https://photo-edit-portability-map.sociobot.in:

- verify-url.sh: pass (title, lang, h1, main, alt text, labels, and no console
  errors).
- Fresh desktop and phone Playwright: 23 passed, 1 desktop-only viewport check
  skipped; sample, reset/isolation, offline demo reload, keyboard, focus,
  reduced motion, privacy request logging, links, 404, and checkout redirect
  were exercised.
- Axe serious/critical: 0.
- Lighthouse mobile report: Performance 99, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.7 s, LCP 1.7 s, CLS 0.
- Initial JS: 6,876 bytes; CSS: 16,437 bytes; mobile art: 24,670 bytes.

Evidence is under /work/.evidence/repair-4-live-final/, including the final
Lighthouse JSON and verification output. The Lighthouse runner reported a
browser-tab crash *after* producing its valid JSON report; live verification
and Playwright did not show a page error.

## Product boundaries and dependencies

- Proprietary Lightroom develop recipes are reported as unsupported; they are
  not converted. This is an intentional product boundary, not a failed path.
- The free CLI remains useful. A larger Pro verification sample uses the
  existing Sociobot checkout and license-verification API. The live checkout
  redirect and recorded invalid-license recovery were tested; no purchase was
  attempted and no payment credentials are held by this product.
- This static/local product has no backend, tenant data, shared database,
  health endpoint, or server-side rate limit. Backend-only checks therefore do
  not apply.

## Run, test, package, and deploy

    npm ci
    npm test
    npm run build
    cargo run -- demo
    cargo package

See README.md for scan usage, .factory/demo.md for the isolated demo, and
.factory/claims.json for every declared claim command. Publishing is left to
the factory; the ready-to-publish crate is made by cargo package.

## Catalog description

.factory/catalog-description.txt and
/work/.evidence/catalog-description.txt contain:
