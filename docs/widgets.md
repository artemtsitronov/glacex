# Glacex Widget Reference

API reference for all widgets provided by `glacex`.

Most widgets take a stable string id as their first constructor argument. That id is what
persistent state is keyed on across frames, and it's also what shows up in the accessibility
tree (see [Accessibility](../README.md#accessibility)) when one is enabled. `Card`,
`Container`, `Divider`, and `ProgressBar` don't require an id up front — call `.id("...")` on
them if you need stable state or want that instance to be addressable by assistive tech;
otherwise they're fine left anonymous. That `.id(..)` builder accepts anything implementing
`IntoId` — `&str`, `String`, `Some("...")`, or `None` to explicitly clear it.

## State access

Every stateful widget implements `StatefulWidget`, which gives it three methods for free,
keyed by its own id:

```rust
widget.state(ui)             // &mut State — get-or-init in place
widget.take_state(ui)        // State — get-or-init, owned (removed from Ui until put back)
widget.put_state(ui, state)  // hand a State back
```

Reach for these when you need the whole state struct at once — e.g. mutating several fields
together, like the three-way mutual-exclusion logic in `examples/demo.rs`. For the common
case of reading or writing a single value, most widgets also expose small typed convenience
methods built on top of these three:

| Widget | Read | Write |
|---|---|---|
| `Checkbox` | `.is_checked(ui) -> bool` | `.check(ui, bool)` |
| `Switch` | `.enabled(ui) -> bool` | `.set_enabled(ui, bool)` |
| `Slider` | `.value(ui) -> f32` | `.set_value(ui, f32)` |
| `ScrollView` | `.offset(ui) -> [f32; 2]` | `.set_offset(ui, [f32; 2])` |
| `SelectBox` | `.selected(ui) -> Option<String>` | `.set_selected(ui, Option<String>)` |
| `TextInput` / `TextArea` | `.text(ui) -> String` | `.set_text(ui, String)` |

`Ui::widget_state` / `take_widget_state` / `put_widget_state` are still there underneath, and
are what the widget methods above call into. Reach for them directly for state that isn't
tied to a single widget's id — `examples/demo.rs` uses a plain `TripleToggleOrder` struct
this way, to remember which of three switches was flipped on most recently.

## 1. Controls

### Button
Push button with hover/press animation.
- **Constructor**: `Button::new("btn_id", "Label")`
- **Variants**: `.primary()`, `.outline()`, `.ghost()`, `.danger()`
- **Builder methods**:
  - `.style(ButtonStyle)` — fill, hover fill, pressed fill, text color, border, shadow
  - `.tooltip("text")` — floating tooltip on hover
  - `.size([w, h])` / `.width(px)` / `.height(px)`
  - `.padding([x, y])`
- **Queries**: `.clicked() -> bool`, `.hovered() -> bool`, `.pressed() -> bool`
- **Animation**: `ButtonState` tracks `hover_t` and `press_t` (`Motion::SNAPPY` / `Motion::INSTANT`). On press the button shifts 1px down and the shadow compresses. Border brightens to `Theme::BORDER_STRONG` on hover.

### Checkbox
Boolean toggle with an animated tick.
- **Constructor**: `Checkbox::new("checkbox_id")`
- **Builder methods**: `.style(CheckboxStyle)`, `.default_checked(bool)`, `.size([w, h])`
- **Reading/writing state**: `checkbox.is_checked(ui) -> bool`, `checkbox.check(ui, bool)`
- **Animation**: `CheckboxState` tracks `anim_progress` and `hover_t` via `Motion::SNAPPY`. The tick draws in two overlapping strokes (left leg 0–35%, right leg 30–100%). Border moves toward `Theme::ACTIVE` when checked.

### RadioButton
Mutually exclusive selection within a named group.
- **Constructor**: `RadioButton::new("group_id", "option_id")`
- **Reading state**:
  ```rust
  let selected = ui.selected_option("group_id");
  ```
- **Animation**: `RadioButtonAnimState` tracks `dot_t` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`).

### Switch
Compact toggle with a sliding knob.
- **Constructor**: `Switch::new("switch_id")`
- **Builder methods**: `.style(SwitchStyle)`, `.default_enabled(bool)`, `.size([w, h])`
- **Reading/writing state**: `switch.enabled(ui) -> bool`, `switch.set_enabled(ui, bool)`
- **Animation**: `SwitchState` tracks `anim_progress` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`).

### Slider
Continuous numeric range control.
- **Constructor**: `Slider::new("slider_id", min, max)`
- **Builder methods**: `.width(px)` / `.height(px)` / `.size([w, h])`, `.style(SliderStyle)`, `.default_value(f32)`
- **Reading/writing state**: `slider.value(ui) -> f32`, `slider.set_value(ui, f32)`
- **Animation**: `SliderState` tracks `hover_t` (`Motion::SNAPPY`) and `drag_t` (`Motion::INSTANT`). Thumb grows 2px while dragging.

### TextInput
Single-line text field.
- Click/drag selection, double-click word select, triple-click select all
- Clipboard: `Ctrl+C` / `Ctrl+V` / `Ctrl+A` / `Ctrl+X`
- Blinking cursor, auto-scroll on overflow
- **Constructor**: `TextInput::new("input_id")`
- **Builder methods**:
  - `.width(px)` / `.height(px)` / `.size([w, h])`
  - `.placeholder("...")` — shown in `Theme::TEXT_MUTED` when empty
  - `.default_text("...")` — initial value
  - `.style(TextInputStyle)`
- **Reading/writing state**: `input.text(ui) -> String`, `input.set_text(ui, String)`
- **Animation**: focus ring via `Motion::GENTLE` — border grows 0.5px, glow shadow expands 6px. Hover border via `Motion::SNAPPY`.

### TextArea
Multi-line editor with scrolling.
- Vertical scroll with interactive scrollbar
- Arrow-key navigation with column memory, `Enter` for newlines
- **Constructor**: `TextArea::new("area_id")`
- **Builder methods**: `.width(px)` / `.height(px)` / `.size([w, h])`, `.default_text("...")`, `.style(TextAreaStyle)`
- **Reading/writing state**: `area.text(ui) -> String`, `area.set_text(ui, String)`
- **Animation**: same focus ring as `TextInput`.

### SelectBox
Combobox: a trigger button that opens a spring-animated floating dropdown.
- Keyboard navigation (↑/↓, `Enter` to select, `Escape` to close), click-outside-to-close
- Optional type-to-filter search bar, mouse-wheel scroll through long option lists
- **Constructor**: `SelectBox::new("select_id", vec![SelectOption::new("value", "Label"), ...])`
- **Builder methods**: `.placeholder("...")`, `.width(px)`, `.searchable()`, `.style(SelectBoxStyle)`, `.tooltip("...")`
- **Reading/writing state**: `select.selected(ui) -> Option<String>`, `select.set_selected(ui, Option<String>)`
- **Animation**: open/close spring (stiffness 380, damping 30) drives the dropdown's slide and the chevron's 180° rotation; per-item hover fades via `Motion::SNAPPY`.

---

## 2. Containers

### Card
Surface with rounded corners, padding, border, and an optional drop shadow.
- **Constructor**: `Card::new(&mut child_widget)`
- **Variants**: `.subtle()` (inset, `Theme::SURFACE_SUBTLE`), `.elevated()` (floating, deeper shadow)
- **Builder methods**: `.id("...")`, `.padding([x, y])`, `.size([w, h])`, `.style(CardStyle)`

### ScrollView
Scrolling container with a draggable scrollbar.
- **Constructor**: `ScrollView::new("scroll_id", &mut child)`
- **Builder methods**: `.size([w, h])`, `.padding([x, y])`, `.default_offset([x, y])`, `.style(ScrollViewStyle)`
- **Reading/writing state**: `scroll.offset(ui) -> [f32; 2]`, `scroll.set_offset(ui, [f32; 2])`

### Container
Fixed-size wrapper around a child widget.
- **Constructor**: `Container::new(&mut child)`
- **Builder methods**: `.id("...")`, `.size([w, h])`, `.padding([x, y])`

### Divider
Separator line.
- **Constructor**: `Divider::horizontal(length)` / `Divider::vertical(length)`
- **Builder methods**:
  - `.id("...")` — only needed if you want it in the accessibility tree
  - `.faint()` — sets the border to `Theme::BORDER_FAINT`
  - `.thickness(px)`, `.color(Color)`

---

## 3. Displays

### Label
Text, using the bundled Geist / Geist Mono fonts.
- **Constructor**: `Label::new("label_id", "Text")` or `Label::new("label_id", format!("Count: {n}"))`
- **Color variants**: `.secondary()`, `.muted()`, `.accent()`, `.success()`, `.warning()`, `.error()`, `.color(Color)`
- **Size presets**: `.caption()` (12px), `.subheading()` (16px), `.heading()` (18px), `.title()` (22px), `.metric()` (28px, bold)
- **Weight**: `.medium()`, `.semibold()`, `.bold()`
- **Font**: `.mono()` switches to Geist Mono

### Badge
Status pill.
- **Constructor**: `Badge::new("Text")`
- **Variants**: `.secondary()`, `.outline()`, `.success()`, `.warning()`, `.error()`
- **Builder methods**: `.id("...")`, `.size([w, h])`, `.padding([x, y])`, `.style(BadgeStyle)`

### ProgressBar
Completion bar.
- **Constructor**: `ProgressBar::new(ratio)` where `ratio` is `0.0..=1.0`
- **Variants**: `.success()`, `.warning()`, `.error()`
- **Builder methods**: `.id("stable_id")`, `.size([w, h])`, `.style(ProgressBarStyle)`
- **Animation**: `ProgressBarState` animates `animated_progress` via `Motion::FLUID` — but only if you gave it a stable `.id(...)`. Without one it just snaps to the raw ratio each frame.
