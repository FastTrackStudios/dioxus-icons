## Quick Start

Add the crate to your Dioxus app:

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

Icons are available under the Lucide-specific module at
`dioxus_icons::lucide::*`.

## API Shape

Import one component per icon and render that component directly:

```rust
use dioxus::prelude::*;
use dioxus_icons::lucide::Trash;

let _ = rsx! {
    Trash { size: 16, color: "red" }
};
```

| Prop | SVG mapping |
| --- | --- |
| `size` | Sets both `width` and `height`. |
| `color` | Sets `stroke`; defaults to `currentColor`. |
| `stroke_width` | Sets `stroke-width`. |
| `stroke_linecap` | Sets `stroke-linecap`. |
| `stroke_linejoin` | Sets `stroke-linejoin`. |
| `class` | Passed to the root SVG when set. |

There is no generic `Icon` component and no string icon name API.

## Dependency Surface

`dioxus-icons` supports Dioxus `0.7.x` starting at `0.7.7` (`>=0.7.7, <0.8.0`).
The published library enables only Dioxus `html` and `macro`; it does not enable
or proxy renderer features such as `web` or `desktop`.

`dioxus-signals` is a normal dependency because generated Dioxus props and RSX
expansion rely on Dioxus' signal-compatible prop machinery. The crate has no
optional public features for the first publish.

## Examples

The crate includes small runnable examples:

- `basic` shows one icon in a button.
- `navbar` shows several icons in app chrome.
- `tailwind` shows Tailwind classes passed through `class`.
- `stateful_button` shows conditional icon rendering.

Build them with:

```sh
cargo build -p dioxus-icons --examples
```

Run one with:

```sh
cargo build -p dioxus-icons --examples
```

The examples use the standard `dioxus::launch(App)` entrypoint.

## Generated Icons

Generated Rust sources live under `crates/dioxus-icons/src/generated/` and are
not edited by hand. Regenerate them with:

```sh
cargo run -p dioxus-icons-codegen
```

The generator uses the pinned Lucide release and the related-icon sidecar under
`crates/dioxus-icons-codegen/data/`. If the Lucide pin changes, regenerate and
review the generated manifest and component diff before publishing.

## Licensing

The crate code is MIT licensed. Generated Lucide icon data is covered by
`LICENSE-LUCIDE`.
