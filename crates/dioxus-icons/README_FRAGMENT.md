# dioxus-icons

Lucide icons for Dioxus, one component per icon.

## Quick Start

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

## Picker

The crate root includes a docs.rs picker backed by Lucide's upstream tags and
categories. Typing a concept such as `delete`, `hamburger`, or `loading` finds
icons whose names alone may not contain that term.
