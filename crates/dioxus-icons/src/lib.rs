//! # dioxus-icons
//!
//! Lucide icons for Dioxus, one component per icon.
#![warn(missing_docs)]
#![doc = include_str!("../STATIC_PICKER.html")]
#![doc = concat!(
    "<script type=\"application/json\" id=\"__icon_manifest__\">",
    include_str!("generated/manifest.json"),
    "</script>"
)]
#![doc = include_str!("../picker/picker.html")]
#![doc = concat!(
    "<script>",
    include_str!(concat!(env!("OUT_DIR"), "/picker.js")),
    "</script>"
)]
//!
#![doc = include_str!("../README_FRAGMENT.md")]

#[allow(missing_docs)]
mod generated;

/// Lucide icons.
pub use crate::generated::lucide;
