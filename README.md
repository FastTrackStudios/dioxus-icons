# dioxus-icons

[![Crates.io](https://img.shields.io/crates/v/dioxus-icons.svg)](https://crates.io/crates/dioxus-icons)
[![Downloads](https://img.shields.io/crates/d/dioxus-icons.svg)](https://crates.io/crates/dioxus-icons)
[![docs.rs](https://img.shields.io/docsrs/dioxus-icons.svg)](https://docs.rs/dioxus-icons)
[![Lucide](https://img.shields.io/badge/Lucide-1.14.0-2da44e)](https://lucide.dev)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7.7-0a7ea4)](https://dioxuslabs.com)

Lucide icons for Dioxus, generated as one component per icon.

```sh
cargo add dioxus-icons
```

```rust
use dioxus::prelude::*;
use dioxus_icons::lucide::Trash;

fn DeleteButton() -> Element {
    rsx! {
        button {
            Trash { size: 16 }
            "Delete"
        }
    }
}
```

The public API is deliberately flat and component-first:

| Item | Notes |
| --- | --- |
| Import path | Import each icon from `dioxus_icons::lucide`, for example `use dioxus_icons::lucide::Trash;`. |
| Component usage | Render icon components directly, for example `rsx! { Trash { size: 16 } }`. |
| `size` | Uses one value for SVG `width` and `height`. |
| `color` | Sets the stroke color for Lucide's line icons; defaults to `currentColor`. |
| `stroke_width`, `stroke_linecap`, `stroke_linejoin` | Forwarded to the matching SVG stroke attributes. |
| `class` | Passed to the root SVG for CSS or Tailwind classes. |

docs.rs includes a tag-driven picker backed by Lucide's upstream tags and categories.

## Dependency And Feature Policy

`dioxus-icons` supports Dioxus `0.7.x` starting at `0.7.7` (`>=0.7.7, <0.8.0`).
The published library enables Dioxus `html` and `macro` for RSX/SVG components.
Choose renderer features such as `web`, `desktop`, `mobile`, `server`, or
`fullstack` in your application crate.

`dioxus-signals` stays a normal dependency because generated Dioxus props and
RSX expansion rely on Dioxus' signal-compatible prop machinery. The docs.rs
picker/widgets are part of the published docs surface.

## Regenerate Icons

```sh
cargo run -p dioxus-icons-codegen
```

Generated Rust sources are committed under `crates/dioxus-icons/src/generated/`.
The generator uses the pinned Lucide release and related-icon sidecar under
`crates/dioxus-icons-codegen/data/`.

When the Lucide version changes, regenerate the related-icon sidecar before running the Rust generator:

```sh
python3 crates/dioxus-icons-codegen/scripts/generate_related_icons.py \
  --icons-dir vendor/lucide-1.14.0/icons \
  --output crates/dioxus-icons-codegen/data/lucide-related-siglip2-base-patch16-224.json \
  --icon-set-version 1.14.0
cargo run -p dioxus-icons-codegen
```

The SigLIP2 sidecar script requires `rsvg-convert`, `torch`, `transformers`, and `Pillow`.
Generated output is deterministic; regenerate after codegen input changes and review the generated diff.

## Examples

Examples live under `crates/dioxus-icons/examples/`.

```sh
cargo build -p dioxus-icons --examples
```

```sh
cargo run -p dioxus-icons --example basic
```

## Licensing

The crate code is MIT licensed under `LICENSE`. Generated icon data comes from
Lucide and is covered by `LICENSE-LUCIDE`, which includes the upstream ISC
notice and the Feather-derived MIT icon notice. The published crate therefore
uses the SPDX expression `MIT AND ISC`.
