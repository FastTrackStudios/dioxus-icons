use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use dioxus_rsx_rosetta::Node;
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

    let source_name = svg_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .context("SVG file stem is not valid UTF-8")?
        .to_owned();
    let svg = compact_svg(&svg);
    let dom = dioxus_rsx_rosetta::Dom::parse(&svg).with_context(|| {
        format!(
            "parsing SVG {} through dioxus-rsx-rosetta",
            svg_path.display()
        )
    })?;

    let root = dom
        .children
        .iter()
        .find_map(|node| match node {
            Node::Element(element) => Some(element),
            _ => None,
        })
        .with_context(|| format!("finding SVG root in {}", svg_path.display()))?;
    let view_box = root
        .attributes
        .get("viewBox")
        .and_then(|value| value.as_deref())
        .unwrap_or_default()
        .to_owned();
    let elements = root
        .children
        .iter()
        .filter_map(|node| match node {
            Node::Element(element) => {
                let attrs = element
                    .attributes
                    .iter()
                    .filter_map(|(name, value)| {
                        value
                            .as_deref()
                            .map(|value| (name.to_owned(), value.to_owned()))
                    })
                    .collect::<BTreeMap<_, _>>();

                Some(SvgElement {
                    tag: element.name.clone(),
                    attrs,
                })
            }
            _ => None,
        })
        .collect();

    Ok(Icon {
        source_name,
        component: names.component,
        module: names.module,
        module_ref: names.module_ref,
        tags: metadata.tags,
        categories: metadata.categories,
        view_box,
        elements,
        svg,
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

fn compact_svg(svg: &str) -> String {
    svg.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
