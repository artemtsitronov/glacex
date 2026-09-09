# Glacex Widget Reference

Glacex 0.2.0 also includes lightweight display and interaction primitives for common product UI:
`Separator`, `Skeleton`, `Spinner`, `Kbd`, `Avatar`, `Alert`, `Empty`, `Toggle`, `Tabs`, and `Accordion`. They implement
`Widget` + `Measurable`, accept builder-style configuration, and compose with the
existing `row!`, `column!`, and `Card` layout APIs.

```rust
let mut status = Alert::new("Build completed").variant(AlertVariant::Success);
let mut shortcut = Kbd::new("⌘ K");
let mut loading = Skeleton::new([180.0, 16.0]);
let mut tabs = Tabs::new("settings-tabs", ["General", "Members", "Billing"]);
let mut details = Accordion::new("details", "Deployment details", "Built from main in 42 seconds.");
let mut dialog = Dialog::new("confirm", "Publish release?", "This will make the current build available to your team.");
let mut toast = Toast::new("saved", "Changes saved");
let mut heading = Typography::new("Project settings").variant(TypographyVariant::Heading);
let mut field = Field::new("Repository name").description("Use a short, recognizable name.");
let mut group = ButtonGroup::new("view", ["List", "Board", "Timeline"]);
let mut crumbs = Breadcrumb::new(["Projects", "Glacex", "Settings"]);
let mut pages = Pagination::new("projects-pages", 8);
let mut table = Table::new("projects", ["Name", "Status"], [["glacex", "Ready"], ["docs", "Draft"]]);
let mut carousel = Carousel::new("release-notes", ["Fast", "Native", "Accessible"]);
let mut chart = Chart::new([12.0, 24.0, 18.0, 32.0, 28.0]);
let mut commands = CommandPalette::new("command-menu", [
    Command::new("new-project", "New project").hint("⌘ N"),
    Command::new("settings", "Open settings"),
]);
let mut calendar = Calendar::new("release-date", CalendarDate { year: 2026, month: 9, day: 9 });
let mut otp = InputOtp::new("verification-code", 6);
let mut toggles = ToggleGroup::new("view-mode", ["List", "Board", "Timeline"]);
let mut menu = DropdownMenu::new("actions", "Actions", [
    MenuItem::new("rename", "Rename"),
    MenuItem::new("archive", "Archive"),
]);
let mut sheet = Sheet::new("details", "Deployment details", "Logs and metadata").side(SheetSide::Right);
let mut sidebar = Sidebar::new("app-nav", ["Overview", "Projects", "Settings"]);
let mut split = Resizable::new("main-split", 720.0).limits(0.25, 0.75);
let mut details = HoverCard::new("Build #1042", "Completed successfully 2 minutes ago.");
let mut row = Item::new("Production deploy").description("2 minutes ago · main");
let mut bubble = Bubble::new("Build completed").outgoing(true);
let mut file = Attachment::new("release-notes.md").meta("14 KB · Markdown");
let mut marker = Marker::new("Beta");
let mut grouped_input = InputGroup::new("repository", "Search repositories…")
    .prefix("⌕")
    .suffix("⌘ K");
let mut popover = Popover::new("release-info", "Release notes", "Everything is ready to publish.");
let mut questionnaire = Questionnaire::new("onboarding", [
    Question::new("How do you ship?", ["Preview", "Production"]),
    Question::new("What matters most?", ["Speed", "Control"]),
]);
```

API reference for all widgets provided by `glacex`.

The 0.2 component surface includes all 64 requested public names. A handful of
names are deliberate compatibility aliases over the mature native implementation:
`DataTable`/`Table`, `Collapsible`/`Accordion`, `Input`/`TextInput`,
`InputOTP`/`InputOtp`, `Select`/`SelectBox`, and `ScrollArea`/`ScrollView`.
This keeps state, keyboard behavior, and accessibility consistent instead of
shipping duplicate controls with subtly different semantics.

