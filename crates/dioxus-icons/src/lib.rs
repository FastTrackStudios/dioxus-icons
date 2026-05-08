//! # dioxus-icons
//!
//! Lucide icons for Dioxus, one component per icon.
#![warn(missing_docs)]
#![cfg_attr(any(doc, rust_analyzer), doc = include_str!("../STATIC_PICKER.html"))]
#![cfg_attr(any(doc, rust_analyzer), doc = concat!(
    "<script type=\"application/json\" id=\"__icon_manifest__\">",
    include_str!("generated/manifest.json"),
    "</script>"
))]
#![cfg_attr(any(doc, rust_analyzer), doc = include_str!("../picker/picker.html"))]
#![cfg_attr(any(doc, rust_analyzer), doc = concat!(
    "<script>",
    include_str!("./js/picker.js"),
    "</script>"
))]
//!
#![cfg_attr(any(doc, rust_analyzer), doc = include_str!("../README_FRAGMENT.md"))]

mod props;
mod vdom;

#[allow(missing_docs)]
mod generated;

pub use props::IconProps;

/// Lucide icons.
pub use crate::generated::lucide;
