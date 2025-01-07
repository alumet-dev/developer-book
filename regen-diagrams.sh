#!/bin/bash

set -eu
set -o pipefail

cd "$(dirname "$0")"

OUT_DIR=src/resources/diagrams
OUT_BAK=src/resources/diagrams.bak
export TMP_DIR=/tmp/alumet-developer-book-diagrams
rm -r "$TMP_DIR" || true
mkdir -p "$TMP_DIR/diagrams"

# BEWARE we cannot just loop on the output of find, see https://unix.stackexchange.com/questions/9496/looping-through-files-with-spaces-in-the-names
# Usually, using find -exec bash -c '<script>' is easier, but here we want to run drawio in the background because it takes some time to load.
find diagrams -name '*.drawio' -print0 | {
    while IFS= read -r -d '' file; do
        echo "=== Exporting $file"
        # $file begins with `diagrams` because of find
        out="$TMP_DIR/${file%.*}.png"
        mkdir -p "$(dirname "$out")"
        # export in the background
        drawio --export -f png --border 2 --scale 2 --output "$out" "$file" &
    done
    echo "Waiting for export to complete..."
    wait
}

# Move the temporary directory to the output directory, so that the preview of the book (created by mdbook serve) only updates once
echo "Moving $TMP_DIR/diagrams to $OUT_DIR..."
rm -r "$OUT_BAK" || true
mv "$OUT_DIR" "$OUT_BAK"
mv "$TMP_DIR"/diagrams "$OUT_DIR"

echo "Done!"
