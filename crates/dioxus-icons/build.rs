use std::fs;
use std::path::PathBuf;

use lazy_js_bundle::{LazyTypeScriptBindings, MinifyLevel};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let picker_src = manifest_dir.join("picker").join("picker.ts");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR set by cargo"));
    let picker_out = out_dir.join("picker.js");

    // lazy-js-bundle's hash cache (src/js/hash.txt) is shared across every
    // OUT_DIR cargo creates (build, doc, check, ...), but picker.js itself is
    // per-OUT_DIR. When one target writes the hash, the others' picker.js
    // stays stale. Invalidate the cache whenever this OUT_DIR's picker.js is
    // missing or older than picker.ts.
    let stale = match (fs::metadata(&picker_src), fs::metadata(&picker_out)) {
        (Ok(src), Ok(out)) => match (src.modified(), out.modified()) {
            (Ok(src_m), Ok(out_m)) => src_m > out_m,
            _ => true,
        },
        (Ok(_), Err(_)) => true,
        _ => false,
    };
    if stale {
        let _ = fs::remove_file(manifest_dir.join("src").join("js").join("hash.txt"));
        let _ = fs::remove_file(&picker_out);
    }

    LazyTypeScriptBindings::new()
        .with_binding(&picker_src, &picker_out)
        .with_watching(&picker_src)
        .with_minify_level(MinifyLevel::Syntax)
        .run();
}
