#!/usr/bin/env bash
set -euo pipefail

cargo run -p dioxus-icons-codegen
git diff --exit-code
