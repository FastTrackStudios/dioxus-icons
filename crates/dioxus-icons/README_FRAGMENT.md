## Quick Start

Add the crate to your Dioxus app:

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
| `size` | Uses one value for `width` and `height`. |
| `color` | Sets the stroke color for Lucide's line icons; defaults to `currentColor`. |
| `stroke_width`, `stroke_linecap`, `stroke_linejoin` | Forwarded to the matching SVG stroke attributes. |
| `class` | Passed to the root SVG for CSS or Tailwind classes. |

## Dependency Surface

`dioxus-icons` supports Dioxus `0.7.x` starting at `0.7.7` (`>=0.7.7, <0.8.0`).
The published library enables Dioxus `html` and `macro` for RSX/SVG components.
Choose renderer features such as `web` or `desktop` in your application crate.

`dioxus-signals` is a normal dependency because generated Dioxus props and RSX
expansion rely on Dioxus' signal-compatible prop machinery.

## Examples

Examples live under `crates/dioxus-icons/examples/`. Build them with:

```sh
cargo build -p dioxus-icons --examples
```

Run one with:

```sh
cargo run -p dioxus-icons --example basic
```

## Generated Icons

Regenerate committed Rust sources under `crates/dioxus-icons/src/generated/` with:

```sh
cargo run -p dioxus-icons-codegen
```

The generator uses the pinned Lucide release and the related-icon sidecar under
`crates/dioxus-icons-codegen/data/`. If the Lucide pin changes, regenerate and
review the generated manifest and component diff before publishing.

## Licensing

The crate code is MIT licensed. Generated Lucide icon data is covered by
`LICENSE-LUCIDE`.
