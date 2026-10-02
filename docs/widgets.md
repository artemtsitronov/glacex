# widget reference

every widget glacex ships, what it does, and which knobs it has. the pattern never changes:

```rust
let mut w = Widget::new(...)   // construct
    .id("stable-id")           // stable id if it remembers anything (see below)
    .size([200.0, 40.0]);      // optional builders

column![&mut w].arrange_at([x, y], ui);  // lay out (draws it)

if w.clicked() { /* react */ }           // ask what happened (after layout)
```

> **ids:** `Widget::new()` without `.id()` generates a fresh random id *every frame*. stateless widgets (labels, dividers...) don't care. stateful ones (checkboxes, text fields, sliders...) lose their memory every frame and malfunction with great confidence. **give every interactive widget a stable `.id()`.** full explanation: [getting started](getting-started.md#the-only-rule-that-matters-give-stateful-widgets-stable-ids).

## state: how widgets remember things

every stateful widget implements `StatefulWidget`, which hands you three methods keyed by the widget's own id:

```rust
widget.state(ui)             // &mut State -- borrow it in place
widget.take_state(ui)        // State -- take it out (owned), put it back later
widget.put_state(ui, state)  // hand it back
```

use these when you need the whole state struct (e.g. flipping several fields at once). for everyday reads/writes, each widget has tiny typed helpers instead:

## reading-state cheat sheet

| widget | read | write |
| ------ | ---- | ----- |
| `Checkbox` | `.is_checked(ui) -> bool` | `.check(ui, bool)` |
| `Switch` | `.enabled(ui) -> bool` | `.set_enabled(ui, bool)` |
| `Slider` | `.value(ui) -> f32` | `.set_value(ui, f32)` |
| `ScrollView` | `.offset(ui) -> [f32; 2]` | `.set_offset(ui, [f32; 2])` |
| `SelectBox` | `.selected(ui) -> Option<String>` | `.set_selected(ui, Option<String>)` |
| `TextInput` / `TextArea` | `.text(ui) -> String` | `.set_text(ui, String)` |
| `Tabs` | `.selected(ui) -> Option<String>` | `.set_selected(ui, id)` |
| radio groups | `ui.selected_option("group_id") -> Option<&str>` | (clicking selects) |

and for state that belongs to *no* widget (like "which switch was flipped last"), `Ui` exposes the raw map directly: `ui.widget_state::<T>(id)` / `ui.take_widget_state::<T>(id)` / `ui.put_widget_state(id, val)`. `examples/demo.rs` uses this for its pick-two-of-three switches logic.

---

## controls (things you poke)

### Button

the fruit fly of ui research. hover/press animations, press sinks 2px, shadow compresses, cursor becomes a pointer because it respects you.

```rust
let mut b = Button::new("launch").id("launch").primary();
column![&mut b].arrange_at([40.0, 40.0], ui);
if b.clicked() { /* ... */ }
```

* constructor: `Button::new("label")`
* variants: `.primary()` (loud, accent-colored), `.outline()` (border, no fill), `.ghost()` (invisible until hovered -- the introvert), `.danger()` (red, for buttons that delete things)
* builders: `.id()`, `.style(ButtonStyle)`, `.tooltip("...")` (hover popup), `.size([w,h])` / `.width()` / `.height()`, `.padding([x,y])`
* queries: `.clicked()`, `.hovered()`, `.pressed()`
* response: `ui()` returns `Interaction { hovered, pressed, clicked }`

### Checkbox

boolean tick box. the checkmark draws itself in two strokes, like it's signing something important.

```rust
let mut c = Checkbox::new().id("remember").default_checked(true);
column![&mut c].arrange_at([40.0, 40.0], ui);
if c.is_checked(ui) { /* ... */ }
```

* builders: `.id()`, `.style(CheckboxStyle)`, `.default_checked(bool)` (initial value), `.size()`
* state: `.is_checked(ui)`, `.check(ui, bool)`
* response: `CheckboxResponse { checked, clicked, hovered }`

### RadioButton

pick exactly one. radios share a **group id**; each option gets its own id via `.id()`:

```rust
let mut yes = RadioButton::new("answer").id("yes");
let mut no  = RadioButton::new("answer").id("no");
row![&mut yes, &mut no].arrange_at([40.0, 40.0], ui);

match ui.selected_option("answer") {
    Some("yes") => { /* ... */ }
    Some("no")  => { /* ... */ }
    _ => { /* nothing picked yet. suspense. */ }
}
```

