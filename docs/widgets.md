# Glacex Widget Reference

This page lists every widget in `glacex` and how to use it.

If you don't give a widget an id with `.id()`, `Widget::new()` makes a new UUID every time you call it.
The id is what the widget's state is saved under between frames. It's also what shows up in the accessibility tree when that's turned on (see [Accessibility](../README.md#accessibility)).

[ATTENTION] Stateful widgets need their id to find their state. If the id changes every frame (which is what happens by default), they can break.
So just give them their own id.

## State access

Every stateful widget implements `StatefulWidget`, so it gets these three methods for free. They all use the widget's own id

```rust
widget.state(ui)             // &mut State, get-or-init in place
widget.take_state(ui)        // State, get-or-init, owned (removed from Ui until put back)
widget.put_state(ui, state)  // hand a State back
```

Use these when you need the whole state struct, like when you're changing a few fields at once. The three switches in `examples/demo.rs` that turn each other off are a good example. If you just want to read or set one value, most widgets have simpler helpers built on top of those three

| Widget | Read | Write |
|---|---|---|
| `Checkbox` | `.is_checked(ui) -> bool` | `.check(ui, bool)` |
| `Switch` | `.enabled(ui) -> bool` | `.set_enabled(ui, bool)` |
| `Slider` | `.value(ui) -> f32` | `.set_value(ui, f32)` |
| `ScrollView` | `.offset(ui) -> [f32; 2]` | `.set_offset(ui, [f32; 2])` |
| `SelectBox` | `.selected(ui) -> Option<String>` | `.set_selected(ui, Option<String>)` |
| `TextInput` / `TextArea` | `.text(ui) -> String` | `.set_text(ui, String)` |

`Ui::widget_state`, `take_widget_state` and `put_widget_state` are still there underneath, and the widget methods above just call them. You can use them yourself for state that doesn't belong to one widget. For example `examples/demo.rs` keeps a plain `TripleToggleOrder` struct this way to remember which of the three switches was turned on last.

## 1. Controls

### Button
A normal button that animates when you hover and press it.
- **Constructor** `Button::new("Label")`
- **Variants** `.primary()`, `.outline()`, `.ghost()` and `.danger()`
- **Builder methods**
  - `.id("...")`
  - `.style(ButtonStyle)` sets the fill, text style, border, shadow, scale and offset. Each one can be different per state with `Stateful<T>`
  - `.fill(..)`, `.border_color(..)`, `.border_width(..)` and `.shadow(..)` take a plain value or a `Stateful`
  - `.tooltip("text")` shows a tooltip when you hover
  - `.size([w, h])`, `.width(px)` and `.height(px)`
  - `.padding([x, y])` which also takes a `Stateful<[f32; 2]>`
- **Queries** `.clicked() -> bool`, `.hovered() -> bool` and `.pressed() -> bool`
- **Animation** `ButtonState` keeps track of `hover_t` and `press_t` (with `Motion::SNAPPY` and `Motion::INSTANT`). When you press it the button moves down 1px and the shadow gets smaller. On hover the border gets brighter and goes to `theme.border_strong`. Hovering and pressing drive the `hovered` and `pressed` layers of every `Stateful` field.

### Checkbox
A checkbox with an animated tick.
- **Constructor** `Checkbox::new()`
- **Builder methods** `.id("...")`, `.style(CheckboxStyle)`, `.default_checked(bool)` and `.size([w, h])`
- **Reading and writing state** `checkbox.is_checked(ui) -> bool` and `checkbox.check(ui, bool)`
- **Animation** `CheckboxState` keeps track of `anim_progress` and `hover_t` with `Motion::SNAPPY`. The tick is drawn as two strokes that overlap a bit. The left one goes from 0 to 35% and the right one from 30 to 100%. When it's checked the border moves toward `theme.active`. Being checked drives the `selected` layer of every `Stateful` field.

### RadioButton
Only one radio button in a group can be picked at a time.
- **Builder methods** `.id("...")`
- **Constructor** `RadioButton::new("group_id")`
- **Reading state**
  ```rust
  let selected = ui.selected_option("group_id");
  ```
