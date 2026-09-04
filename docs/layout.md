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
    &mut Label::new("Header"),
    &mut row![
        &mut Label::new("Left"),
        &mut Label::new("Right")
    ]
    .spacing(12.0)
    .align(Alignment::Center),
]
.spacing(8.0)
.align(Alignment::Start);

root.arrange_at([20.0, 20.0], ui);
```

## 2. Layout Modifiers

### `.spacing(px: f32)`
Sets the pixel gap between adjacent children along the primary axis. Default is `8.0`.

### `.align(alignment: Alignment)`
Sets child alignment along the cross-axis:
- `Alignment::Start`: Align to top (rows) or left (columns).
- `Alignment::Center`: Center along the cross-axis.

### `.arrange_at(pos: [f32; 2], ui: &mut Ui)`
Measures and positions the layout tree at the given screen coordinates `[x, y]`.

## 3. The `Measurable` Trait

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
