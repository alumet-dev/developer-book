#!/bin/bash

set -eux
set -o pipefail

cd "$(dirname "$0")"

mkdir -p src/resources/diagrams

find diagrams -name '*.drawio' -exec bash -c '
    file=$1
    echo "=== Exporting $file"
    drawio --export -f png --border 2 --scale 2 --output "./src/resources/${file%.*}.png" "$file"
' find-sh {} \;
