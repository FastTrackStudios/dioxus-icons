use std::fs;
use std::path::PathBuf;

use lazy_js_bundle::{LazyTypeScriptBindings, MinifyLevel};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let picker_src = manifest_dir.join("picker").join("picker.ts");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR set by cargo"));
    let picker_out = out_dir.join("picker.js");

    // lazy-js-bundle skips work when its hash cache (src/js/hash.txt) matches
    // the watched inputs. After `cargo clean` wipes OUT_DIR the cache lies, so
    // invalidate it whenever the output is missing.
    if !picker_out.exists() {
        let _ = fs::remove_file(manifest_dir.join("src").join("js").join("hash.txt"));
    }

    LazyTypeScriptBindings::new()
        .with_binding(&picker_src, &picker_out)
        .with_watching(&picker_src)
        .with_minify_level(MinifyLevel::Syntax)
        .run();
}
