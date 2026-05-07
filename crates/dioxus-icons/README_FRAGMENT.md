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
