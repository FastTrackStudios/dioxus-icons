use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::fetch::LUCIDE_VERSION;
use crate::parse::Icon;

const RELATED_FILE: &str = "data/lucide-related-siglip2-base-patch16-224.json";

#[derive(Debug)]
pub struct RelatedIcons {
    by_source_name: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RelatedFile {
    model: String,
    icon_set: String,
    icon_set_version: String,
    related_count: usize,
    items: Vec<RelatedItem>,
}

#[derive(Debug, Deserialize)]
struct RelatedItem {
    name: String,
    related: Vec<String>,
}

impl RelatedIcons {
    pub fn load_lucide(icons: &[Icon]) -> Result<Self> {
        let path = related_file_path();
        let contents =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let file: RelatedFile = serde_json::from_str(&contents)
            .with_context(|| format!("parsing {}", path.display()))?;

        validate_related_file(&file, icons, &path)?;

        Ok(Self {
            by_source_name: file
                .items
                .into_iter()
                .map(|item| (item.name, item.related))
                .collect(),
        })
    }

    pub fn for_icon<'a>(
        &self,
        icon: &Icon,
        icons_by_source_name: &'a BTreeMap<&str, &'a Icon>,
    ) -> Result<Vec<&'a Icon>> {
        let related_names = self
            .by_source_name
            .get(&icon.source_name)
            .with_context(|| format!("missing related icons for `{}`", icon.source_name))?;

        related_names
            .iter()
            .map(|name| {
                icons_by_source_name
                    .get(name.as_str())
                    .copied()
                    .with_context(|| format!("unknown related icon `{name}`"))
            })
            .collect()
    }
}

fn validate_related_file(file: &RelatedFile, icons: &[Icon], path: &PathBuf) -> Result<()> {
    if file.icon_set != "lucide" {
        bail!(
            "{} was generated for icon set `{}`, expected `lucide`",
            path.display(),
            file.icon_set
        );
    }

    if file.icon_set_version != LUCIDE_VERSION {
        bail!(
            "{} was generated for Lucide v{}, expected v{}",
            path.display(),
            file.icon_set_version,
            LUCIDE_VERSION
        );
    }

    if !file.model.starts_with("google/siglip2-") {
        bail!(
            "{} was generated with `{}`, expected a google/siglip2 model",
            path.display(),
            file.model
        );
    }

    let source_names = icons
        .iter()
        .map(|icon| icon.source_name.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();

    if file.items.len() != icons.len() {
        bail!(
            "{} contains {} related entries, expected {}",
            path.display(),
            file.items.len(),
            icons.len()
        );
    }

    for item in &file.items {
        if !source_names.contains(item.name.as_str()) {
            bail!(
                "{} contains related entry for unknown icon `{}`",
                path.display(),
                item.name
            );
        }
        if !seen.insert(item.name.as_str()) {
            bail!(
                "{} contains duplicate related entry for `{}`",
                path.display(),
                item.name
            );
        }
        if item.related.len() != file.related_count {
            bail!(
                "{} entry `{}` contains {} related icons, expected {}",
                path.display(),
                item.name,
                item.related.len(),
                file.related_count
            );
        }

        let mut related_seen = BTreeSet::new();
        for related in &item.related {
            if related == &item.name {
                bail!("{} entry `{}` links to itself", path.display(), item.name);
            }
            if !source_names.contains(related.as_str()) {
                bail!(
                    "{} entry `{}` links to unknown icon `{}`",
                    path.display(),
                    item.name,
                    related
                );
            }
            if !related_seen.insert(related.as_str()) {
                bail!(
                    "{} entry `{}` links to `{}` more than once",
                    path.display(),
                    item.name,
                    related
                );
            }
        }
    }

    Ok(())
}

fn related_file_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(RELATED_FILE)
}
