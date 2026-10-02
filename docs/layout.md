# layout guide

layout in glacex is flexbox with the scary parts removed. two macros, five modifiers, one finishing call. this page is all of it.

## the two macros

```rust
column![&mut a, &mut b, &mut c]  // stack vertically, top to bottom
row![&mut a, &mut b]             // line up horizontally, left to right
```

that's the whole vocabulary. everything else is modifiers. a full example:

```rust
use glacex::{Alignment, Label, Ui, Widget, column, row};

struct LayoutExample;

impl Widget for LayoutExample {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let mut header = Label::new("Header").heading();
        let mut left = Label::new("Left");
        let mut right = Label::new("Right");

        let mut middle = row![&mut left, &mut right]
            .spacing(12.0)
            .align(Alignment::Center);

        let mut root = column![&mut header, &mut middle]
            .spacing(8.0)
            .align(Alignment::Start);

        root.arrange_at([20.0, 20.0], ui);
    }
}
```

note the types: `column!`/`row!` take `&mut` references to your widgets. the macro just collects them into a `Column`/`Row` struct. those structs also have `Column::new(vec![...])` if you'd rather build the vec yourself (see [custom widgets](custom-widgets.md) -- dynamic lists do exactly this).

## the five modifiers

### `.spacing(px)`

gap between children along the main axis. default `8.0`. the only modifier with a non-zero default, because widgets touching each other looks like a subway at rush hour.

```rust
column![&mut a, &mut b].spacing(16.0)
```

### `.align(alignment)`

how children sit along the **cross** axis (perpendicular to stacking): `Alignment::Start`, `Alignment::Center` (default), `Alignment::End`.

```rust
row![&mut a, &mut b].align(Alignment::Center)  // vertically centered in the row
```

gotcha worth knowing: a `row!`/`column!` **hugs its content** by default -- it's exactly as big as its children. with no extra room, alignment has nothing to do and quietly does nothing. it only becomes visible once the container is bigger than its kids, via `.size()` below:

```rust
// center three buttons in a 600px-wide band:
row![&mut b1, &mut b2, &mut b3]
    .width(600.0)
    .align(Alignment::Center)
```

### `.size([w, h])` / `.width(px)` / `.height(px)`

override the container's own dimensions instead of hugging content. `.size()` sets both. mainly used to (a) give `.align()` room to work, or (b) claim a fixed region of the window:

```rust
column![&mut content]
    .size([800.0, 600.0])   // i live here now
    .align(Alignment::Start)
```

### `.padding([x, y])`

insets children from the container's edges: `x` pixels left/right, `y` pixels top/bottom. without an explicit `.size()`, padding is *added* around the hugged content (children aren't squeezed); with `.size()`, children share what's left.

```rust
column![&mut form].padding([24.0, 24.0])  // ah. breathing room.
```

### `.arrange_at([x, y], ui)` -- the finishing call

measures the whole tree, computes flexbox positions via `taffy`, and draws everything at `[x, y]`. **a layout that never gets `arrange_at` draws nothing.** it's the difference between planning a party and having one.

```rust
root.arrange_at([20.0, 20.0], ui);
```

`Container` and `ScrollView` have their own `.arrange_at()` too. `Card` doesn't need one (it goes inside a `row!`/`column!` like everything else).

## nesting (the actual skill)

real uis are layouts inside layouts. the pattern:

```rust
let mut name_row = row![&mut name_label, &mut name_input].spacing(8.0);
let mut pass_row = row![&mut pass_label, &mut pass_input].spacing(8.0);
let mut buttons  = row![&mut ok, &mut cancel].spacing(8.0);

let mut form = column![&mut name_row, &mut pass_row, &mut buttons]
    .spacing(12.0)
    .align(Alignment::Start);

let mut card = Card::new(&mut form).padding([24.0, 24.0]);

column![&mut card].arrange_at([40.0, 40.0], ui);
```

rows of inputs, stacked in a column, wrapped in a card, placed on screen. read it inside-out: the smallest groups first, the screen position last. every example in `examples/` does this; [`reminder.rs`](../examples/reminder.rs) is the clearest specimen.

## full-window layouts

need something centered in (or filling) the window? ask for the size and do the arithmetic -- there's no magic "center" flag, just numbers:

```rust
let size = ui.window_size();               // [width, height] of the window
column![&mut card]
    .size(size)                            // claim the whole window...
    .align(Alignment::Center)              // ...then center the card inside it
    .arrange_at([0.0, 0.0], ui);
```

(`examples/reminder.rs` centers its todo app exactly this way.)

## how it works (60-second version)

every layout child implements `Measurable`:

```rust
pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];                          // "how big are you?"
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;  // "draw yourself HERE"
}
```

per frame: the container calls `measure` on each child (exactly once -- results are cached), hands the sizes to `taffy`, gets positions back, then calls `arrange` on each child with its assigned rectangle. measure → solve → place. flexbox does the solving; you do the describing. fair trade.

the deep internals (sdf rendering, clipping, springs) live in [architecture](architecture.md), should you ever wonder what your `.spacing(8.0)` goes through to become pixels.
