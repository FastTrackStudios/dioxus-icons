use dioxus::prelude::*;
use dioxus::ssr::render_element;
use dioxus_icons::lucide::{Calendar1, Trash, Type};

fn render_icon(element: Element) -> String {
    render_element(element)
}

#[test]
fn default_svg_shape() {
    let html = render_icon(rsx! { Trash {} });

    assert!(html.starts_with("<svg "));
    assert!(html.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
    assert!(html.contains(r#"width="24""#));
    assert!(html.contains(r#"height="24""#));
    assert!(html.contains(r#"viewBox="0 0 24 24""#));
    assert!(html.contains(r#"fill="none""#));
    assert!(html.contains(r#"stroke="currentColor""#));
    assert!(html.contains(r#"stroke-width="2""#));
    assert!(html.contains(r#"stroke-linecap="round""#));
    assert!(html.contains(r#"stroke-linejoin="round""#));
    assert!(!html.contains("class="));
    assert!(html.contains(r#"<path d="M19 6v14"#));
}

#[test]
fn props_map_to_svg_attributes() {
    let html = render_icon(rsx! {
        Trash {
            size: 32,
            color: "red",
            stroke_width: 3,
            stroke_linecap: "butt",
            stroke_linejoin: "bevel",
            class: "text-red-500",
        }
    });

    assert!(html.contains(r#"width="32""#));
    assert!(html.contains(r#"height="32""#));
    assert!(html.contains(r#"fill="none""#));
    assert!(html.contains(r#"stroke="red""#));
    assert!(!html.contains(r#"fill="red""#));
    assert!(html.contains(r#"stroke-width="3""#));
    assert!(html.contains(r#"stroke-linecap="butt""#));
    assert!(html.contains(r#"stroke-linejoin="bevel""#));
    assert!(html.contains(r#"class="text-red-500""#));
}

#[test]
fn public_imports_cover_keyword_and_numeric_names() {
    assert!(render_icon(rsx! { Type {} }).contains(r#"viewBox="0 0 24 24""#));
    assert!(render_icon(rsx! { Calendar1 {} }).contains(r#"viewBox="0 0 24 24""#));
}
