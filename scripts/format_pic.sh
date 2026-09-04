#!/usr/bin/env bash
set -euo pipefail

# Styles screenshots with Apple / Linear / Vercel grade visual treatment:
# - Generous smooth squircle-like rounded corners with 4x supersampled anti-aliasing
# - Pure 100% transparent canvas with natural Gaussian falloff
# - Triple-layer ambient + key + diffuse volumetric soft shadow (silky smooth, zero harshness)
# - Ultra-subtle hairline rim lighting that blends seamlessly

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

echo "Processing $INPUT (${WIDTH}x${HEIGHT}) -> Silky smooth Apple/Vercel tier aesthetic..."

# 1. High-fidelity 4x supersampled mask for buttery smooth rounded corners
SS_SCALE=4
SS_W=$((WIDTH * SS_SCALE))
SS_H=$((HEIGHT * SS_SCALE))
SS_R=$((RADIUS * SS_SCALE))

MASK_SS="$TEMP_DIR/mask_ss.png"
MASK="$TEMP_DIR/mask.png"

magick -size "${SS_W}x${SS_H}" xc:none \
    -fill white \
    -draw "roundrectangle 0,0,$((SS_W - 1)),$((SS_H - 1)),$SS_R,$SS_R" \
    "$MASK_SS"

magick "$MASK_SS" -filter Lanczos -resize "${WIDTH}x${HEIGHT}" "$MASK"

# 2. Cut rounded corners with silky smooth edges
ROUNDED="$TEMP_DIR/rounded.png"
magick "$INPUT" "$MASK" \
    -alpha set -compose DstIn -composite \
    "$ROUNDED"

# 3. Soft translucent hairline rim light (subtle top highlight, feathering into edges)
STROKED="$TEMP_DIR/stroked.png"
magick "$ROUNDED" \
    \( -size "${WIDTH}x${HEIGHT}" xc:none \
       -stroke "rgba(255,255,255,0.09)" -strokewidth 1 -fill none \
       -draw "roundrectangle 0.5,0.5,$((WIDTH - 1)).5,$((HEIGHT - 1)).5,$RADIUS,$RADIUS" \) \
    -compose Over -composite \
    "$STROKED"

# 4. Triple-layered volumetric soft drop shadow on transparent canvas:
# Layer A: Ultra-diffuse wide ambient glow (very large blur, very low opacity)
# Layer B: Medium body shadow (soft spread, gentle downward drift)
# Layer C: Close contact shadow (grounds the window, tight blur)
PAD_X=110
PAD_TOP=70
PAD_BOTTOM=110
CANVAS_W=$((WIDTH + PAD_X * 2))
CANVAS_H=$((HEIGHT + PAD_TOP + PAD_BOTTOM))

SHADOW_BASE="$TEMP_DIR/shadow_base.png"
magick "$MASK" -fill black -opaque white "$SHADOW_BASE"

# Layer A: Wide ambient atmosphere
SHADOW_A="$TEMP_DIR/shadow_a.png"
magick "$SHADOW_BASE" \( +clone -background "black" -shadow 25x38+0+12 \) -delete 0 "$SHADOW_A"

# Layer B: Medium volumetric key shadow
SHADOW_B="$TEMP_DIR/shadow_b.png"
magick "$SHADOW_BASE" \( +clone -background "black" -shadow 35x20+0+24 \) -delete 0 "$SHADOW_B"

# Layer C: Soft contact shadow
SHADOW_C="$TEMP_DIR/shadow_c.png"
magick "$SHADOW_BASE" \( +clone -background "black" -shadow 20x8+0+6 \) -delete 0 "$SHADOW_C"

FINAL="$TEMP_DIR/final.png"

# Composite all layers onto pure transparent background
magick -size "${CANVAS_W}x${CANVAS_H}" xc:none \
    "$SHADOW_A" -geometry "+${PAD_X}+${PAD_TOP}" -composite \
    "$SHADOW_B" -geometry "+${PAD_X}+${PAD_TOP}" -composite \
    "$SHADOW_C" -geometry "+${PAD_X}+${PAD_TOP}" -composite \
    "$STROKED"  -geometry "+${PAD_X}+${PAD_TOP}" -composite \
    "$FINAL"

cp "$FINAL" "$OUTPUT"
echo "Successfully generated silky smooth screenshot: $OUTPUT"
magick identify "$OUTPUT"
