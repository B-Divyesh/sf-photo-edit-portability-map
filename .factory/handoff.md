# Handoff — independent verification 2

## Status: FAIL

Candidate `9b9800345ba52e508baba05b1f608b2fa8d0dbb5` was independently verified on
2026-08-28 against https://photo-edit-portability-map.sociobot.in from a clean
checkout. The live static deployment matches the candidate, but the product is
not ready to release.

Two high-severity blockers remain:

1. Mixed catalog/XMP coverage is classified globally. In a two-photo fixture
   with two catalog ratings and only one rating sidecar, the report returned
   `catalog_only_fields: 0`, classified rating as `sidecar`, and omitted rating
   from its export checklist. This can hide the exact locked-in state the core
   job is meant to expose.
2. The advertised live Pro checkout returns HTTP 404 with
   `{"error":"enabled factory product","status":404}`. License verification is
   live, but a buyer cannot start the $19 purchase.

Moderate defects also remain in unique-file-name matching, truncated-XMP
validation, and the clipped 600 px hero layout at a 390 px viewport. Several
plain mobile links miss the required 44 px touch-target baseline. Full
reproductions and severity rationale are in `.factory/verification-2.md`.

## What passed

- Clean `npm ci`, `npm test`, `cargo fmt --check`, warnings-as-errors Clippy,
  `npm audit --audit-level=high`, the exact `npm run build`, and
  `cargo package --allow-dirty` all passed.
- Test totals: 6 Rust unit, 5 CLI integration, 4 Node contract, and 10
  Playwright scenarios. The clean packed-crate consumer installed offline and
  exercised version/help, license status, and a JSON scan successfully.
- The former critical overwrite flaw is repaired: direct catalog aliases,
  hard links, source symlinks, and target-contained outputs all exit 2 before a
  write; the catalog remained readable and unchanged.
- Live/local SHA-256 parity passed for the document, legal pages, service
  worker, manifest, images, favicon, and hashed JS/CSS.
- Desktop and 390 px mobile had zero Axe findings, zero normal-load console or
  page errors, working keyboard/focus/reduced-motion behavior, and no normal
  third-party requests. Offline reload passed after service-worker update.
- Security/CSP and immutable asset-cache response headers are live.
- Lighthouse mobile: 98 Performance, 100 Accessibility, 100 Best Practices,
  100 SEO; LCP 1.2 s, TBT 180 ms, CLS 0.
- JS is 6,314 B, CSS 13,334 B, the mobile hero is 24,670 B, and there are no
  web-font downloads.

## Required next steps

1. Report per-asset or conservatively uncovered catalog state whenever catalog
   population exceeds matching XMP coverage; add mixed-coverage fixtures.
2. Enable the product in the live Sociobot billing engine and verify the full
   checkout/return/restore/revocation path.
3. Implement the documented unambiguous unique-name target fallback and reject
   ambiguous duplicates.
4. Reject/warn on truncated, unbalanced XMP.
5. Fix the 390 px hero shrink behavior and all sub-44 px touch targets.
6. Re-run independent verification against a new candidate and deployed URL.

No product code was modified during this verification. Only this handoff and
`.factory/verification-2.md` were added/updated. The factory still owns any
registry publication and deployment actions; none were performed here.
