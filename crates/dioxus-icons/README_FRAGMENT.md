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

## Dioxus Compatibility

`dioxus-icons` supports Dioxus `0.7.x` starting at `0.7.7` (`>=0.7.7, <0.8.0`).
The library enables Dioxus `html` and `macro` for RSX/SVG components.
Choose renderer features such as `web` or `desktop` in your application crate.

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
`LICENSE-LUCIDE`, including the upstream ISC notice and Feather-derived MIT
icon notice.
