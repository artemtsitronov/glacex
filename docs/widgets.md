# Glacex Widget Reference

API reference for all widgets provided by `glacex`.

## 1. Controls

### Button
Interactive push button with smooth hover/press physics and tactile elevation feedback.
- **Constructor**: `Button::new("Label")`
- **Builder methods**:
  - `.style(ButtonStyle)` -- customize fill, hover fill, pressed fill, text color, border, shadow
  - `.primary()` -- electric indigo CTA style (`Theme::ACTIVE`)
  - `.outline()` -- transparent fill with prominent border (`Theme::BORDER_STRONG`)
  - `.ghost()` -- borderless flat button highlighting on hover
  - `.danger()` -- destructive action style (`Theme::ERROR`)
  - `.tooltip("text")` -- floating tooltip on hover
- **Queries**:
  - `btn.clicked() -> bool`
  - `btn.hovered() -> bool`
  - `btn.pressed() -> bool`
- **Animation**: `ButtonState` stores `hover_t` and `press_t` (0.0..=1.0), animated with `Motion::SNAPPY` (hover, 45ms) and `Motion::INSTANT` (press, 30ms). On press, the button shifts 1px down and the shadow blur compresses to simulate physical elevation. Border brightens to `Theme::BORDER_STRONG` on hover.

### Checkbox
Boolean toggle with an animated diagonal tick that draws stroke-by-stroke when checked.
- **Constructor**: `Checkbox::new("checkbox_id")`
- **Builder methods**: `.style(CheckboxStyle)`, `.default_checked(bool)`
- **Reading state**:
  ```rust
  let is_checked = ui.widget_state::<CheckboxState>("checkbox_id").checked;
  ```
- **Animation**: `CheckboxState` stores `anim_progress` (0.0..=1.0) and `hover_t`, driven by `Motion::SNAPPY`. The tick draws progressively: the left leg (0..35%) and right leg (30..100%) overlap slightly for a fluid stroke-on feel. Border transitions toward `Theme::ACTIVE` when checked and `Theme::BORDER_STRONG` on hover.

### RadioButton
Mutually exclusive selector within a named group. Inner dot scales in on selection.
- **Constructor**: `RadioButton::new("group_id", "option_id")`
- **Reading state**:
  ```rust
  let selected = ui.selected_option("group_id");
  ```
- **Animation**: `RadioButtonAnimState` tracks `dot_t` (dot scale, `Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`). The inner dot grows in from zero radius and the background blends idle/hover/selected. Border transitions toward `Theme::ACTIVE` on selection and `Theme::BORDER_STRONG` on hover.

### Switch
Compact toggle. Knob glides across the track and the track color cross-fades on toggle.
- **Constructor**: `Switch::new("switch_id")`
- **Builder methods**: `.style(SwitchStyle)`, `.default_enabled(bool)`
- **Reading state**:
  ```rust
  let enabled = ui.widget_state::<SwitchState>("switch_id").enabled;
  ```
- **Animation**: `SwitchState` stores `anim_progress` (`Motion::FLUID`) and `hover_t` (`Motion::SNAPPY`). The knob X position and track fill blend across idle/hover/active using a 3-way color mix. A micro shadow beneath the knob gives it physical lift. Border blends toward `Theme::ACTIVE * 0.5` when enabled.

### Slider
Continuous numerical range control. Thumb shows a soft glow halo on hover and drag.
- **Constructor**: `Slider::new("slider_id", min, max, width)`
- **Builder methods**: `.style(SliderStyle)`, `.default_value(f32)`
- **Reading state**:
  ```rust
  let val = ui.widget_state::<SliderState>("slider_id").value;
  ```
- **Animation**: `SliderState` stores `hover_t` (`Motion::SNAPPY`) and `drag_t` (`Motion::INSTANT`). The thumb scales up 2px while dragging. The hover glow halo expands to 10px radius and the drop shadow grows during drag for tactile elevation feedback.

### TextInput
Single-line text field with selection, cursor animation, and placeholder text.
- Supports click and drag selection, double-click word select, triple-click select all
- Clipboard shortcuts: `Ctrl+C`, `Ctrl+V`, `Ctrl+A`, `Ctrl+X`
- Cursor blink and auto-scrolling
- **Constructor**: `TextInput::new("input_id", width)`
- **Builder methods**:
  - `.placeholder("Enter text...")` -- placeholder shown in `Theme::TEXT_MUTED` when empty
  - `.default_text("...")` -- initial text value
  - `.style(TextInputStyle)` -- customize fill, text color, placeholder color, borders, shadows
