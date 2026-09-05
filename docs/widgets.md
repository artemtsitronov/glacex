# Glacex Widget Reference

API reference for all widgets provided by `glacex`.

## 1. Controls

### Button
Interactive push button. Hover and press states animate smoothly via `ButtonState`.
- **Constructor**: `Button::new("Label")`
- **Builder methods**:
  - `.style(ButtonStyle)` — customize fill, hover fill, pressed fill, border, shadow
  - `.tooltip("text")` — floating tooltip on hover
- **Queries**:
  - `btn.clicked() -> bool`
  - `btn.hovered() -> bool`
  - `btn.pressed() -> bool`
- **Animation**: `ButtonState` stores `hover_t` and `press_t` (0.0..=1.0), interpolated each frame with `animate_towards`. Fill blends from rest to hover to pressed with no jarring jumps.

### Checkbox
Boolean toggle with an animated diagonal tick that draws stroke-by-stroke when checked.
- **Constructor**: `Checkbox::new("checkbox_id")`
- **Builder methods**: `.style(CheckboxStyle)`, `.default_checked(bool)`
- **Reading state**:
  ```rust
  let is_checked = ui.widget_state::<CheckboxState>("checkbox_id").checked;
  ```
- **Animation**: `CheckboxState` stores `anim_progress` (0.0..=1.0). The tick draws segment-by-segment as progress climbs, giving a clean stroke-on effect. The background fill cross-fades between idle and checked colors.

### RadioButton
Mutually exclusive selector within a named group. Inner dot scales in on selection.
- **Constructor**: `RadioButton::new("group_id", "option_id")`
- **Reading state**:
  ```rust
  let selected = ui.selected_option("group_id");
  ```
- **Animation**: `RadioButtonAnimState` tracks `dot_t` (dot scale) and `hover_t`. The inner dot grows in from zero radius, and the background blends from idle to hover to selected.

### Switch
Compact toggle. Knob glides across the track and the track color cross-fades on toggle.
- **Constructor**: `Switch::new("switch_id")`
- **Builder methods**: `.style(SwitchStyle)`, `.default_enabled(bool)`
- **Reading state**:
  ```rust
  let enabled = ui.widget_state::<SwitchState>("switch_id").enabled;
  ```
- **Animation**: `SwitchState` stores `anim_progress` (0.0..=1.0). The knob X position and track fill both interpolate smoothly via `animate_towards`.

### Slider
Continuous numerical range control. Thumb shows a soft glow halo on hover and drag.
- **Constructor**: `Slider::new("slider_id", min, max, width)`
- **Builder methods**: `.style(SliderStyle)`, `.default_value(f32)`
- **Reading state**:
  ```rust
  let val = ui.widget_state::<SliderState>("slider_id").value;
  ```
- **Animation**: `SliderState` stores `hover_t`. A soft radial glow ring behind the thumb expands and fades with hover/drag state.

### TextInput
Single-line text field.
- Supports click and drag selection, double-click word select, triple-click select all
- Clipboard shortcuts: `Ctrl+C`, `Ctrl+V`, `Ctrl+A`, `Ctrl+X`
- Cursor blink and auto-scrolling
- **Constructor**: `TextInput::new("input_id", width)`
- **State access**:
  ```rust
  let state = ui.widget_state::<TextEditState>("input_id");
  let text = state.text();
  state.set_text("New value");
  ```

### TextArea
Multi-line editor.
- Vertical scrolling with interactive scrollbar
- Arrow key navigation with column memory
- Enter for line breaks, text selection and clipboard operations
- **Constructor**: `TextArea::new("area_id", width, height)`

---

## 2. Containers

### Card
Elevated surface with rounded corners, configurable padding, border, and soft drop shadow.
- **Constructor**: `Card::new(&mut child_widget)`
- **Style**: `CardStyle` (`padding`, `corner_radius`, `fill`, `border_width`, `border_color`, `shadow`)

### ScrollView
Dual-axis scrolling container with draggable scrollbars.
- **Constructor**: `ScrollView::new("scroll_id", [viewport_w, viewport_h], &mut child)`

### Container
Explicit fixed-size wrapper around a child widget.
- **Constructor**: `Container::new([width, height], &mut child)`

### Divider
Separation rule for dividing layout sections.
- **Horizontal**: `Divider::horizontal(width)`
- **Vertical**: `Divider::vertical(height)`

---

## 3. Displays

### Label
Plain or dynamic text.
- **Constructor**: `Label::new("Text")` or `Label::new(format!("Count: {n}"))`

### Badge
Compact semantic status pill.
- **Constructor**: `Badge::new("ACTIVE")`
- **Variants**: `BadgeVariant::Default`, `Outline`, `Success`, `Warning`, `Error`

### ProgressBar
Filled percentage track.
- **Constructor**: `ProgressBar::new(ratio, width)` where `ratio` is `0.0..=1.0`
