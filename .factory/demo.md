# Demo sandbox

## CLI command

Run `edit-portability-map demo` after installation, or `cargo run -- demo` in
this repository. The command creates a new system temporary folder. It copies
the bundled placeholders from `examples/demo/`, creates `sample.lrcat` from
`examples/demo/catalog.sql`, and runs the normal scan against those copies.

The command prints the folder path. It writes `portability-report.txt` and
`portability-report.json` there. It reads no user-supplied path and does not
change a real library. Remove the printed temporary folder after inspection.

## Website route

Open <https://photo-edit-portability-map.sociobot.in/demo/> or use the
first-screen **Try it with sample data** action. The route immediately displays
a realistic report for three source files, three matched target files, and
catalog-only metadata.

The page uses only `localStorage` key
`demo:photo-edit-portability-map:state`. The key stores the selected target
profile. **Reset demo** returns it to Immich. **Start for real** removes it
before returning to the home page. No non-demo storage is read or changed.

The service worker caches `/demo/` after its first HTTPS visit. The browser
claim test opens a new context, visits this route, then reloads it offline.