- **State access**:
  ```rust
  let state = ui.widget_state::<TextEditState>("input_id");
  let text = state.text();
  state.set_text("New value");
  ```
- **Animation**: `TextEditState` stores `focus_t` and `hover_t`. Focus ring animates in via `Motion::GENTLE` (90ms): border width grows 0.5px, glow shadow blur expands 6px, and glow color fades in. Hover border highlights via `Motion::SNAPPY`.

### TextArea
Multi-line editor with vertical scrolling and focus-visible glow ring.
- Vertical scrolling with interactive scrollbar
- Arrow key navigation with column memory
- Enter for line breaks, text selection and clipboard operations
- **Constructor**: `TextArea::new("area_id", width, height)`
- **Animation**: Same animated focus ring as `TextInput` via `TextEditState.focus_t` and `Motion::GENTLE`.

---

## 2. Containers

### Card
Elevated surface with rounded corners, configurable padding, border, and soft drop shadow.
- **Constructor**: `Card::new(&mut child_widget)`
- **Builder methods**:
  - `.subtle()` -- inset surface (`Theme::SURFACE_SUBTLE`) with faint hairline border
  - `.elevated()` -- floating card (`Theme::SURFACE_ELEVATED`) with deep shadow
  - `.style(CardStyle)` -- custom fill, border, corner radius, padding, and shadow

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
- **Builder methods**:
  - `.faint()` -- sets the border to ultra-subtle hairline (`Theme::BORDER_FAINT`)
  - `.thickness(px)` -- custom stroke thickness
  - `.color(Color)` -- custom stroke color

---

## 3. Displays

### Label
Typography text powered by authentic bundled Geist & Geist Mono fonts with semantic hierarchy, scale, and weight customization.
- **Constructor**: `Label::new("Text")` or `Label::new(format!("Count: {n}"))`
- **Variants**:
  - `.primary()` -- high-contrast body text in `Theme.text_primary`
  - `.secondary()` -- supporting caption in `Theme.text_secondary`
  - `.muted()` -- subdued/dim text in `Theme.text_muted`
  - `.accent()` -- primary action accent in `Theme.active`
  - `.success()` -- emerald success text in `Theme.success`
  - `.warning()` -- amber warning text in `Theme.warning`
  - `.error()` -- rose error text in `Theme.error`
  - `.color(Color)` -- explicit custom text color
- **Typography Sizing & Weights**:
  - `.size(px)` -- custom font size in pixels (line height automatically scaled)
  - `.caption()` -- 12px micro metadata
  - `.subheading()` -- 16px section subtitle
  - `.heading()` -- 18px card / section title
  - `.title()` -- 22px display header
  - `.metric()` -- 28px bold KPI metric display (shadcn stat cards)
  - `.medium()` -- 500 font weight
  - `.semibold()` -- 600 font weight
  - `.bold()` -- 700 font weight
  - `.mono()` -- switches font to Geist Mono

### Badge
Compact semantic status pill with role-tinted surfaces, 6px pill radius, and Geist Medium typography.
- **Constructor**: `Badge::new("ACTIVE")`
- **Variants**:
  - `Badge::new("...").default()` -- neutral dark / active badge
  - `Badge::new("...").secondary()` -- zinc subtle background with secondary text
  - `Badge::new("...").outline()` -- transparent fill with crisp border
  - `Badge::new("...").success()` -- emerald text and border with 12% alpha surface
  - `Badge::new("...").warning()` -- amber text and border with 12% alpha surface
  - `Badge::new("...").error()` -- rose text and border with 12% alpha surface

### ProgressBar
Filled percentage track with sleek 6px height and smooth animated fill.
- **Constructor**: `ProgressBar::new(ratio, width)` where `ratio` is `0.0..=1.0`
- **Builder methods**:
  - `.id("stable_id")` -- required to enable smooth fill animation across frames
  - `.success()` -- emerald success fill
  - `.warning()` -- amber warning fill
  - `.error()` -- rose error fill
- **Animation**: `ProgressBarState` tracks `animated_progress` via `Motion::FLUID`. Without a stable `.id()`, the bar renders at the raw ratio with no animation.
