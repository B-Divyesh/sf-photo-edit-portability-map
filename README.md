# Edit Portability Map

## Map Lightroom metadata before you move

Edit Portability Map is for photographers leaving Lightroom. It checks a source
folder, adjacent XMP files, an optional Lightroom catalog, and a target folder.
The report shows metadata that travels, metadata that stays in the catalog, and
metadata that needs a manual check.

It is a read-only CLI. A scan does not change supplied source, target, or
catalog files. It writes reports only to paths you name outside scanned folders.
It does not translate Lightroom develop recipes. Render finished versions that
you need to keep.

## Try the bundled demo

Run this first:

```sh
cargo run -- demo
```

The command creates a new temporary folder with three sample source photos,
three target files, a sample Lightroom-shaped SQLite catalog, and text and JSON
reports. It prints the temporary folder path. Remove that folder when you are
finished. Your own library is not used.

The same sample is available at
<https://photo-edit-portability-map.sociobot.in/demo/>.

## Install

Rust 1.85 or newer builds the binary.

```sh
cargo install --git https://github.com/B-Divyesh/sf-photo-edit-portability-map
edit-portability-map --help
```

## Scan a library copy

Copy or back up the catalog before migration. Then run a scan with folders you
want to compare:

```sh
edit-portability-map scan \
  --catalog "$HOME/Pictures/Lightroom Catalog.lrcat" \
  --source "$HOME/Pictures/Originals" \
  --target /mnt/archive/Photos \
  --target-app immich \
  --json-report portability.json
```

`--catalog` is optional for an XMP-only check. The supported target profiles
are `generic`, `immich`, `darktable`, and `digikam`. Each profile returns a
versioned JSON report when you pass `--json` or `--json-report`.

Use `--sample-size 0` to omit the verification list. Add `--fail-on-blockers`
when you want a non-zero result for migration blockers.

## Optional Pro license

The free CLI checks up to 10 files. A Pro license is needed for larger samples.
Buy or restore a license at <https://photo-edit-portability-map.sociobot.in/#license>.
Checkout opens through Sociobot.

## Read the report

- **Embedded**: expected inside the photo container.
- **Sidecar**: found in an adjacent XMP file.
- **Catalog-only**: found in the catalog more often than in XMP.
- **Unsupported**: the selected target cannot use this state directly.

Keep your originals and catalog until the destination sample has been checked.

## Develop and verify

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
cargo package --allow-dirty
```

To test every public claim from a clean setup, run every command in
`.factory/claims.json`.

To package for a consumer check:

```sh
cargo package --allow-dirty
cargo install --path target/package/edit-portability-map-0.1.0 --root /tmp/epm-consumer --offline --locked
/tmp/epm-consumer/bin/edit-portability-map demo
```

The factory owns registry publication credentials. Do not publish from this
repository.

## Privacy, terms, and deploy

The static site has no analytics, remote fonts, or third-party runtime scripts.
The browser demo uses only bundled sample data and a separate `demo:` storage
key. See the hosted [privacy policy](https://photo-edit-portability-map.sociobot.in/privacy/)
and [terms](https://photo-edit-portability-map.sociobot.in/terms/).

Deploy the contents of `dist/site/` using the durable static deployment for
`photo-edit-portability-map.sociobot.in`.

## License

MIT. See [LICENSE](LICENSE).
