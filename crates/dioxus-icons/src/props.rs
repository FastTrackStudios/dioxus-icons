use dioxus::prelude::*;

/// Properties shared by every Lucide icon component.
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// SVG width and height in CSS pixels.
    #[props(default = 24)]
    pub size: u32,
    /// SVG stroke color.
    ///
    /// This maps to the root SVG `stroke` attribute, not `fill`.
    #[props(into, default = "currentColor")]
    pub color: std::borrow::Cow<'static, str>,
    /// SVG stroke width.
    #[props(default = 2)]
    pub stroke_width: u32,
    /// SVG stroke linecap.
    #[props(into, default = "round")]
    pub stroke_linecap: std::borrow::Cow<'static, str>,
    /// SVG stroke linejoin.
    #[props(into, default = "round")]
    pub stroke_linejoin: std::borrow::Cow<'static, str>,
    /// Class passed to the root SVG.
    #[props(into, default = "")]
    pub class: std::borrow::Cow<'static, str>,
}
