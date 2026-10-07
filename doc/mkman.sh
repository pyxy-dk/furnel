#!/usr/bin/env bash
# Generate doc/furnel.1 from the --help output of a built furnel binary.
# Usage: doc/mkman.sh [path/to/furnel]   (default: target/release/furnel)
set -euo pipefail

dir="$(dirname "$0")"
bin="${1:-target/release/furnel}"

LC_ALL=C.UTF-8 help2man \
  --no-info \
  --locale=C.UTF-8 \
  --section=1 \
  --name="compress files using the brotli algorithm" \
  --include="$dir/furnel.h2m" \
  --output="$dir/furnel.1" \
  "$bin"
