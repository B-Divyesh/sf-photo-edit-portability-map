# Edit Portability Map — visual thesis

## Direction: luminous glass data landscape

Migration is usually described as a transfer, but the useful mental model is a
survey: image files form the terrain while metadata travels through several
transparent layers. The site therefore looks like a night-time inspection
bench—deep ink, translucent cyan plates, amber exceptions, and hairline paths.
It is deliberately not photographic portfolio chrome and not a generic SaaS
gradient. Decoration always explains the product's central question: where is
each fact stored, and will the destination understand it?

The CLI mirrors that hierarchy without pretending a terminal can be glass:
compact uppercase wayfinding, a four-state ASCII key, aligned numeric columns,
and restrained cyan/amber/red ANSI color only when stdout is a TTY.

## Palette

The site is explicitly single-mode dark because the illustration, inspection
metaphor, and status colors are designed as emitted light on a dark workbench.
The page paints every surface.

| Token | Value | Use |
| --- | --- | --- |
| Night | `#071315` | page background |
| Deep glass | `#0d2022` | elevated surfaces |
| Mist | `#f0f8f5` | primary text |
| Fog | `#a9beb9` | secondary text (7.8:1 on Night) |
| Signal cyan | `#4de7cf` | portable/safe and focus (11.3:1 on Night) |
| Cyan ink | `#05201c` | text on Signal cyan |
| Archive amber | `#ffc76b` | catalog-local/warning (11.4:1) |
| Fault coral | `#ff7b72` | unsupported/error (6.8:1) |
| Periwinkle | `#a9b8ff` | sidecar state (9.5:1) |
| Glass line | `#315557` | boundaries and quiet controls |

Status always includes a word or symbol; color never carries meaning alone.

## Type and spacing

No font files or CDN requests are needed. Headings use the local editorial
serif stack `Iowan Old Style, Palatino Linotype, Book Antiqua, Georgia, serif`;
interface copy and code use `ui-monospace, SFMono-Regular, Menlo, Consolas,
monospace`. The contrast resembles a field notebook annotated by an exacting
instrument. The scale is 14 / 16 / 20 / 28 / 44 / 72 px with 1.48–1.65 line
height for reading text. Spacing follows a 4 px base: 4, 8, 12, 16, 24, 32,
48, 72, 96. Text measures never exceed 72 characters.

## Layout and interaction grammar

The header is a thin instrument rail, not a floating navigation pill. The hero
is an asymmetric two-column survey: thesis and primary install command on the
left; an explanatory glass terrain on the right. Below it, the live report
demo reads like a migration field sheet with a four-state legend. Larger
sections use open space and divider rules rather than a grid of generic cards.
On a 390 px phone, secondary navigation disappears, the terrain becomes a
wide shallow crop, the result table becomes stacked field records, and all
controls remain at least 44 px.

Keyboard focus is a 3 px cyan outline plus dark offset. Copy, demo profile,
license verification, and disclosure controls give immediate text feedback in
an `aria-live` region. The CLI examples are real and covered by tests.

## Motion policy

Only three motions exist: the terrain settles upward once on entry (420 ms),
glass rows disclose over 180 ms, and copied text gives a short opacity change.
Nothing loops. Under `prefers-reduced-motion: reduce`, transforms and smooth
scrolling are disabled and all state changes are instant.

## Asset plan and provenance

- `site/public/portability-landscape.webp`: original raster hero, generated
  for this product with the factory image generator (`factory-image`) on
  2026-08-28, then converted locally to WebP. Prompt: “A luminous glass data
  landscape for a photo metadata migration utility: translucent stacked
  archival plates in a dark teal inspection room, thin cyan routes passing
  safely through layers, a few amber routes stopping inside a catalog layer,
  subtle contact-sheet geometry and XMP-like marks, editorial scientific
  still life, wide composition with calm negative space, deep ink background,
  cyan, periwinkle and amber emitted light, crisp glass edges, tactile but
  abstract, no people, no cameras, no logos, no text, no watermark.” Licensed
  for this project as generated output.
- Interface symbols are original CSS/Unicode constructions. No stock icons,
  third-party artwork, remote fonts, scripts, or visual libraries are used.

The illustration is explanatory: cyan paths cross layers while amber paths
remain trapped, making “portable versus app-local” legible before the copy is
read.
