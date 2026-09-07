# Glacex Widget Reference

API reference for all widgets provided by `glacex`.

Most widgets take a stable string id as their first constructor argument. That id is what
`Ui::widget_state` uses to find the right persistent state across frames, and it's also what
shows up in the accessibility tree (see [Accessibility](../README.md#accessibility)) when
one is enabled. `Card`, `Container`, `Divider`, and `ProgressBar` don't require an id up
front — call `.id("...")` on them if you need stable state or want that instance to be
addressable by assistive tech; otherwise they're fine left anonymous.

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
- **Reading state**:
  ```rust
  let is_checked = ui.widget_state::<CheckboxState>("checkbox_id").checked;
  ```
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
- **Reading state**:
  ```rust
  let enabled = ui.widget_state::<SwitchState>("switch_id").enabled;
  ```
- **Animation**: `SwitchState` tracks `anim_progress` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`).

### Slider
Continuous numeric range control.
- **Constructor**: `Slider::new("slider_id", min, max)`
- **Builder methods**: `.width(px)` / `.height(px)` / `.size([w, h])`, `.style(SliderStyle)`, `.default_value(f32)`
- **Reading state**:
  ```rust
  let val = ui.widget_state::<SliderState>("slider_id").value;
  ```
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
- **State access**:
  ```rust
  let state = ui.widget_state::<TextEditState>("input_id");
  let text = state.text();
  state.set_text("New value");
  ```
- **Animation**: focus ring via `Motion::GENTLE` — border grows 0.5px, glow shadow expands 6px. Hover border via `Motion::SNAPPY`.

### TextArea
Multi-line editor with scrolling.
- Vertical scroll with interactive scrollbar
- Arrow-key navigation with column memory, `Enter` for newlines
- **Constructor**: `TextArea::new("area_id")`
- **Builder methods**: `.width(px)` / `.height(px)` / `.size([w, h])`, `.default_text("...")`, `.style(TextAreaStyle)`
- **Animation**: same focus ring as `TextInput`.

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
