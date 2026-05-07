#!/usr/bin/env bash
set -euo pipefail

package_args=(-p dioxus-icons)
if [ "${ALLOW_DIRTY:-}" = "1" ]; then
  package_args+=(--allow-dirty)
fi

cargo package "${package_args[@]}"

crate_file="$(find target/package -maxdepth 1 -name 'dioxus-icons-[0-9]*.crate' | sort | tail -n 1)"
if [ -z "$crate_file" ]; then
  echo "Could not find packaged dioxus-icons crate under target/package" >&2
  exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

tar -xzf "$crate_file" -C "$tmpdir"
crate_dir="$tmpdir/$(basename "$crate_file" .crate)"

(
  cd "$crate_dir"
  DOCS_RS=1 RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --no-deps -p dioxus-icons

  doc_dir="target/doc/dioxus_icons"
  index_html="$doc_dir/index.html"
  icon_pages="$(find "$doc_dir/lucide" -maxdepth 1 -name 'fn.*.html' | wc -l | tr -d ' ')"
  doc_size="$(du -sh "$doc_dir" | cut -f1)"
  index_size="$(wc -c < "$index_html" | tr -d ' ')"

  echo "docsrs_package=$crate_file"
  echo "docsrs_doc_dir=$doc_dir"
  echo "docsrs_doc_size=$doc_size"
  echo "docsrs_index_bytes=$index_size"
  echo "docsrs_icon_pages=$icon_pages"
)
