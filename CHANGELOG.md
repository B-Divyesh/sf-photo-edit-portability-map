# Changelog

All notable changes follow [Keep a Changelog](https://keepachangelog.com/) and
this project uses semantic versioning.

## [0.1.0] - 2026-08-28

### Added

- Read-only Lightroom SQLite and XMP inventory.
- Target profiles for Immich, darktable, digiKam, and a generic file workflow.
- Human and versioned JSON reports with checklists and verification samples.
- Responsive, local-first product documentation and license restoration.

### Fixed

- Ignore and warn about orphan or relocated XMP documents so they cannot hide
  catalog-only metadata; case-variant adjacent sidecars remain supported.
- Reserve target assets one-to-one, prioritizing exact matches before safe
  stem fallbacks so paired RAW and JPEG sources cannot share one target.
- Validate the 100-file hard sample limit before checking the optional Pro
  license.
- Correct the visible install URL and enforce 44 px desktop navigation targets.
- Prevent report paths, including paths through symlinks or hard links, from
  overwriting the input catalog or writing inside scanned folders.
- Serve fingerprinted site assets with a one-year immutable cache policy.
- Conservatively flag mixed catalog/XMP coverage as catalog-only state instead
  of hiding uncovered values behind a category-wide sidecar classification.
- Match safely flattened or reorganized targets by an unambiguous file stem,
  while leaving duplicate names unmatched for review.
- Warn when an XMP sidecar ends with unclosed XML elements.
- Keep the 390 px hero and its controls inside the viewport and give every
  visible link a 44 px minimum touch target.
- Bind cached browser license verdicts to the token they verified so a restored
  or revoked token cannot inherit another token's Pro state.
