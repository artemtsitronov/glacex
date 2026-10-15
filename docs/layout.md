# Glacex Layout

## 1. Layout Macros

Glacex has two macros for layout
- `row![...]` puts widgets next to each other from left to right
- `column![...]` stacks widgets from top to bottom

### Example

```rust
use glacex::{Alignment, Label, Ui, Widget, column, row};

struct LayoutExample;

impl Widget for LayoutExample {
    type Output = ()
    fn ui(&mut self, ui: &mut Ui) {
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
    }
}
```

## 2. Layout Modifiers

### `.spacing(px: f32)`
The gap in pixels between each child. The default is `8.0`.

### `.align(alignment: Alignment)`
Lines up the children on the other axis
- `Alignment::Start` puts them at the top in a row, or on the left in a column
- `Alignment::Center` puts them in the middle
- `Alignment::End` puts them at the bottom in a row, or on the right in a column

By default a `row!` or `column!` is exactly as big as its content, so there's no extra space and `.align()` won't look like it does anything. You'll only see it work once the container is bigger than what's inside, like when you use `.size()` from below.

### `.size([w, h]: [f32; 2])` / `.width(px: f32)` / `.height(px: f32)`
Sets the width or height of the row or column yourself, instead of it just fitting the content. `.size()` sets both at once. This goes well with `.align()`. For example if you give a `row!` a `.size()` that's taller than its content, you can center the children vertically in the extra space.

### `.padding([x, y]: [f32; 2])`
Adds space inside the row or column. `x` pixels go on the left and right, and `y` pixels on the top and bottom. If you didn't set a `.size()`, the padding gets added on top of the content size, so the children don't get squished.

### `.arrange_at(pos: [f32; 2], ui: &mut Ui)`
Measures everything and places it at the screen position `[x, y]` you give it.

## 3. The `Measurable` Trait

Any widget you put in `row![]` or `column![]` implements `Measurable`

```rust
pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;
}
```

Here's what happens
1. **Measuring**. The container calls `measure` on each child to see how big it wants to be.
2. **Layout**. `taffy` does the flexbox math and works out the sizes and spacing.
3. **Arranging**. The container calls `arrange` on each child with its final position and size, and the child draws itself there.
