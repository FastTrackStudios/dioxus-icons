# dioxus-icons Publishing TODO

This checklist is based on the production shape of `../dioxus-code`: polished
crate metadata, docs.rs configuration, runnable examples, release CI, a
published docs/demo surface, and pre-publish verification that exercises both
Rust and browser behavior.

## Blockers

- [x] Fix the docs picker Playwright suite or update it to the current picker
      markup. The picker keeps the legacy selector contract and the no-JS
      fallback emits CSS so `.dioxus-icons-static-grid` computes as a grid.
- [x] Make `cargo clippy --workspace --all-targets -- -D warnings` pass.
      `dioxus-icons-codegen` now avoids the single-binding match and takes
      `&Path` for related-file validation.
- [x] Add the missing published-crate metadata that `dioxus-code` already has:
      `readme`, `homepage`, `keywords`, `categories`, and
      `[package.metadata.docs.rs]` with `rustdoc-args = ["--cfg", "docsrs"]`.
- [x] Decide and document the first publish version. The working tree has
      `workspace.package.version = "0.0.1"` as the first publish candidate.
- [x] Validate docs.rs with the real generated docs. Local rustdoc,
      docs-output validation, Playwright, and a packaged `.crate` docs.rs-style
      build pass; public docs.rs URL verification happens after publish.

## Crate Surface

- [x] Add a crate README for `crates/dioxus-icons` or point `readme` at the
      root `README.md`, matching how `dioxus-code` publishes a full crates.io
      landing page.
- [x] Expand the README to the `dioxus-code` standard: badges for crates.io,
      downloads, and docs.rs; dependency snippet; import example; props table;
      examples list; generator/update notes; and license notes for MIT plus
      Lucide ISC data.
- [x] Include the README as crate-level rustdoc after the picker, or split docs
      so the docs.rs root has both the searchable picker and a compact API
      introduction.
- [x] Add `#![warn(missing_docs)]` to hand-written public API once generated
      modules are either documented well enough or explicitly exempted.
- [x] Audit the dependency surface against `dioxus-code`: keep Dioxus features
      minimal, confirm why `dioxus-signals` must be a normal dependency, and
      document the supported Dioxus version range.
- [x] Decide whether any optional features are needed, such as a no-doc-widget
      build or smaller docs mode, before the public API is frozen.

## API Tests

- [x] Add Rust tests that render representative icons with SSR and assert the
      generated SVG shape: default `size`, `width`, `height`, `fill="none"`,
      `stroke="currentColor"`, and no class attribute when `class` is empty.
- [x] Add Rust tests for prop mapping: `color` maps to SVG `stroke`, not fill;
      `stroke_width`, `stroke_linecap`, `stroke_linejoin`, and `class` pass
      through correctly.
- [x] Add generated-name coverage for keyword and numeric cases beyond the
      current codegen naming unit test, including public imports like
      `dioxus_icons::lucide::Type` and a numeric public import.
- [x] Keep the generated doctests, but add a faster non-doctest smoke test for a
      few high-risk icons so CI catches API regressions without relying only on
      generated doctests.

## Examples And Demos

- [x] Convert examples to standard Dioxus examples like `dioxus-code` does.
      The examples now use the standard `dioxus::launch(App)` entrypoint.
- [x] Consider moving substantial examples into workspace example crates with
      `publish = false`, `desktop` and `web` features, and copyable source
      snippets, matching the `examples/basic`, `examples/live-input`, and
      `examples/editor` pattern in `dioxus-code`. Decision for the first
      publish: keep small in-package examples and defer larger renderer demos.
- [x] Build a small docsite or choose an explicit alternative homepage.
      This crate uses the repository homepage and the docs.rs picker as the
      initial published surface instead of adding a separate Pages site.
- [x] If a docsite is added, include realistic workflows: icon search,
      prop tweaking, Tailwind class usage, and common app navigation examples.
      No separate docsite is being added for the first publish; the docs.rs
      picker plus runnable examples cover these workflows.

## CI And Release Automation

- [x] Bring CI up to the `dioxus-code` bar: fmt, check, test, docs, clippy, and
      package dry-run should all be required on pull requests.
- [x] Add the Playwright picker suite to CI after building docs:
      `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p dioxus-icons`,
      `npm ci`, then `npm test` in `tests/playwright`.
- [x] Add a package dry run: `cargo package -p dioxus-icons`.
      Current package output is 1727 files, 20.8 MiB uncompressed, 1.5 MiB
      compressed.
- [x] Add a nightly workflow if this should track the same early-warning policy
      as `dioxus-code`.
- [x] Make `ci/verify_codegen.sh` robust for CI and local use. It runs the
      generator and checks whether the generated diff changed during the run, so
      unrelated dirty files do not make it fail locally.
- [x] Decide whether to use the reusable `ealmloff/dioxus-ci` workflows, as
      `dioxus-code` does, or keep local workflow definitions with equivalent
      coverage.
- [x] Add release instructions for ordering: update Lucide pin, regenerate
      related sidecar if needed, run codegen, inspect generated diff, run full
      CI locally, package, publish.

## Codegen And Generated Data

- [x] Remove `scripts/__pycache__` from the codegen crate tree and add an ignore
      rule so it cannot be packaged or committed again.
- [x] Pin the Python sidecar environment with a `requirements.txt`, `uv.lock`,
      or equivalent so related-icon regeneration is reproducible.
- [x] Store or validate sidecar provenance: Lucide version, Lucide commit, model
      name, input icon count, and output schema version.
- [x] Add tests for the generated manifest JSON: valid JSON, expected icon
      count, stable sort order, escaped SVG payloads, tags, categories, and
      docs links.
- [x] Add a small fixture-based codegen test so parser, naming, manifest, docs,
      and component emission can be tested without fetching the full Lucide
      archive.
- [x] Keep generated output committed, but document that generated files under
      `crates/dioxus-icons/src/generated/` are never edited by hand.

## Docs.rs Picker Quality

- [x] Fix the selector contract between `picker.html`, `picker.ts`, generated
      static picker HTML, and Playwright tests.
- [x] Test with JavaScript enabled and disabled on the generated rustdoc HTML.
- [x] Test keyboard navigation, search ranking, clear behavior, empty state,
      related-icon links, and per-icon widget copy behavior.
- [x] Verify the picker and per-icon widgets in docs.rs-like sanitizer
      conditions.
- [x] Measure docs page size and interaction latency after generation; adjust
      docs payloads if per-icon base64 previews make docs.rs too heavy.

## Pre-Publish Gate

- [x] `cargo fmt --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p dioxus-icons`
- [x] `cargo package -p dioxus-icons`
      Local verification passes with `--allow-dirty` while this branch is
      uncommitted; rerun the exact command from the clean publish commit.
- [x] `npm ci && npm test` in `tests/playwright`
- [x] `ci/verify_codegen.sh` from a clean tree
      The script passes in this dirty branch and detects generator-introduced
      changes; rerun from the clean publish commit.
- [x] `ci/validate_docsrs.sh`
      Packaged `.crate` docs.rs-style validation passes with `ALLOW_DIRTY=1`
      while this branch is uncommitted; rerun from the clean publish commit.
- [x] Manual review of `Cargo.toml`, generated diff, README, docs.rs output,
      package file list, and license notices