* constructor: `RadioButton::new("group_id")`, then `.id("option_id")`
* read: `ui.selected_option("group_id") -> Option<&str>`
* response: `RadioButtonResponse { selected, clicked, hovered }`

### Switch

a checkbox that went to design school. sliding knob, slides with feeling.

```rust
let mut s = Switch::new().id("wifi").default_enabled(true);
column![&mut s].arrange_at([40.0, 40.0], ui);
if s.enabled(ui) { /* ... */ }
```

* builders: `.id()`, `.style(SwitchStyle)`, `.default_enabled(bool)`, `.size()`
* state: `.enabled(ui)`, `.set_enabled(ui, bool)`
* response: `SwitchResponse { enabled, clicked, hovered }`

### Slider

drag a number between min and max. thumb grows while dragging, leaves a little motion trail, because joy matters.

```rust
let mut s = Slider::new(0.0, 100.0).id("volume").width(200.0).default_value(65.0);
column![&mut s].arrange_at([40.0, 40.0], ui);
let volume: f32 = s.value(ui);
```

* constructor: `Slider::new(min, max)` (default width 200px)
* builders: `.id()`, `.width()` / `.height()` / `.size()`, `.style(SliderStyle)`, `.default_value(f32)`
* state: `.value(ui)`, `.set_value(ui, f32)`
* response: `SliderResponse { value, changed, dragging, hovered }`

### TextInput

single-line text field. click/drag to select, double-click selects a word, triple-click selects all, `ctrl+a/c/v`, blinking cursor, auto-scrolls when your novel exceeds the box.

```rust
let mut name = TextInput::new()
    .id("name")
    .width(240.0)
    .placeholder("your name here");
column![&mut name].arrange_at([40.0, 40.0], ui);
let typed: String = name.text(ui);
```

* builders: `.id()`, `.width()` / `.height()` / `.size()`, `.placeholder("...")` (ghost text when empty), `.default_text("...")` (initial content), `.style(TextInputStyle)`, `.padding()`
* state: `.text(ui) -> String`, `.set_text(ui, String)`
* queries: `.focused(ui)`, `.hovered(ui)`

### TextArea

`TextInput`'s taller sibling. multi-line, `enter` for newlines, arrow keys with column memory, its own draggable scrollbar.

```rust
let mut notes = TextArea::new()
    .id("notes")
    .size([280.0, 120.0])
    .default_text("dear diary,");
column![&mut notes].arrange_at([40.0, 40.0], ui);
let essay: String = notes.text(ui);
```

* builders: same as `TextInput` minus placeholder (default 240×120)
* state: `.text(ui) -> String`, `.set_text(ui, String)`

### SelectBox

dropdown menu. click to open a spring-animated floating list, click outside or `esc` to close, arrow keys + `enter` work, mouse wheel scrolls long lists, `.searchable()` adds a type-to-filter box.

```rust
let mut pick = SelectBox::new(vec![
    SelectOption::new("mocha", "Catppuccin Mocha"),
    SelectOption::new("nord", "Nord"),
])
.id("theme-pick")
.placeholder("choose...")
.width(220.0);

column![&mut pick].arrange_at([40.0, 40.0], ui);

if let Some(value) = pick.selected(ui) {
    // value is "mocha" / "nord" -- the value, not the label.
}
```

* builders: `.id()`, `.placeholder()`, `.width()`, `.searchable()`, `.style(SelectBoxStyle)`, `.tooltip()`
* state: `.selected(ui) -> Option<String>`, `.set_selected(ui, Option<String>)`
* note: the dropdown renders as an overlay, so it escapes `ScrollView` clipping. it knows what it did.

### Tabs

tab strip with a pill that slides between tabs like it pays rent nowhere.

```rust
let mut tabs = Tabs::new(vec![
    TabItem::new("Files").id("files"),
    TabItem::new("Edit").id("edit"),
    TabItem::new("View").id("view"),
])
.id("main-tabs");

let response = column![&mut tabs].arrange_at([40.0, 40.0], ui);
// wait -- column! returns (), not the response. see below.
```

careful: inside `row!`/`column!` the response is swallowed (containers return `()`). to get `TabsResponse`, call it directly:

```rust
let mut tabs = Tabs::new(vec![
    TabItem::new("Files").id("files"),
    TabItem::new("Edit").id("edit"),
]).id("main-tabs");

let response = tabs.ui(ui);  // draws at 0,0 -- or wrap in layout and use tabs.selected(ui)

match response.selected.as_str() {
    "files" => { /* ... */ }
    "edit"  => { /* ... */ }
    _ => {}
}
```

