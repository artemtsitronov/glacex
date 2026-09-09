# Glacex Layout Guide

This guide describes layout, measurement, and positioning in `glacex`.

## 1. Layout Macros

Glacex provides two declarative macros for layout:
- `row![...]`: Lays out child widgets horizontally from left to right.
- `column![...]`: Lays out child widgets vertically from top to bottom.

### Example

```rust
use glacex::{Alignment, Label, column, row};

let mut root = column![
    &mut Label::new("header", "Header"),
    &mut row![
        &mut Label::new("left", "Left"),
        &mut Label::new("right", "Right")
    ]
    .spacing(12.0)
    .align(Alignment::Center),
]
.spacing(8.0)
.align(Alignment::Start);

root.arrange_at([20.0, 20.0], ui);
```

## 2. Default sizing and centering

Rows and columns naturally hug their measured content. When a `Row`, `Column`, or
`Container` is used directly as a widget with `.ui(ui)`, it measures itself and
centers itself in the window. This makes small standalone compositions usable
without manual coordinates.

Use `arrange_at([x, y], ui)` when the parent owns positioning, such as a custom
dashboard, overlay, scroll region, or anchored panel. Explicit arrangement always
takes precedence over the centered direct-use default.

## 3. Layout Modifiers

### `.spacing(px: f32)`
Sets the pixel gap between adjacent children along the primary axis. Default is `8.0`.

### `.align(alignment: Alignment)`
Sets child alignment along the cross-axis:
- `Alignment::Start`: Align to top (rows) or left (columns).
- `Alignment::Center`: Center along the cross-axis.
- `Alignment::End`: Align to bottom (rows) or right (columns).

By default a `row!`/`column!` hugs its content, so there is no extra cross-axis
space for `.align()` to use until the container receives an explicit larger size.
The default alignment is `Alignment::Center`.

For application headers, keep branding, window metadata, and primary actions in
one centered row with a small consistent gap. Use a separate centered row for
navigation tabs and context actions so the heading bar does not compete with the
content grid.

### `.size([w, h]: [f32; 2])` / `.width(px: f32)` / `.height(px: f32)`
Overrides the row/column's own width and/or height instead of hugging its
content. `.size()` is shorthand for calling both. Useful together with
`.align()` — a `row!` given a taller `.size()` than its content, for
instance, can then vertically center its children in that extra space.

### `.padding([x, y]: [f32; 2])`
Insets children from the row/column's own bounds by `x` pixels on the left
and right, and `y` pixels on the top and bottom. When no explicit `.size()`
is set, padding is added on top of the hugged content size (so the children
themselves aren't squeezed).

### `.arrange_at(pos: [f32; 2], ui: &mut Ui)`
Measures and positions the layout tree at the given screen coordinates `[x, y]`.

## 4. The `Measurable` Trait

Widgets placed inside `row![]` or `column![]` implement `Measurable`:

```rust
pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;
}
```

1. **Measurement**: The container calls `measure` on children to determine natural dimensions.
2. **Layout**: `taffy` resolves flexbox constraints, sizes, and spacing.
3. **Arrangement**: The container calls `arrange` on children with resolved position and size, rendering them in place.
