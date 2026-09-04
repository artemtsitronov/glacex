# Glacex Widget Reference

API reference for widgets provided by `glacex`.

## 1. Controls

### Button
Interactive push button with hover, pressed, and clicked interaction states.
- **Constructor**: `Button::new("Label")`
- **Builder Methods**:
  - `.style(ButtonStyle)`
  - `.tooltip("Helpful text")`
- **Queries**:
  - `btn.clicked() -> bool`
  - `btn.hovered() -> bool`
  - `btn.pressed() -> bool`

### Checkbox
Stateful boolean toggle widget. State persists in `Ui`.
- **Constructor**: `Checkbox::new("checkbox_id")`
- **Style**: `CheckboxStyle`
- **Reading State**:
  ```rust
  let is_checked = ui.widget_state::<CheckboxState>("checkbox_id").checked;
  ```

### RadioButton
Mutually exclusive selector within a shared group.
- **Constructor**: `RadioButton::new("group_id", "option_id")`
- **Reading State**:
  ```rust
  let selected = ui.selected_option("group_id");
  ```

### Switch
Compact toggle control.
- **Constructor**: `Switch::new("switch_id")`
- **Reading State**:
  ```rust
  let enabled = ui.widget_state::<SwitchState>("switch_id").enabled;
  ```

### Slider
Continuous numerical slider with a draggable knob and active track fill.
- **Constructor**: `Slider::new("slider_id", min_val, max_val, width)`
- **Reading State**:
  ```rust
  let val = ui.widget_state::<SliderState>("slider_id").value;
  ```

### TextInput
Single-line text field supporting:
- Mouse click and drag selection
- Double click for word selection, triple click for select all
- Clipboard shortcuts (`Ctrl+C`, `Ctrl+V`, `Ctrl+A`)
- Cursor blinking and auto-scrolling
- **Constructor**: `TextInput::new("input_id", width)`
- **State Access**:
  ```rust
  let state = ui.widget_state::<TextEditState>("input_id");
  let text = state.text();
  state.set_text("New Text");
  ```

### TextArea
Multi-line text editor supporting:
- Vertical scrolling with interactive scrollbar
- Arrow navigation (`Up`/`Down`) with column memory
- Enter key line-breaks
- Text selection and clipboard operations
- **Constructor**: `TextArea::new("area_id", width, height)`

## 2. Containers

### Card
Surface container with rounded corners, padding, border, and soft drop shadow.
- **Constructor**: `Card::new(&mut child_widget)`
- **Style**: `CardStyle` (`padding`, `corner_radius`, `fill`, `shadow`).

### ScrollView
Dual-axis scrolling container with draggable scrollbars.
- **Constructor**: `ScrollView::new("scroll_id", [viewport_w, viewport_h], &mut child)`

### Container
Explicit fixed-size wrapper around a child widget.
- **Constructor**: `Container::new([width, height], &mut child)`

### Divider
Separation line rule for dividing layout sections.
- **Horizontal**: `Divider::horizontal(width)`
- **Vertical**: `Divider::vertical(height)`

## 3. Displays

### Label
Plain or formatted text widget.
- **Constructor**: `Label::new("Text")`

### Badge
Compact status badge.
- **Constructor**: `Badge::new("ACTIVE")`
- **Variants**: `BadgeVariant::Default`, `Outline`, `Success`, `Warning`, `Error`.

### ProgressBar
Progress bar with filled percentage track.
- **Constructor**: `ProgressBar::new(progress_ratio, width)` (`progress_ratio`: `0.0..=1.0`).
