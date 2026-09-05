# Repair 5 handoff — Edit Portability Map

> Independent verification 6 completed after this repair. The current result
> is **PASS** with zero findings and zero untested public claims. See
> `.factory/verification-6.md` for the clean-checkout, live, claim, and
> consumer-artifact evidence. It reviewed implementation
> `f13374d266494b2aa81403f1c895d411d1d51c03` and documentation
> `2b0e627a03cbd2e641c3694e8541a2fec2fff03f`.

## Status

**PASS.** Review 3's two findings and two untested public claims are resolved.
No known product defect remains.

- Work order: `photo-edit-portability-map-repair-5`
- Live URL: <https://photo-edit-portability-map.sociobot.in>
- Repair base: `c2d3dc905ed8715e4d4449a480b4b95e34edb851`
- Implementation SHA: `f13374d266494b2aa81403f1c895d411d1d51c03`
- Main repair commit: `f65d014cc92da3442b0525fc0d790b57d2c55116`
- Verification date: 2026-09-05 UTC

The implementation SHA includes a final privacy-copy correction after the main
repair commit. The later handoff commit changes documentation only; its SHA is
reported separately in the final delivery.

## What changed

- Removed the 9 px mobile overflow at its cause. Responsive grid tracks now use
  `minmax(0, 1fr)`, and both Pro-section children may shrink inside the track.
- Expanded the 390 px browser regression to check the full home page and demo,
  including exact viewport/document width equality and 44 px visible targets.
- Added `site-runtime-privacy` claim coverage. Its browser test visits home,
  demo, Privacy, Terms, and 404 while recording every request. It rejects
  external requests, non-GET requests, and runtime font requests.
- Added `license-daily-verdict` claim coverage. Its browser test proves token
  and verdict storage, reuse before 24 hours, refresh after 24 hours, and the
  exact Sociobot verification endpoint as the only external destination.
- Tightened the paid-sample claim to cover the 10-file free boundary and the
  100-file Pro boundary with observable CLI outcomes.
- Corrected Privacy copy to match reset behavior: Reset restores Immich, while
  leaving the demo removes the demo key.

## Product and demo result

The job is to map Lightroom metadata before moving photos. It is for
photographers leaving Lightroom. Before scrolling, the first action is **Try
it with sample data**, followed by the temporary-folder explanation.

Fresh 1440 × 900 desktop and 390 × 844 phone contexts both showed that job,
audience, and first action at scroll position zero. The phone document and
viewport widths were both exactly 390 px.

One click opened `/demo/` with the persistent **Demo — sample data, nothing is
saved** label and five populated rows. The rows include embedded EXIF, sidecar
keywords, catalog-only dates and ratings, and an unsupported Lightroom develop
recipe. Reset restored Immich. Start for real removed only the `demo:` key; a
seeded real-data key remained unchanged.

## Verification

From the documented clean setup:

- `npm ci`: passed; 0 vulnerabilities.
- `npm test`: passed with 7 Rust unit tests, 20 Rust CLI integration tests, 6
  Node contract/claim tests, and 23 applicable local browser tests. Five local
  environment or viewport skips were expected.
- Every command in `.factory/claims.json` passed individually: 14 of 14.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed with 0 vulnerabilities.
- `npm run build`: passed and produced `dist/site/` and the release binary.
- `cargo package --allow-dirty`: passed; the crate is ready to publish.

The packaged crate installed offline into a fresh consumer root. Its installed
binary matched the release binary at SHA-256
`f54af446fe842f5ed42d21de9916d2c19b4b0d9c7d3ffc563e322cee3e78d5f4`.
The installed `demo` command produced three matched photos, three catalog-only
categories, a develop-recipe blocker, and populated text and JSON reports in a
new temporary folder. The crate SHA-256 is
`c748961b1a40dc25b75f35855b92022f0f93ce8ad334017cb88f963e18b7158f`.

## Live checks

- The durable static deployment completed for the existing
  `sf-photo-edit-portability-map` resource. Replica and SQLite checks do not
  apply because this product is a static site plus a local CLI.
- Fresh live Playwright: 27 passed and 1 expected desktop-only skip. This covers
  desktop, phone, demo reset/isolation, keyboard, focus, reduced motion,
  accessibility, privacy requests, license caching, checkout, and offline
  reload.
- Integrated Axe checks found 0 serious or critical issues on home, demo,
  Privacy, Terms, and 404. `verify-url.sh` passed with one h1, `lang=en`, a main
  landmark, complete image alt text, labelled buttons, and no console errors.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200. A missing
  path returns the designed page with HTTP 404, as expected.
- Every discovered link resolved. The registered checkout returned 303 to the
  hosted Dodo page, which showed Edit Portability Map Pro, $19.00, and a
  one-time purchase description.
- The root response sends CSP with `frame-ancestors 'none'`, HSTS, `nosniff`,
  and a strict referrer policy. Hashed assets use one-year immutable caching.
- All 17 checked local and live public files have identical SHA-256 digests.

Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100,
SEO 100; FCP 0.87 s, LCP 1.20 s, TBT 17 ms, CLS 0, Speed Index 0.87 s.
Initial JavaScript is 6.88 kB, CSS is 16.50 kB, and the mobile hero is 24.67
kB. No web font payload is shipped.

## Earlier finding disposition

| Finding set | Current disposition |
| --- | --- |
| Verification 1: unsafe report paths and asset caching | Catalog aliases and symlinked input paths are refused by passing black-box tests. Live hashed assets are immutable. |
| Verification 2: partial XMP, matching, malformed XMP, checkout, mobile clipping, touch targets | All dedicated regressions pass. The registered live checkout and 390 px geometry pass. |
| Verification 3: orphan XMP, one-to-one matching, install URL, Pro target, sample bound | All dedicated CLI/site regressions pass. |
| Review 1 and 2: demo, claims, first screen, routing, metadata, copy, release status | The isolated CLI/browser demo, plain first screen, claim registry, route shell, 404, copy audit, and correct status are present and pass. |
| Review 3: 9 px overflow and two untested claims | Closed by the responsive-grid regression and the two exact claim tests above. |

## Known gaps and next steps

- The factory owns registry credentials, so the crate was packaged and consumer
  tested but not published.
- No payment or refund was performed. The live offer and checkout contents were
  verified without entering payment details. Recorded verification responses
  cover browser entitlement behavior.
- `/work/.evidence/billing-offer.json` records the live offer for the separate
  billing-registration operator. No credentials are present.

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

Run every command in `.factory/claims.json`. For a clean consumer check:

```sh
cargo install --path target/package/edit-portability-map-0.1.0 --root /tmp/epm-consumer --offline --locked
/tmp/epm-consumer/bin/edit-portability-map demo
```

## Independent verification 6

The fresh independent review passed every declared claim command (14 of 14),
the clean setup quality gates, a clean packaged consumer installation, and the
live Playwright suite (27 passed, 1 expected viewport skip). The live output
matched the rebuilt implementation for 17 checked files. The static CLI has no
backend-only tenant, persistence, health, or 429 behavior to verify. The
separate report is `.factory/verification-6.md`.
