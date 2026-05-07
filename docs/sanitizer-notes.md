# docs.rs sanitizer notes

Status: local rustdoc validation is automated; external docs.rs publication of a
scratch crate still requires crate-owner credentials.

Validated locally with:

```sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p dioxus-icons
```

Findings applied to the implementation:

- Rustdoc rejects self-closing SVG child tags under `rustdoc::invalid-html-tags`.
  Generated component docs use base64 `data:image/svg+xml` image assets for
  hover-friendly previews, while runtime SVG strings use explicit closing tags
  such as `<path></path>`.
- Rustdoc scans JSON script contents for HTML-looking text. The generated
  manifest escapes `<` and `>` as JSON unicode escapes so SVG strings survive
  docs rendering without tripping HTML-tag lints.
- Widget and picker scripts avoid minified comparison fragments that look like
  incomplete HTML tags to rustdoc, such as `i<tags.length`. Runtime SVG previews
  are read from data attributes or the JSON manifest.
- The no-JS floor is the generated static icon index plus per-component default
  image preview and RSX snippet.

Before publishing v1.0, publish a tiny scratch crate to docs.rs that contains:

- one large JSON `<script type="application/json">` manifest,
- one per-item widget,
- one crate-root picker,
- clipboard write with textarea fallback,
- two widget instances on one page.

Record the public docs.rs result in this file before release.
