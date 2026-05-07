#!/usr/bin/env bash
set -euo pipefail

generated_paths=(
  crates/dioxus-icons/STATIC_PICKER.html
  crates/dioxus-icons/src/generated
)

before="$(mktemp)"
after="$(mktemp)"
trap 'rm -f "$before" "$after"' EXIT

git diff --binary -- "${generated_paths[@]}" > "$before"
cargo run -p dioxus-icons-codegen
git diff --binary -- "${generated_paths[@]}" > "$after"

if ! cmp -s "$before" "$after"; then
  echo "Generated icon output changed while running codegen." >&2
  echo "Run cargo run -p dioxus-icons-codegen and commit the generated output." >&2
  git diff --stat -- "${generated_paths[@]}" >&2
  exit 1
fi
