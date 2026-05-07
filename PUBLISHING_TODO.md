# dioxus-icons Publishing TODO

This checklist is based on the production shape of `../dioxus-code`: polished
crate metadata, docs.rs configuration, runnable examples, release CI, a
published docs/demo surface, and pre-publish verification that exercises both
Rust and browser behavior.

## Blockers

- [ ] Fix the docs picker Playwright suite or update it to the current picker
      markup. Current `npm test` under `tests/playwright` fails all 12 tests:
      the interactive tests wait for `#dxi-picker:not([hidden])`, while the
      current picker markup exposes `.dxi-picker` without that id; the no-JS
      fallback expects `.dioxus-icons-static-grid` to compute as `display:
      grid`, but it currently computes as `block`.
- [ ] Make `cargo clippy --workspace --all-targets -- -D warnings` pass. Current
      failures are in `dioxus-icons-codegen`:
      `override_component_name` trips `clippy::match_single_binding`, and
      `validate_related_file` takes `&PathBuf` where `&Path` is enough.
- [ ] Add the missing published-crate metadata that `dioxus-code` already has:
      `readme`, `homepage`, `keywords`, `categories`, and
      `[package.metadata.docs.rs]` with `rustdoc-args = ["--cfg", "docsrs"]`.
- [ ] Decide and document the first publish version. The working tree currently
      has `workspace.package.version = "0.0.1"` while the previous checked-in
      value was `0.1.0`; do not publish until that is intentional.
- [ ] Validate docs.rs with the real generated docs. Local rustdoc succeeds, but
      `target/doc/dioxus_icons` is about 203 MiB, so docs.rs build time, output
      size, sanitizer behavior, and page load performance need explicit proof.

## Crate Surface

- [ ] Add a crate README for `crates/dioxus-icons` or point `readme` at the
      root `README.md`, matching how `dioxus-code` publishes a full crates.io
      landing page.
- [ ] Expand the README to the `dioxus-code` standard: badges for crates.io,
      downloads, and docs.rs; dependency snippet; import example; props table;
      examples list; generator/update notes; and license notes for MIT plus
      Lucide ISC data.
- [ ] Include the README as crate-level rustdoc after the picker, or split docs
      so the docs.rs root has both the searchable picker and a compact API
      introduction.
- [ ] Add `#![warn(missing_docs)]` to hand-written public API once generated
      modules are either documented well enough or explicitly exempted.
- [ ] Audit the dependency surface against `dioxus-code`: keep Dioxus features
      minimal, confirm why `dioxus-signals` must be a normal dependency, and
      document the supported Dioxus version range.
- [ ] Decide whether any optional features are needed, such as a no-doc-widget
      build or smaller docs mode, before the public API is frozen.

## API Tests

- [ ] Add Rust tests that render representative icons with SSR and assert the
      generated SVG shape: default `size`, `width`, `height`, `fill="none"`,
      `stroke="currentColor"`, and no class attribute when `class` is empty.
- [ ] Add Rust tests for prop mapping: `color` maps to SVG `stroke`, not fill;
      `stroke_width`, `stroke_linecap`, `stroke_linejoin`, and `class` pass
      through correctly.
- [ ] Add generated-name coverage for keyword and numeric cases beyond the
      current codegen naming unit test, including public imports like
      `dioxus_icons::lucide::Type` and `Circle1`.
- [ ] Keep the generated doctests, but add a faster non-doctest smoke test for a
      few high-risk icons so CI catches API regressions without relying only on
      1704 generated doctests.

## Examples And Demos

- [ ] Convert examples to runnable Dioxus examples like `dioxus-code` does.
      Current examples compile, but `main` only stores `App` in a function
      pointer instead of launching a renderer.
- [ ] Consider moving substantial examples into workspace example crates with
      `publish = false`, `desktop` and `web` features, and copyable source
      snippets, matching the `examples/basic`, `examples/live-input`, and
      `examples/editor` pattern in `dioxus-code`.
- [ ] Build a small docsite or choose an explicit alternative homepage.
      `dioxus-code` publishes a Pages site and uses it as `homepage`; this
      crate currently has no comparable homepage target.
- [ ] If a docsite is added, include realistic workflows: icon search,
      prop tweaking, Tailwind class usage, and common app navigation examples.

## CI And Release Automation

- [ ] Bring CI up to the `dioxus-code` bar: fmt, check, test, docs, clippy, and
      package dry-run should all be required on pull requests.
- [ ] Add the Playwright picker suite to CI after building docs:
      `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p dioxus-icons`,
      `npm ci`, then `npm test` in `tests/playwright`.
- [ ] Add a package dry run: `cargo package -p dioxus-icons`.
      Current package output is 1726 files, 20.7 MiB uncompressed, 1.5 MiB
      compressed.
- [ ] Add a nightly workflow if this should track the same early-warning policy
      as `dioxus-code`.
- [ ] Make `ci/verify_codegen.sh` robust for CI and local use. It currently runs
      the generator and then checks the whole git diff, so unrelated dirty files
      make it fail locally.
- [ ] Decide whether to use the reusable `ealmloff/dioxus-ci` workflows, as
      `dioxus-code` does, or keep local workflow definitions with equivalent
      coverage.
- [ ] Add release instructions for ordering: update Lucide pin, regenerate
      related sidecar if needed, run codegen, inspect generated diff, run full
      CI locally, package, publish.

## Codegen And Generated Data

- [ ] Remove `scripts/__pycache__` from the codegen crate tree and add an ignore
      rule so it cannot be packaged or committed again.
- [ ] Pin the Python sidecar environment with a `requirements.txt`, `uv.lock`,
      or equivalent so related-icon regeneration is reproducible.
- [ ] Store or validate sidecar provenance: Lucide version, Lucide commit, model
      name, input icon count, and output schema version.
- [ ] Add tests for the generated manifest JSON: valid JSON, expected icon
      count, stable sort order, escaped SVG payloads, tags, categories, and
      docs links.
- [ ] Add a small fixture-based codegen test so parser, naming, manifest, docs,
      and component emission can be tested without fetching the full Lucide
      archive.
- [ ] Keep generated output committed, but document that generated files under
      `crates/dioxus-icons/src/generated/` are never edited by hand.

## Docs.rs Picker Quality

- [ ] Fix the selector contract between `picker.html`, `picker.ts`, generated
      static picker HTML, and Playwright tests.
- [ ] Test with JavaScript enabled and disabled on the generated rustdoc HTML.
- [ ] Test keyboard navigation, search ranking, clear behavior, empty state,
      related-icon links, and per-icon widget copy behavior.
- [ ] Verify the picker and per-icon widgets in docs.rs-like sanitizer
      conditions, then record the public result in `docs/sanitizer-notes.md`.
- [ ] Measure docs page size and interaction latency after generation; adjust
      docs payloads if per-icon base64 previews make docs.rs too heavy.

## Pre-Publish Gate

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p dioxus-icons`
- [ ] `cargo package -p dioxus-icons`
- [ ] `npm ci && npm test` in `tests/playwright`
- [ ] `ci/verify_codegen.sh` from a clean tree
- [ ] Manual review of `Cargo.toml`, generated diff, README, docs.rs output,
      package file list, and license notices
