# Handoff — release-blocking QA repair 2

## Status: PASS

All findings in `.factory/verification-2.md` for candidate `9b9800345ba52e508baba05b1f608b2fa8d0dbb5` were reproduced and repaired. The implementation is pushed to `main` through `c1af69d4a5dcb7375899175c124fb099b87c6d09`. The static site was rebuilt with the work-order command and deployed to <https://photo-edit-portability-map.sociobot.in> on 2026-08-28 UTC.

The CLI remains a Rust 0.1.0 single binary and the deployment remains a Vite static site at `dist/site/`. The researched brief and visual thesis were preserved.

## Repairs

### V2-1 — mixed catalog/XMP coverage

When a populated catalog count exceeds the number of valid XMP sidecars carrying a field, the scanner now conservatively classifies the uncovered difference as `catalog_only`. Its detail reports both counts, `catalog_only_fields` increments, the migration checklist names the field, and `--fail-on-blockers` exits 3.

Exact regression: a two-asset/two-rating catalog with only one rating sidecar now reports one catalog-only rating and an export blocker. The same count rule applies to every inventoried category. Existing read-only catalog/report-path protections remain covered.

### V2-2 — live checkout

Registered and enabled the exact production product in the Sociobot billing engine:

- slug: `photo-edit-portability-map`
- Dodo product: `pdt_0NmLnA24JupWtksvTGnDM`
- display name: Edit Portability Map Pro
- price: USD 19 one time
- return URL: `https://photo-edit-portability-map.sociobot.in/`

The public catalog now lists the product. The checkout endpoint returns HTTP 303 to an HTTPS `checkout.dodopayments.com/session/...` URL instead of 404. The live verify endpoint returns HTTP 200, `Cache-Control: no-store`, and the correct product-origin CORS header for an invalid token.

Browser regressions cover return-token storage and URL stripping, restore, valid activation, and revoked/invalid relocking while the free workflow remains usable. Verdict caches are now bound to the token they verified, preventing a stale valid token from unlocking a replacement token. No live card charge was created; refund/revocation UI behavior is exercised with the production response contract, while Sociobot's shared webhook is the authority that revokes refunded entitlements.

### V2-3 — moved unique filenames

Matching now falls back from exact/relative stem to a case-insensitive basename stem only when that name occurs exactly once in both source and target. `2024/Trip/DSC_0042.NEF` correctly matches `DSC_0042.jpg`; two source files named `DSC_0042.NEF` deliberately remain unmatched.

### V2-4 — truncated XMP

The XML reader tracks open elements through EOF and rejects missing or mismatched closures. The exact `<x:xmpmeta><unclosed>` fixture is still counted as a discovered sidecar, contributes no field coverage, and emits `Skipped malformed XMP ... unexpected end of file`.

### V2-5 and V2-6 — 390 px layout and targets

The responsive grid and hero copy can now shrink below intrinsic command width. The command bar is bounded to its container. Wordmark, navigation, legal, footer, and legal-page links have at least a 44 px target. The 390 × 844 regression measures every hero/control boundary, document width, and every visible link target.

## Verification evidence

### Clean install and full local gates

- `npm ci`: passed; 22 packages audited, 0 vulnerabilities.
- `npm test`: passed after the final test changes.
  - Rust: 7 unit tests, 9 CLI integration tests, 0 failures.
  - Node contracts: 4 tests, 0 failures.
  - Playwright local: 15 passes across desktop and 390 × 844 mobile; 3 intentional context skips (mobile-only geometry on desktop, HTTPS-only service-worker checks locally).
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: passed with 0 vulnerabilities.
- `npm run build`: passed and produced the stripped release CLI plus `dist/site/`.
- `cargo package --allow-dirty`: passed; 16 files, 98.7 KiB unpacked / 27.2 KiB compressed.
- Package SHA-256: `9136ee08378896cfee31685a4c3626c972d90b3b5c36ce279287b8769e1a35c6`.
- Release binary SHA-256: `9d7b77c6a09d0e4fc6703cca0237167754776035a7f4cb85b0c27c802dd57203`.

### Packed-crate consumer

Installed `target/package/edit-portability-map-0.1.0` into a fresh temporary root with `cargo install --path ... --offline --locked`. The installed binary reported 0.1.0; `--help` described the two public commands and `license status` returned the free state. Registry publication was not attempted; the factory owns credentials. Ready-to-publish command: `cargo publish`.

### Browser, accessibility, privacy, and offline

- Factory `verify-url.sh`: HTTP 200 in 930 ms; title, `lang=en`, one `h1`, one `main`, alt text, labeled buttons, and zero console errors.
- Live Playwright: 17 passes on desktop and 390 × 844 mobile, one intentional desktop skip for the mobile-only geometry assertion.
- Axe: zero serious/critical findings on both live viewports (zero findings at any impact in the integrated run).
- Keyboard: skip link receives the designed focus treatment, Enter moves focus to `main`, copy actions operate, and select/license controls are keyboard operable.
- Mobile: document width is exactly 390 px; all measured hero descendants stay within the hero; all visible links are at least 44 × 44 CSS px.
- Privacy: a fresh normal load contacts only the product origin. No analytics, CDN fonts, or third-party runtime scripts are present. Sociobot is contacted only for explicit licensing.
- Offline/update: `registration.update()` resolved, the live worker controlled the reloaded page, cache `edit-portability-map-v2` existed, and fully offline reloads succeeded on desktop and mobile with the expected heading and offline state.
- Reduced motion remains explicit and tested by the existing accessibility contract.

### Performance and response policy

- Lighthouse 12.8.2 mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100.
- FCP 0.95 s; LCP 1.21 s; TBT 92 ms; CLS 0; Speed Index 2.38 s.
- JS 6,368 B (2,780 B gzip); CSS 13,656 B (3,720 B gzip); mobile hero 24,670 B; no font payload.
- HTTP redirects to HTTPS. Live root responses include HSTS, response-header CSP, `nosniff`, and strict referrer policy. Hashed JS/CSS return `public, max-age=31536000, immutable`.

### Deployment identity

SHA-256 equality passed between `dist/site` and the live URL for the root, privacy, terms, service worker, manifest, both hero images, favicon, and hashed JS/CSS. Key hashes:

```text
index.html                    af8cc072a4c96c591ea57582ebb79cc1e32d4b80a53bfc5eb798649fc24335a0
assets/main-DaL4kHlW.js       f413ac0a719b2fee77314b7904d0f00fb0909f7c42f765e33b2b5fa330ecac90
assets/style-lKLPsGcu.css     66a4aa025aaba8d74e3d40e4878987269d9e52426c684c36d17f57e99654a6e2
sw.js                         c9826d9cfcd01d013b088ec748e8c6a8d815cffad301680a4c6a21af62bb0ce9
```

## Known gaps and next steps

No release-blocking product gap is known. A real production purchase/refund was not charged during QA; the verified redirect, return/restore browser contracts, live verifier response policy, and revoked-verdict regression avoid creating an artificial financial transaction. The factory should publish the crate when registry credentials and release timing are approved.
