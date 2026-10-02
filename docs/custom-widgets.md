# custom widgets

built-in widgets cover the classics. sooner or later you'll want something that isn't in the box -- a todo row, a color swatch, a kanban card with opinions. good news: a widget is just a struct plus two traits. this page builds one from scratch.

## the contract

```rust
pub trait Widget {
    type Output;                              // what ui() returns (answers, or () for quiet widgets)
    fn ui(&mut self, ui: &mut Ui) -> Self::Output;  // draw + respond. runs every frame.
    fn on_start(&mut self, _ui: &mut Ui) {}   // runs once before frame one (load images here)
}

pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];  // natural size [width, height]
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;  // draw at position
}
```

`Widget` alone: drawn via `ui.add(&mut w)` or `w.ui(ui)` at the origin. `+ Measurable`: placeable inside `row!`/`column!`/`Card`/`ScrollView` -- which is where you want to be. implement both. it takes ten lines.

## example: a todo row

a label plus a delete button. measures itself, arranges its children, reports whether it was deleted:

```rust
use glacex::*;

struct TodoResponse {
    deleted: bool,   // the answer this widget gives each frame
}

struct Todo {
    text: String,
}

impl Measurable for Todo {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [300.0, 40.0]   // "i am 300 by 40. no further questions."
    }

    fn arrange(&mut self, pos: [f32; 2], size: [f32; 2], ui: &mut Ui) -> TodoResponse {
        // compose built-ins inside yourself. free labor.
        let mut label = Label::new(&self.text);
        let mut button = Button::new("delete").id(format!("del-{}", self.text));

        row![&mut label, &mut button]
            .size(size)
            .align(Alignment::Center)
            .arrange_at(pos, ui);

        TodoResponse { deleted: button.clicked() }
    }
}

impl Widget for Todo {
    type Output = TodoResponse;

    fn ui(&mut self, ui: &mut Ui) -> TodoResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}
```

three observations:

1. **compose, don't draw.** the todo row contains zero drawing code -- it arranges a `Label` and a `Button` and reads the answer. most custom widgets are just choreography over built-ins.
2. **ids must be unique per instance.** two todos with the same text would share `del-<text>` state (both delete buttons would act as one -- spooky action at a distance). in real code, key on a unique todo id, not the text.
3. **fresh children every frame is fine.** `label` and `button` are rebuilt each frame; their *state* persists in `Ui` under their ids. building is cheap, remembering is `Ui`'s job.

## using it: dynamic lists

parents usually hold a `Vec` of your widget and rebuild layout each frame:

```rust
struct TodoApp {
    todos: Vec<String>,
}

impl Widget for TodoApp {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        // 1. build one Todo per item (fresh structs, every frame -- normal, healthy)
        let mut rows: Vec<Todo> = self.todos.iter().map(|t| Todo { text: t.clone() }).collect();

        // 2. box them as trait objects for a dynamic Column
        let boxed: Vec<Box<dyn AnyWidget + '_>> =
            rows.iter_mut().map(|t| Box::new(t) as Box<dyn AnyWidget>).collect();

        // 3. lay out inside a scroll view inside a card, like a civilized person
        let mut list = Column::new(boxed);
        let mut scroll = ScrollView::new(&mut list).size([320.0, 400.0]).id("todos");
        let mut card = Card::new(&mut scroll);
        column![&mut card].arrange_at([40.0, 40.0], ui);

        // 4. react: borrow ended with the layout (drop it if the compiler complains),
        //    so now we may mutate self.
        let deleted: Vec<String> = rows.iter()
            .filter(|t| /* was its button clicked? see below */ false)
            .map(|t| t.text.clone())
            .collect();
        self.todos.retain(|t| !deleted.contains(t));
    }
}
```

the borrow dance in step 4 deserves a sentence: `rows` is borrowed by the layout while it arranges. to mutate `self.todos` afterwards, the borrows must be over -- usually they end naturally, and if not, `drop(layout)` ends them explicitly. (`examples/reminder.rs` shows the full working version, including the add-item input and the `drop`.)

to actually read each row's answer, have `arrange` *record* it somewhere you can reach -- e.g. store the `TodoResponse` on the struct (`self.deleted = button.clicked()`), then read `rows[i].deleted` after layout. state on your own struct is just a field; only *widget* state needs `Ui`.

## going lower: raw drawing

if composition isn't enough (custom charts, game boards, visualizations), skip widgets and draw shapes directly in your `ui()`:

```rust
// a red dot. modern art. you may applaud.
ui.draw_shape(
    MeasurablePath::rect([x, y], [8.0, 8.0], [4.0; 4]),
    Fill::Solid(Color::RED),
    0.0, Color::TRANSPARENT, 0.0, false, 0.0,
);
```

`Ui` gives you the primitives: `draw_shape`, `draw_open_shape` (partial strokes, `reveal: 0.0..=1.0`), `draw_text` / `draw_text_styled`, `measure_text`, `push_clip` / `pop_clip` (confine drawing to a rectangle), `push_input_block` / `pop_input_block` (make an overlay swallow clicks), `show_tooltip`, `set_cursor_icon`, mouse/keyboard queries (`mouse_position`, `mouse_pressed_this_frame`, `key_pressed`, `typed_text`...), clipboard (`copy_to_clipboard` / `paste_from_clipboard`), `dt()` / `elapsed_seconds()` for animation.

input handling pattern (from the built-ins): test `shape.contains(mouse)` + `!ui.is_input_blocked(mouse)` + `ui.point_in_current_clip(mouse)` → hovered; `mouse_pressed_this_frame()` on hover → clicked. or just use `Interaction::update(&shape, ui)` -- it does exactly that and returns `{ hovered, pressed, clicked }`. and `animate_towards(value, target, dt, Motion::SNAPPY)` makes any number glide instead of snap. custom slider in an afternoon, easy.

## checklist before you ship it

* `measure` returns something sane (parents trust it blindly).
* stateful children get **unique, stable** `.id()`s (one id per instance, same id every frame).
* if it holds children, implement `Measurable` so it works in layouts.
* `Output` type carries the answers (`clicked`, `deleted`, `value`...); `()` if it has nothing to say.
* register accessibility (`ui.register_accessible(self, bounds)` + `impl Accessible`) if you enable accesskit -- one line each, screen-reader users thank you.

see it all working: [`examples/reminder.rs`](../examples/reminder.rs) -- a todo app built around exactly this pattern, heavily commented.