Interactive disclosure, skeleton, spinner, progress, selection, and overlay
components use frame time and the shared motion helpers. Motion is deliberately
short and low-amplitude; the library avoids decorative animation that competes
with content.

Components resolve their visual styles from the active `Theme` on every frame.
Dark presets use semantic surface and foreground tokens, and active controls
choose a contrasting foreground automatically. Switch thumbs, radio dots, and
checkbox marks remain legible when the active surface is light or dark.
Danger actions, sliders, badges, bubbles, and markers use the same contrast-aware
foreground rules instead of fixed black or white text.

When a layout container is used directly, it measures its natural content and
centers itself in the window. Use explicit `arrange_at` coordinates when building
anchored or custom layouts.

`Theme::all()` returns the built-in presets in presentation order: light presets
first, followed by dark presets. The main demo follows this order and starts with
the light theme for a neutral first-run preview.

## Runnable galleries

The repository includes small, focused examples so component families can be
reviewed without navigating one oversized screen:

```bash
cargo run --example foundations  # controls, forms, feedback, typography
cargo run --example data         # tabs, charts, tables, pagination, calendar
cargo run --example overlays     # command, menu, dialog, sheet, popover, tooltip
```

`components` remains the compact mixed gallery, while `demo` is the composed
dashboard example.

## Text and Nerd Font icons

`NerdIcon` provides stable common Nerd Font glyphs without requiring callers to
paste private-use-area code points:

```rust
let search = NerdIcon::Search.mono();
ui.draw_text_with_style(
    search,
    [16.0, 16.0],
    [16.0, 16.0, 40.0, 40.0],
    ui.theme().text_primary,
    16.0,
    20.0,
    TextStyle::default().mono(true),
);
```

The complete upstream catalog is available through `NerdIcon::named("name")`
and `NerdIcon::all()`. Names and codepoints are generated from the bundled
Nerd Fonts `glyphnames.json` catalog, so the full icon set is available without
adding a dependency or manually copying glyph characters. Icons are ordinary
colored text: use `Ui::draw_nerd_icon` or `draw_text_with_style` with any theme
color, opacity, and size.

`TextStyle` supports Geist or Nerd Mono selection, regular/medium/semibold/bold
weights, italic, underline, and strikethrough. The bundled Nerd Mono family is
used by `.mono()` and by `Kbd`, so keyboard labels and icon glyphs share the same
font metrics.

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
Text uses the bundled Geist family. `.mono()` uses the bundled
`GeistMono Nerd Font Mono` family, including the regular, medium, semibold, and
bold weights with Nerd Font icon glyph coverage.
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

---

## 4. Inputs (Continued)

### SelectBox (Combobox)

A spring-animated floating dropdown for selecting a single value from a list. Matches shadcn/ui and Apple HIG quality — soft neumorphic styling with no visible borders, shadow-based depth, and responsive spring physics.

- **Constructor**: `SelectBox::new("id", options)` where `options: Vec<SelectOption>`
- **Builder methods**:
  - `.placeholder("Choose…")` — text shown when nothing is selected
  - `.width(220.0)` — trigger button width
  - `.searchable()` — adds a type-to-filter search bar at the top of the dropdown
  - `.show_clear(true)` — adds a × button to clear the selection
  - `.style(SelectBoxStyle)` — full visual override
  - `.tooltip("hint text")` — tooltip on trigger hover
- **Creating options**: `SelectOption::new("value", "Display Label")`
- **Reading the selection**: After `arrange`, read state via `ui.widget_state::<SelectBoxState>("id").selected`
- **Keyboard navigation**: ↑/↓ to move, Enter to confirm, Escape to close
- **Interaction model**: clicking the trigger toggles open/closed; clicking outside the dropdown closes it; pushing a `push_input_block` internally protects underlying widgets from receiving clicks while the dropdown is visible
- **Design**: zero border-width by default, large diffuse drop shadow for depth, 14px corner radius, subtle surface fill, smooth spring animations (stiffness 420, damping 28)
