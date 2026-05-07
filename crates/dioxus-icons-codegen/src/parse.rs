use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::naming::IconNames;

#[derive(Debug, Clone)]
pub struct Icon {
    pub source_name: String,
    pub component: String,
    pub module: String,
    pub module_ref: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub view_box: String,
    pub elements: Vec<SvgElement>,
    pub svg: String,
}

#[derive(Debug, Clone)]
pub struct SvgElement {
    pub tag: String,
    pub attrs: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct IconMetadata {
    tags: Vec<String>,
    categories: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Manifest {
    pub version: String,
    pub source: String,
    pub icons: Vec<ManifestIcon>,
}

#[derive(Debug, Serialize)]
pub struct ManifestIcon {
    pub name: String,
    pub module: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub svg: String,
}

pub fn parse_icon(svg_path: &Path, json_path: &Path, names: IconNames) -> Result<Icon> {
    let svg = fs::read_to_string(svg_path)
        .with_context(|| format!("reading SVG {}", svg_path.display()))?;
    let metadata = fs::read_to_string(json_path)
        .with_context(|| format!("reading metadata {}", json_path.display()))?;
    let metadata: IconMetadata = serde_json::from_str(&metadata)
        .with_context(|| format!("parsing metadata {}", json_path.display()))?;

    let document = roxmltree::Document::parse(&svg)
        .with_context(|| format!("parsing SVG {}", svg_path.display()))?;
    let root = document.root_element();

    if root.tag_name().name() != "svg" {
        bail!("{} root element is not <svg>", svg_path.display());
    }

    validate_root(svg_path, &root)?;

    let mut elements = Vec::new();
    for child in root.children().filter(roxmltree::Node::is_element) {
        let tag = child.tag_name().name().to_owned();
        validate_child_tag(svg_path, &tag)?;

        if child.children().any(|node| node.is_element()) {
            bail!(
                "{} contains nested SVG element <{}>; update the IR before generating it",
                svg_path.display(),
                tag
            );
        }

        let mut attrs = BTreeMap::new();
        for attr in child.attributes() {
            attrs.insert(attr.name().to_owned(), attr.value().to_owned());
        }

        elements.push(SvgElement { tag, attrs });
    }

    let source_name = svg_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .context("SVG file stem is not valid UTF-8")?
        .to_owned();

    Ok(Icon {
        source_name,
        component: names.component,
        module: names.module,
        module_ref: names.module_ref,
        tags: metadata.tags,
        categories: metadata.categories,
        view_box: root.attribute("viewBox").unwrap_or_default().to_owned(),
        elements,
        svg: compact_svg(&svg),
    })
}

pub fn manifest(version: &str, icons: &[Icon]) -> Manifest {
    Manifest {
        version: version.to_owned(),
        source: "lucide".to_owned(),
        icons: icons
            .iter()
            .map(|icon| ManifestIcon {
                name: icon.component.clone(),
                module: icon.module.clone(),
                tags: icon.tags.clone(),
                categories: icon.categories.clone(),
                svg: icon.svg.clone(),
            })
            .collect(),
    }
}

fn validate_root(svg_path: &Path, root: &roxmltree::Node<'_, '_>) -> Result<()> {
    let namespace = root.tag_name().namespace().unwrap_or_default();
    if namespace != "http://www.w3.org/2000/svg" {
        bail!(
            "{} has unexpected SVG namespace: expected {:?}, got {:?}",
            svg_path.display(),
            "http://www.w3.org/2000/svg",
            namespace
        );
    }

    let expected = [
        ("width", "24"),
        ("height", "24"),
        ("viewBox", "0 0 24 24"),
        ("fill", "none"),
        ("stroke", "currentColor"),
        ("stroke-width", "2"),
        ("stroke-linecap", "round"),
        ("stroke-linejoin", "round"),
    ];

    for (attr, expected_value) in expected {
        let actual = root.attribute(attr).unwrap_or_default();
        if actual != expected_value {
            bail!(
                "{} has unexpected root {attr:?}: expected {expected_value:?}, got {actual:?}",
                svg_path.display()
            );
        }
    }

    Ok(())
}

fn validate_child_tag(svg_path: &Path, tag: &str) -> Result<()> {
    match tag {
        "circle" | "ellipse" | "line" | "path" | "polygon" | "polyline" | "rect" => Ok(()),
        _ => bail!(
            "{} contains unsupported SVG element <{}>",
            svg_path.display(),
            tag
        ),
    }
}

fn compact_svg(svg: &str) -> String {
    svg.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
