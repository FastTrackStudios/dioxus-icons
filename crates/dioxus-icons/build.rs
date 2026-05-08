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
}
