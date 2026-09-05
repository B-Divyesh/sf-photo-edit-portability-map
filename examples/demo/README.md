# Bundled CLI demo data

`edit-portability-map demo` copies these placeholder source and target files to
a new system temporary folder. It creates `sample.lrcat` from `catalog.sql`,
runs the normal scanner, and writes text and JSON reports in that same folder.

The files are deliberately small placeholders. The scanner lists paths and
reads XMP text; it does not decode image pixels. Remove the printed temporary
folder when you are finished inspecting the demo.
