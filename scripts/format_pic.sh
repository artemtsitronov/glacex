#!/usr/bin/env bash

set -euo pipefail

INPUT="${1:-screenshots/demo.png}"
OUTPUT="${2:-screenshots/demo.png}"

TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT

if [[ ! -f "$INPUT" ]]; then
    echo "Error: Input file '$INPUT' does not exist." >&2
    exit 1
fi

WIDTH=$(magick identify -format "%w" "$INPUT")
HEIGHT=$(magick identify -format "%h" "$INPUT")

RADIUS=24
PAD_X=88
PAD_TOP=88
PAD_BOTTOM=88

CANVAS_W=$((WIDTH + PAD_X * 2))
CANVAS_H=$((HEIGHT + PAD_TOP + PAD_BOTTOM))

MASK="$TEMP_DIR/mask.png"
ROUNDED="$TEMP_DIR/rounded.png"
FINAL="$TEMP_DIR/final.png"

echo "Processing $INPUT (${WIDTH}x${HEIGHT})..."

magick -size "${WIDTH}x${HEIGHT}" xc:none \
    -fill white \
    -draw "roundrectangle 0,0,$((WIDTH - 1)),$((HEIGHT - 1)),$RADIUS,$RADIUS" \
    png32:"$MASK"

magick "$INPUT" "$MASK" \
    -alpha set \
    -compose DstIn \
    -composite \
    png32:"$ROUNDED"

magick "$ROUNDED" \
    \( +clone -background "rgba(0,0,0,0.38)" -shadow 70x32+0+24 \) \
    \( +clone -background "rgba(0,0,0,0.22)" -shadow 40x12+0+6 \) \
    -reverse \
    -background none \
    -layers merge \
    +repage \
    png32:"$FINAL"

cp "$FINAL" "$OUTPUT"

magick identify "$OUTPUT"

echo "Successfully generated styled screenshot: $OUTPUT"
