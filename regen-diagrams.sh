#!/bin/bash

set -eu
set -o pipefail

cd "$(dirname "$0")"

OUT_DIR=src/resources/diagrams
OUT_BAK=src/resources/diagrams.bak
export TMP_DIR=/tmp/alumet-developer-book-diagrams
rm -r "$TMP_DIR" || true
mkdir -p "$TMP_DIR/diagrams"

find diagrams -name '*.drawio' -exec bash -c '
    set -eu
    set -o pipefail
    file=$1
    echo "=== Exporting $file"
    out="$TMP_DIR/${file%.*}.png"
    mkdir -p "$(dirname "$out")"
    drawio --export -f png --border 2 --scale 2 --output "$out" "$file"
    # the file path begins with `diagrams`
' find-sh {} \;

rm -r "$OUT_BAK" || true
mv "$OUT_DIR" "$OUT_BAK"
mv "$TMP_DIR"/diagrams "$OUT_DIR"
echo "Done!"
