# getting started

you want a window with buttons in it. this page gets you there in about five minutes. no prior gui knowledge required -- if anything, prior gui knowledge may slow you down, because glacex throws most of it away.

## step 0: install it

```bash
cargo add glacex
```

(linux: you may need x11/wayland dev packages first -- see the [README](../README.md#linux-dependencies). windows and macos: it just works. enjoy it while it lasts.)

## step 1: the smallest possible app

create a rust project, paste this into `main.rs`, run it:

```rust
use glacex::*;

struct Hello;

impl Widget for Hello {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let mut label = Label::new("hello. it works.").title();

        column![&mut label].arrange_at([40.0, 40.0], ui);
    }
}

fn main() {
    App::new(Hello).run();
}
```

a window appears with some text in it. congratulations, you're a gui programmer now. the bar was on the floor and you cleared it.

here's what each piece does:

| piece | job |
| ----- | --- |
| `struct Hello;` | your app's data. right now: none. that's fine. |
| `impl Widget for Hello` | makes your struct drawable. `fn ui()` runs **every frame**. |
| `Label::new(...)` | a text widget. just a struct describing text. |
| `column![...]` | a vertical layout holding the label. |
| `.arrange_at([40, 40], ui)` | measures everything and draws it at x=40, y=40. **layouts do nothing until you call this.** |
| `App::new(Hello).run()` | opens the window, starts the loop that calls `ui()` forever. |

## step 2: the frame loop (the one idea that matters)

most ui frameworks: create a button object once, keep it alive, mutate it when things happen. "retained mode". you own a little tree of objects and you must keep it consistent. it's like gardening.

glacex: **you rebuild the whole interface from scratch, every frame, at 60fps.** "immediate mode". there is no tree to keep consistent because you throw it away 60 times a second. it's less like gardening and more like describing what you want, loudly, forever.

```text
every frame:
  your ui() runs → widgets get built → layout measures them →
  widgets check input + draw themselves → gpu presents → repeat
```

this sounds wasteful. it isn't -- building a few dozen structs per frame is nothing, and the actual drawing happens on the gpu. what you gain: **no stale state, no "forgot to refresh the label" bugs.** if it's on screen, your code just said so.

practical consequence: write `ui()` as if it runs once. don't cache widgets in your struct (the borrow checker will stop you anyway -- that's it doing you a favor).

## step 3: handle a click

widgets return answers. you ask *after* laying them out:

```rust
fn ui(&mut self, ui: &mut Ui) {
    let mut button = Button::new("press it").id("the-button");

    column![&mut button].arrange_at([40.0, 40.0], ui);

    // .clicked() is only true on the frame the press-and-release finished.
    if button.clicked() {
        println!("pressed. the machine spirits are pleased.");
    }
}
```

the order is always the same:

1. **build** -- `let mut w = Widget::new(...)...;`
2. **layout** -- `column![...].arrange_at(pos, ui);`
3. **react** -- `if w.clicked() { ... }`, `w.text(ui)`, `w.value(ui)`...

react before layout and the widget hasn't seen this frame's input yet -- you'll be reading yesterday's news. build → layout → react. tattoo it somewhere.

## step 4: keep your own data

your struct is the natural home for *your* data (a counter, a todo list, which theme is active). widget-specific state (is the checkbox ticked? what's in the text field?) lives in `Ui`, not in your struct:

```rust
struct Counter {
    count: u32,          // yours. lives here.
}

fn ui(&mut self, ui: &mut Ui) {
    let mut button = Button::new("add one").id("add-one");
    //                                ^^^ the checkbox/text/slider state lives in Ui,
    //                                    keyed by this id. you'll meet it properly below.

    column![&mut button].arrange_at([40.0, 40.0], ui);

    if button.clicked() {
        self.count += 1; // your data, your rules.
    }
}
```

reading widget state uses tiny typed helpers: `checkbox.is_checked(ui)`, `slider.value(ui)`, `input.text(ui)`, `select.selected(ui)`... the full list is in the [widget reference](widgets.md#reading-state-cheat-sheet).

## the only rule that matters: give stateful widgets stable ids

every widget gets an id. no id given? it gets a random one -- a fresh uuid **every frame**. for a label that's fine (labels remember nothing). for a checkbox that's a disaster: new id every frame means its "am i ticked?" memory is wiped every frame. the checkbox will look at you blankly forever.

so: **any widget that remembers something gets `.id("something-stable")`:**

```rust
let mut check = Checkbox::new().id("remember-me");   // remembers ticked/unticked
let mut name  = TextInput::new().id("name-field");   // remembers its text
let mut vol   = Slider::new(0.0, 100.0).id("volume"); // remembers position
let mut label = Label::new("i forget nothing because i know nothing"); // no id needed
```

which widgets need ids? anything interactive: `Button` (technically stateless, but an id keeps its press animation smooth and its tooltip/accessibility stable), `Checkbox`, `Switch`, `Slider`, `RadioButton`, `TextInput`, `TextArea`, `SelectBox`, `Tabs`, `ScrollView`, `ProgressBar`. when in doubt, add `.id()`. ids are free and regret is expensive.

under the hood, state is a `HashMap<String, Box<dyn Any>>` inside `Ui`. `widget.state(ui)`, `.take_state(ui)`, `.put_state(ui, state)` give you the whole state struct when you need it (see [widget reference](widgets.md#state-how-widgets-remember-things)), and `ui.widget_state::<T>(id)` stores your own custom structs. that's the whole persistence story. no databases were harmed.

## step 5: arrange your furniture (layout in 30 seconds)

```rust
// vertical stack:
column![&mut a, &mut b, &mut c]
    .spacing(12.0)              // gap between kids (default 8)
    .align(Alignment::Start)    // cross-axis: Start | Center | End
    .padding([16.0, 16.0])      // breathing room inside the edges
    .arrange_at([40.0, 40.0], ui);  // put it THERE. required. no arrange, no pixels.

// horizontal line:
row![&mut a, &mut b].spacing(8.0).arrange_at([40.0, 200.0], ui);
```

nest freely: rows inside columns inside cards inside scroll views. full guide: [layout](layout.md).

## step 6: change how it looks

```rust
ui.set_theme(Theme::DARK);   // first line of ui(). whole app follows.
```

nine built-in themes, swap any time, even mid-frame. per-widget costumes via `.primary()`, `.outline()`, `.ghost()`, `.danger()`, `.success()`, `.elevated()`, `.subtle()`... or full custom `*Style` structs when you're feeling fancy. details: [themes & styling](themes-and-styling.md).

## the `App` builder

```rust
App::new(MyRoot)
    .title("my app")              // window title
    .window_size(800, 600)        // initial size (logical pixels)
    .accessibility_enabled(true)  // screen-reader support via accesskit
    .update(|root| { /* runs before ui() each frame */ })
    .run();                       // opens the window, never returns (until close)
```

also useful inside `ui()`: `ui.set_bgcolor(color)`, `ui.set_title("...")`, `ui.window_size()`, `ui.set_theme(theme)`, `ui.load_image("assets/pic.png")`.

note on `.update()`: it runs before drawing with `&mut` access to your root struct -- handy for polling external state (a file watcher, a channel) before widgets read it. most apps never need it. that's fine. it's there, it's patient.

## where to go next

| you want... | go read... |
| ----------- | ---------- |
| every widget + every builder method | [widget reference](widgets.md) |
| rows, columns, sizing, alignment | [layout guide](layout.md) |
| colors, themes, gradients, custom styles | [themes & styling](themes-and-styling.md) |
| your own widget (todo-item style) | [custom widgets](custom-widgets.md) |
| gpu pipelines, sdf, springs, accesskit internals | [architecture](architecture.md) |
| commented code you can steal | `cargo run --example example1` (smallest) |

and when something looks wrong, check the two usual suspects: **did you call `.arrange_at()`?** and **does your stateful widget have a stable `.id()`?** these two cover roughly 90% of all beginner suffering. the remaining 10% is the borrow checker, and nobody can help you with that. not even us.
