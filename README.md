# dioxus-icons

[![Lucide](https://img.shields.io/badge/Lucide-1.14.0-2da44e)](https://lucide.dev)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7.7-0a7ea4)](https://dioxuslabs.com)

Lucide icons for Dioxus, generated as one component per icon.

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

- `dioxus_icons::lucide::Trash` imports a single Lucide icon component.
- Each icon accepts `size`, `color`, `stroke_width`, `stroke_linecap`, `stroke_linejoin`, and `class`.
- docs.rs includes a tag-driven picker backed by Lucide's upstream metadata.

Generated sources are committed under `crates/dioxus-icons/src/generated/`.
The hand-run generator lives in `crates/dioxus-icons-codegen/` and fetches the pinned Lucide release into the gitignored `vendor/` cache when needed.
Per-icon related links are backed by `crates/dioxus-icons-codegen/data/`, generated with Google SigLIP2 image/text score fusion over Lucide icons only.

## Regenerate Icons

```sh
cargo run -p dioxus-icons-codegen
```

When the Lucide version changes, regenerate the related-icon sidecar before running the Rust generator:

```sh
python3 crates/dioxus-icons-codegen/scripts/generate_related_icons.py \
  --icons-dir vendor/lucide-1.14.0/icons \
  --output crates/dioxus-icons-codegen/data/lucide-related-siglip2-base-patch16-224.json \
  --icon-set-version 1.14.0
cargo run -p dioxus-icons-codegen
```

The SigLIP2 sidecar script requires `rsvg-convert`, `torch`, `transformers`, and `Pillow`.
Generated output is deterministic. CI should run the generator and then check that `git diff --exit-code` is clean.

## Examples

```sh
cargo build --examples
```

The examples are intentionally small and copyable:

- `basic` shows one icon in a button.
- `navbar` shows several icons in app chrome.
- `tailwind` shows the `class` prop, which defaults to an empty class.
- `stateful_button` shows conditional icon rendering.

## Licensing

This crate is MIT licensed. Generated icon data comes from Lucide and is
covered by `LICENSE-LUCIDE`.