- **Animation** `RadioButtonAnimState` keeps track of `dot_t` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`).

### Switch
A small toggle with a knob that slides.
- **Constructor** `Switch::new()`
- **Builder methods** `.id("...")`, `.style(SwitchStyle)`, `.default_enabled(bool)` and `.size([w, h])`
- **Reading and writing state** `switch.enabled(ui) -> bool` and `switch.set_enabled(ui, bool)`
- **Animation** `SwitchState` keeps track of `anim_progress` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`).

### Slider
Lets you pick a number in a range.
- **Constructor** `Slider::new(min, max)`
- **Builder methods** `.id("...")`, `.width(px)`, `.height(px)`, `.size([w, h])`, `.style(SliderStyle)` and `.default_value(f32)`
- **Reading and writing state** `slider.value(ui) -> f32` and `slider.set_value(ui, f32)`
- **Animation** `SliderState` keeps track of `hover_t` (`Motion::SNAPPY`) and `drag_t` (`Motion::INSTANT`). The thumb gets 2.5px bigger while you drag it. That works because `thumb_size` is `Stateful` and dragging counts as the `pressed` layer.

### TextInput
A one line text box.
- Click and drag to select, double click selects a word and triple click selects everything
- Clipboard with `Ctrl+C`, `Ctrl+V`, `Ctrl+A` and `Ctrl+X`
- Blinking cursor, and it scrolls by itself when the text gets too long
- **Constructor** `TextInput::new()`
- **Builder methods**
  - `.id("...")`
  - `.width(px)`, `.height(px)` and `.size([w, h])`
  - `.placeholder("...")` is shown in `theme.text_muted` when it's empty
  - `.default_text("...")` is the text it starts with
  - `.style(TextInputStyle)`
- **Reading and writing state** `input.text(ui) -> String` and `input.set_text(ui, String)`
- **Animation** When it's focused there's a focus ring with `Motion::GENTLE`. The border gets 0.5px thicker and the glow shadow grows by 6px. The hover border uses `Motion::SNAPPY`.

### TextArea
A text editor with multiple lines that scrolls.
- Scrolls up and down and has a scrollbar you can drag
- Arrow keys move around and remember which column you were in, and `Enter` makes a new line
- **Constructor** `TextArea::new()`
- **Builder methods** `.id("...")`, `.width(px)`, `.height(px)`, `.size([w, h])`, `.default_text("...")` and `.style(TextAreaStyle)`
- **Reading and writing state** `area.text(ui) -> String` and `area.set_text(ui, String)`
- **Animation** Same focus ring as `TextInput`.

### SelectBox
A button that opens a dropdown list with a spring animation.
- You can use the arrow keys, `Enter` to pick and `Escape` to close. Clicking outside closes it too
- You can turn on a search bar to filter the list, and long lists scroll with the mouse wheel
- **Constructor** `SelectBox::new(vec![SelectOption::new("value", "Label"), ...])`
- **Builder methods** `.placeholder("...")`, `.width(px)`, `.searchable()`, `.style(SelectBoxStyle)` and `.tooltip("...")`
- **Reading and writing state** `select.selected(ui) -> Option<String>` and `select.set_selected(ui, Option<String>)`
- **Animation** A spring (stiffness 380 and damping 30) slides the dropdown open and closed and spins the chevron 180°. Each item fades on hover with `Motion::SNAPPY`.

