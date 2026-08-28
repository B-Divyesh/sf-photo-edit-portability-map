# Edit Portability Map

Edit Portability Map is a read-only pre-flight CLI for photographers leaving
Lightroom or auditing an external photo library. It inventories file names,
XMP sidecars, and a Lightroom SQLite catalog; classifies visible state as
embedded, sidecar, catalog-only, or unsupported by the selected destination;
and produces a migration checklist plus a deterministic verification sample.

It never changes the source tree or catalog, never decodes image pixels, and
has no telemetry. It also does not claim to translate proprietary Lightroom
development recipes: those must be rendered or rebuilt in the destination.

## Install

Build the single binary with stable Rust 1.85 or newer:

```sh
cargo install --git https://github.com/B-Divyesh/sf-photo-edit-portability-map
edit-portability-map --help
```

Release artifacts can be built with `cargo build --release`. The binary is
`target/release/edit-portability-map`.

## Usage

Run a useful free inventory against a Lightroom catalog, source library, and
destination folder:

```sh
edit-portability-map scan \
  --catalog "$HOME/Pictures/Lightroom Catalog.lrcat" \
  --source "$HOME/Pictures/Originals" \
  --target /mnt/archive/Photos \
  --target-app immich
```

Write stable JSON for scripts or later comparison:

```sh
edit-portability-map scan \
  --source ./Originals \
  --target ./Export \
  --target-app darktable \
  --json > portability-report.json
```

Or save both human and machine reports without shell redirection:

```sh
edit-portability-map scan --source ./Originals --target ./Export \
  --report report.txt --json-report report.json
```

`--catalog` is optional for XMP-only workflows. `--source` and `--target` are
always required. Supported target profiles are `generic`, `immich`,
`darktable`, and `digikam`. Use `--sample-size 0` to omit the verification
sample, or a value up to 10 in the free edition. Exit code `0` means the scan
completed, `2` means an input/usage problem, and `3` means the inventory found
migration blockers (only with `--fail-on-blockers`).

Saved reports must resolve outside the source and target trees and cannot
alias the input catalog. The CLI resolves symlinked paths and existing parent
directories before scanning, then refuses unsafe output with exit code `2`.

The JSON shape is versioned with `schema_version`. The documented examples are
covered by integration tests.

## What the classifications mean

- **Embedded** — normally travels inside the photo container. The CLI reports
  this as an expected location without opening binary image data.
- **Sidecar** — observed in an `.xmp` file next to a source asset.
  Adjacency is matched by case-normalized relative path and stem; relocated or
  orphan XMP files are warned about and cannot mask catalog-only state.
- **Catalog-only** — observed in Lightroom tables without equivalent sampled
  sidecar coverage. If a field appears in more populated catalog rows than XMP
  sidecars, the uncovered difference is conservatively reported here; export,
  render, or recreate it before leaving.
- **Target-unsupported** — the selected profile cannot faithfully consume the
  source representation. It is a capability warning, not a conversion error.

The generated checklist is intentionally conservative. Verify the sample in
the actual destination before deleting or cancelling anything. Target matching
is one-to-one: exact relative paths are reserved first, then only unambiguous
relative-stem or unique-name fallbacks are used.

## Pro unlock

The free scan, safety warnings, and JSON export are never gated. A one-time
$19 Pro license unlocks verification evidence packs up to 100 files. Purchase
and restore are handled by Sociobot, the merchant of record, on the project
site. Activate the returned token once, or supply it through the environment:

```sh
edit-portability-map license activate YOUR_TOKEN
edit-portability-map scan --source ./Originals --target ./Export --sample-size 50
# CI: EDIT_PORTABILITY_MAP_LICENSE=YOUR_TOKEN edit-portability-map scan ...
```

The verdict is cached in the user config directory and checked at most daily.
A previously valid license remains optimistic during a temporary network
failure. The token is never written into a report.

## Develop and verify

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo package --allow-dirty
npm install
npm test
npm run build
```

`npm run build` runs the Rust release build and the Vite documentation site;
the static deployment output is exactly `dist/site/`. `npm run build:site`
builds only the site. The site contains a browser-local recorded demo; it does
not upload files.

## Privacy and limitations

There is no analytics or runtime third-party script. The website stores only a
license token and daily verification cache when a buyer opts in. The CLI reads
SQLite with read-only/query-only flags and reads XMP text; it does not access
cloud credentials, image pixels, or write source metadata. A copied catalog is
still recommended because Lightroom may itself be using a live catalog.

See the hosted [privacy](https://photo-edit-portability-map.sociobot.in/privacy/)
and [terms](https://photo-edit-portability-map.sociobot.in/terms/) pages.

## License

MIT. See [LICENSE](LICENSE).
