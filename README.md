# dioxus-icons

[![Crates.io](https://img.shields.io/crates/v/dioxus-icons.svg)](https://crates.io/crates/dioxus-icons)
[![Downloads](https://img.shields.io/crates/d/dioxus-icons.svg)](https://crates.io/crates/dioxus-icons)
[![docs.rs](https://img.shields.io/docsrs/dioxus-icons.svg)](https://docs.rs/dioxus-icons)
[![Lucide](https://img.shields.io/badge/Lucide-1.14.0-2da44e)](https://lucide.dev)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7.7-0a7ea4)](https://dioxuslabs.com)

Lucide icons for Dioxus, generated as one component per icon.

```toml
[dependencies]
dioxus-icons = "0.0.1"
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
| Component usage | Render one component per icon, for example `rsx! { Trash { size: 16 } }`. There is no generic `Icon` component or string icon name. |
| `size` | Sets both SVG `width` and `height`. |
| `color` | Sets the SVG `stroke`. It defaults to `currentColor`; it does not set `fill`. |
| `stroke_width` | Sets SVG `stroke-width`. |
| `stroke_linecap` | Sets SVG `stroke-linecap`. |
| `stroke_linejoin` | Sets SVG `stroke-linejoin`. |
| `class` | Passed to the root SVG when set. It defaults to an empty string and is the right place for Tailwind classes. |

docs.rs includes a tag-driven picker backed by Lucide's upstream tags and categories.

Generated sources are committed under `crates/dioxus-icons/src/generated/`.
The hand-run generator lives in `crates/dioxus-icons-codegen/` and fetches the pinned Lucide release into the gitignored `vendor/` cache when needed.
Per-icon related links are backed by `crates/dioxus-icons-codegen/data/`, generated with Google SigLIP2 image/text score fusion over Lucide icons only.

The workspace version is currently `0.0.1`, which is the intended first publish
candidate unless it is intentionally changed before `cargo publish`.

## Dependency And Feature Policy

`dioxus-icons` supports Dioxus `0.7.x` starting at `0.7.7` (`>=0.7.7, <0.8.0`).
The published library disables Dioxus default features and enables only `html`
and `macro`, so it provides RSX/SVG components without choosing a web, desktop,
mobile, server, or fullstack renderer for your app.

`dioxus-signals` stays a normal dependency because generated Dioxus props and
RSX expansion rely on Dioxus' signal-compatible prop machinery. The crate has no
optional public features for the first publish; renderer selection belongs in
the application crate, and the docs.rs picker/widgets are part of the published
docs surface.

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
Generated output is deterministic. CI runs `ci/verify_codegen.sh` to make sure regeneration does not change the committed generated output.

## Examples

```sh
cargo build -p dioxus-icons --examples
```

The examples are intentionally small, copyable Dioxus apps that use
`dioxus::launch(App)`.

- `basic` shows one icon in a button.
- `navbar` shows several icons in app chrome.
- `tailwind` shows the `class` prop, which defaults to an empty class.
- `stateful_button` shows conditional icon rendering.

## Licensing

This crate is MIT licensed under `LICENSE`. Generated icon data comes from
Lucide and is covered by `LICENSE-LUCIDE`, which includes the upstream ISC
notice and the Feather-derived icon notice.