### Tabs
A row of tabs with a pill that slides to the one you picked.
- **Constructor** `Tabs::new(vec![TabItem::new("Tab 1"), TabItem::new("Tab 2"), ...])`
- **Builder methods** `.style(TabsStyle)` with the presets `TabsStyle::segmented` and `TabsStyle::underline`
- **Reading and writing state** `tabs.selected(ui) -> Option<String>` and `tabs.set_selected(ui, "tab_id")`
- **Response** `.ui(ui)` gives you back `TabsResponse { selected, changed, hovered_index }`. `changed` is `true` on the frame the selection changed. It's handy when you `match` on the selected id, because you don't have to read the state again.
- **Behavior** If nothing is selected yet, the first `TabItem` gets picked on the first frame.
- **Animation** The pill behind the active tab slides and resizes with `indicator_motion` (`Motion::FLUID` by default). Each tab's `text_style` and `tab_fill` fade from `idle` to `hovered` with `hover_motion` (`Motion::SNAPPY` by default). The active tab uses the `selected` values. If the selected text style doesn't have a color, it picks one that's readable on top of `indicator_fill`.

```rust
let tabs_response = Tabs::new(
    vec![
        TabItem::new("Tab 1").id("tab1"),
        TabItem::new("Tab 2").id("tab2"),
        TabItem::new("Tab 3").id("tab3"),
    ],
)
.id("tabs")
.ui(ui);

match tabs_response.selected.as_str() {
    "tab1" => { /* ... */ }
    "tab2" => { /* ... */ }
    _ => { /* ... */ }
}
```

---

## 2. Containers

### Card
A box with rounded corners, padding, a border and a shadow if you want one.
- **Constructor** `Card::new(&mut child_widget)`
- **Variants** `.subtle()` looks sunk in and uses `theme.surface_subtle`. `.elevated()` floats with a bigger shadow
- **Builder methods** `.id("...")`, `.padding([x, y])`, `.size([w, h])` and `.style(CardStyle)`

### ScrollView
A container that scrolls and has a scrollbar you can drag.
- **Constructor** `ScrollView::new(&mut child)`
- **Builder methods** `.id("...")`, `.size([w, h])`, `.padding([x, y])`, `.default_offset([x, y])` and `.style(ScrollViewStyle)`
- **Reading and writing state** `scroll.offset(ui) -> [f32; 2]` and `scroll.set_offset(ui, [f32; 2])`

### Container
Wraps a child widget in a box with a fixed size.
- **Constructor** `Container::new(&mut child)`
- **Builder methods** `.id("..")`, `.size([w, h])` and `.padding([x, y])`

### Divider
A line to separate things.
- **Constructor** `Divider::horizontal(length)` or `Divider::vertical(length)`
- **Builder methods**
  - `.id("...")`
  - `.faint()` makes it use `theme.border_faint`
  - `.thickness(px)` and `.color(Color)`
  - `.style(DividerStyle)` sets the fill, thickness, margin, how far in the ends are and the shape. There's `DividerStyle::faint`, `strong` and `rounded`

---

## 3. Displays

### Label
Text. It uses the Geist and Geist Mono fonts that come with glacex.
- **Constructor** `Label::new("Text")`
- **Colors** `.secondary()`, `.muted()`, `.accent()`, `.success()`, `.warning()`, `.error()` and `.color(Color)`
- **Sizes** `.caption()` is 12px, `.subheading()` is 16px, `.heading()` is 18px, `.title()` is 22px and `.metric()` is 28px and bold
- **Weight** `.medium()`, `.semibold()` and `.bold()`
- **Font** `.mono()` switches to Geist Mono
- **Layout and background** `.style(LabelStyle)` sets the padding, `align_x` and `align_y` inside its box, and an optional background, border and shape. `.align(Alignment)` is a shortcut

### Badge
A small status pill.
- **Constructor** `Badge::new("Text")`
- **Variants** `.secondary()`, `.outline()`, `.success()`, `.warning()` and `.error()`
- **Builder methods** `.id("...")`, `.size([w, h])`, `.padding([x, y])` and `.style(BadgeStyle)`

### ProgressBar
Shows how much of something is done.
- **Constructor** `ProgressBar::new(ratio)` where `ratio` goes from `0.0` to `1.0`
- **Variants** `.success()`, `.warning()` and `.error()`
- **Builder methods** `.id("stable_id")`, `.size([w, h])` and `.style(ProgressBarStyle)`
- **Animation** `ProgressBarState` animates `animated_progress` with `Motion::FLUID`
