#!/usr/bin/env bash
# src_work/*.dat are byte-identical to libcint/src/*.dat and are not tracked in
# git. Restore them from the libcint submodule (run `git submodule update --init`
# first if libcint/ is empty).
set -euo pipefail

here="$(cd "$(dirname "$0")/.." && pwd)"

for f in "$here"/libcint/src/*.dat; do
  name="$(basename "$f")"
  if [ ! -f "$here/src_work/$name" ]; then
    cp "$f" "$here/src_work/$name"
    echo "restored src_work/$name"
  fi
done
