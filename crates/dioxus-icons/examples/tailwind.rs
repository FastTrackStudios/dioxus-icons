use dioxus::prelude::*;
use dioxus_icons::lucide::{Bell, Menu, Search, Settings};

fn main() {
    // Add your preferred Dioxus renderer launch here, such as web or desktop.
    let _app: fn() -> Element = App;
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        nav {
            class: "flex items-center gap-3 border-b border-slate-200 px-4 py-3 text-slate-900",
            Menu { size: 20, class: "text-slate-700" }
            strong { class: "mr-auto", "Tailwind Example" }
            Search { size: 18, class: "text-slate-500" }
            Bell { size: 18, class: "text-amber-600" }
            Settings { size: 18, class: "text-slate-600" }
        }
    }
}