* builders: `.id()`, `.style(TabsStyle)`
* state: `.selected(ui) -> Option<String>`, `.set_selected(ui, id)`
* response: `TabsResponse { selected, changed, hovered_index }` -- `changed` is true exactly on the frame the selection moved, handy for driving a `match` without re-reading state.
* behavior: the first tab auto-selects on frame one if nothing is stored yet. the active label color picks itself for contrast against the pill fill.

---

## containers (things that hold other things)

### Card

a rounded panel with padding, border, and shadow. the workhorse. put anything inside:

```rust
let mut body = Label::new("contents. very important.");
let mut card = Card::new(&mut body)   // Card borrows your widget for the frame
    .padding([20.0, 20.0])
    .elevated();                       // or .subtle(), or default
column![&mut card].arrange_at([40.0, 40.0], ui);
```

* constructor: `Card::new(&mut child)` (any `Measurable`)
* variants: default, `.subtle()` (quiet, inset feel), `.elevated()` (floating, deeper shadow)
* builders: `.id()`, `.padding([x,y])`, `.size()`, `.style(CardStyle)`

### ScrollView

a box with more inside than fits. wheel to scroll, drag the thumb, scrollbar fades away when idle like a shy animal.

```rust
let mut list = column![&mut item1, &mut item2, &mut item3];
let mut scroll = ScrollView::new(&mut list)
    .id("list-scroll")          // scrolling offset is state → stable id required
    .size([300.0, 200.0]);
column![&mut scroll].arrange_at([40.0, 40.0], ui);
```

* builders: `.id()`, `.size()`, `.padding()`, `.default_offset([x, y])`, `.style(ScrollViewStyle)`
* state: `.offset(ui) -> [f32; 2]`, `.set_offset(ui, [f32; 2])`

### Container

a fixed-size box with padding that draws nothing itself. useful when a layout needs a child of exact size, or you want padding without a card's opinions.

```rust
let mut inner = Label::new("exactly here");
let mut box_ = Container::new(&mut inner).size([200.0, 100.0]).padding([8.0, 8.0]);
```

* builders: `.id()`, `.size()` (default 100×100), `.padding()`, `.arrange_at(pos, ui)`

### Divider

a line. separates things. asks for nothing.

```rust
let mut line = Divider::horizontal(320.0);          // 320px wide, 1px tall
let mut side = Divider::vertical(120.0).faint();   // subtle. mysterious.
```

* constructors: `Divider::horizontal(length)`, `Divider::vertical(length)`
* builders: `.id()`, `.thickness(px)`, `.color(Color)`, `.faint()` (barely-there border color), `.size()`

---

## displays (things you look at)

### Label

text, in the bundled Geist / Geist Mono fonts. sizes and colors chain like adjectives:

```rust
let mut a = Label::new("Big News").title();        // 22px bold
let mut b = Label::new("details...").secondary();  // dimmer
let mut c = Label::new("x = 42").mono().muted();   // mono font, muted -- chains stack
```

* colors: default, `.secondary()`, `.muted()`, `.accent()` (theme accent), `.success()`, `.warning()`, `.error()`, `.color(anything)`
* sizes: `.caption()` (12px), default 14px, `.subheading()` (16px), `.heading()` (18px), `.title()` (22px), `.metric()` (28px bold), `.size_preset(px)` (custom, sensible line height), `.size_with_line_height(px, lh)` (custom, your funeral)
* weight: `.medium()`, `.semibold()`, `.bold()`
* font: `.mono()` for Geist Mono
* sizing: `.width()` / `.height()` / `.size()` to override the natural text size

### Badge

a status pill. for when a label needs to look like it has a job:

```rust
let mut b = Badge::new("LIVE").success();   // .secondary() .outline() .warning() .error()
```

* builders: `.id()`, `.variant(BadgeVariant)`, `.size()`, `.padding()`, `.style(BadgeStyle)`

### ProgressBar

a bar that fills. takes `0.0..=1.0`, animates smoothly toward it even if you yank the value around (it has grace, unlike us):

```rust
let mut p = ProgressBar::new(downloaded_total).id("dl").width(240.0).success();
column![&mut p].arrange_at([40.0, 40.0], ui);
// .success() .warning() .error() -- pick your emotional register
```

* builders: `.id()`, `.width()` / `.height()` / `.size()` (default 200×6), `.style(ProgressBarStyle)`

---

that's all of them. nine controls, four containers, three displays -- sixteen widgets, zero javascript. for layout, see the [layout guide](layout.md); for making them pretty, [themes & styling](themes-and-styling.md); for inventing your own, [custom widgets](custom-widgets.md).
