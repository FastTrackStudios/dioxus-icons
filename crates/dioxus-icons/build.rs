use std::fs;
use std::path::PathBuf;

use lazy_js_bundle::{LazyTypeScriptBindings, MinifyLevel};

fn main() {
    // lazy-js-bundle stores its source hash in ./src/js/hash.txt. Keep the
    // generated JS beside that hash so clean CI/docs.rs builds can use the
    // checked-in bundle without requiring Bun.
    LazyTypeScriptBindings::new()
        .with_watching("./picker")
        .with_binding("./picker/picker.ts", "./src/js/picker.js")
        .with_minify_level(MinifyLevel::Syntax)
        .run();

    // Split the workspace README at its `---` divider so the docs.rs landing
    // can render: header (title + badges) → icon picker → body. The picker
    // replaces the divider's visual role.
    //
    // The README lives at the WORKSPACE root (`readme = "../../README.md"`),
    // which cargo does NOT copy into the vendored package tarball. So in a
    // vendored/sandboxed build (Nix `dockerTools`, docs.rs, `cargo vendor`)
    // neither path resolves. That's fine — the split only feeds the docs.rs
    // landing page; the crate itself needs nothing from it. Degrade to empty
    // header/body instead of panicking the whole build.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let readme = [
        manifest_dir.join("../../README.md"),
        manifest_dir.join("README.md"),
    ]
    .into_iter()
    .find_map(|path| path.canonicalize().ok());

    let full = match &readme {
        Some(path) => {
            println!("cargo:rerun-if-changed={}", path.display());
            fs::read_to_string(path).unwrap_or_default()
        }
        None => String::new(),
    };
    let (header, body) = full
        .split_once("\n---\n")
        .map(|(h, b)| (h.trim_end(), b.trim_start()))
        .unwrap_or((full.as_str(), ""));

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("README_HEADER.md"), header).expect("write README_HEADER.md");
    fs::write(out_dir.join("README_BODY.md"), body).expect("write README_BODY.md");
}
