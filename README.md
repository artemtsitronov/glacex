<div align="center">

![demo](screenshots/demo.png)

# glacex

<p align="center">
  <a href="https://crates.io/crates/glacex"><img src="https://img.shields.io/crates/v/glacex?style=for-the-badge&logo=rust&logoColor=cdd6f4&label=crates.io&labelColor=181825&color=cba6f7" alt="Crates.io Version"></a>
  <a href="https://docs.rs/glacex"><img src="https://img.shields.io/docsrs/glacex?style=for-the-badge&logo=docsdotrs&logoColor=cdd6f4&label=docs.rs&labelColor=181825&color=89b4fa" alt="docs.rs"></a>
  <a href="https://github.com/artemtsitronov/glacex/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-a6e3a1?style=for-the-badge&logo=opensourceinitiative&logoColor=cdd6f4&labelColor=181825" alt="License"></a>
  <img src="https://img.shields.io/badge/rustc-1.85+-fab387?style=for-the-badge&logo=rust&logoColor=cdd6f4&labelColor=181825" alt="Rustc Version">
  <img src="https://img.shields.io/badge/wgpu-30.0-f38ba8?style=for-the-badge&logo=webgpu&logoColor=cdd6f4&labelColor=181825" alt="wgpu 30.0">
  <a href="https://github.com/artemtsitronov/glacex/stargazers"><img src="https://img.shields.io/github/stars/artemtsitronov/glacex?style=for-the-badge&logo=github&logoColor=cdd6f4&label=stars&labelColor=181825&color=f9e2af" alt="GitHub Stars"></a>
</p>

GPU-accelerated, immediate-mode UI library built from scratch in Rust on top of `wgpu`, `winit`, and `taffy`.

Built by **Artem Tsitronov** and **Soumalya Das**.

</div>

> **Status**: early and actively changing. Expect breaking API changes between 0.x releases.

## Table of Contents

- [What is glacex?](#what-is-glacex)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
  - [App](#app)
  - [Widget & Measurable](#widget-and-measurable)
  - [Ui Context](#ui)
  - [Layout](#layout)
- [Widgets](#widgets)
  - [Controls](#controls)
  - [Containers](#containers)
  - [Displays](#displays)
- [Styling & Theming](#styling)
  - [Style Structs](#style-structs)
  - [ShadowStyle](#shadowstyle)
  - [Color Type](#color)
  - [Fill & Gradients](#fill-and-gradients)
  - [Theme Palette](#theme)
  - [Window Control](#window-title-and-background)
- [Accessibility](#accessibility)
- [How Rendering Works](#how-rendering-works)
- [Examples](#examples)
- [Project Layout](#project-layout)
- [Known Limitations](#known-limitations)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## What is glacex?

`glacex` is an immediate-mode UI library for Rust that draws its own pixels instead of wrapping a native toolkit or a browser engine:

- **`winit`** owns the window and the cross-platform event loop.
- **`wgpu`** renders every shape as an instanced signed-distance-field (SDF) quad on the GPU.
- **`glyphon`** (+ `swash`) shapes and rasterizes text into glyph atlases with per-widget clipping.
- **`taffy`** does the flexbox math for `row!`/`column!` layouts.

There's no retained widget tree and no markup — you describe the UI in plain Rust every frame, and glacex measures, lays out, hit-tests, animates, and renders it.

## Features

- Custom renderer: instanced rounded rects (anti-aliased SDF), borders, and soft drop shadows, batched into one draw call per shared clip rect.
- Text rendering via `glyphon` with independent clip bounds per widget.
- Fills: solid colors and gradients (linear, radial, conic), cached into a GPU atlas.
- `Color` is `#[repr(C)]` + `Pod`/`Zeroable`, so it maps straight onto GPU vertex buffers. Hex, RGB, HSV, alpha blending, `lerp`, lighten/darken.
- Cursor changes (pointer, text, resize, default) driven by hover state.
- A floating tooltip layer that clamps to the viewport.
- Widgets: `Button`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `ProgressBar`, `TextInput`, `TextArea`, `ScrollView`, `Card`, `Container`, `Badge`, `Divider`, `Label`, `SelectBox`.
- `row![]` / `column![]` macros backed by `taffy`, with alignment and spacing.
- Interaction: hover/press/click, secondary/middle mouse buttons, Tab/Shift+Tab focus order, double/triple-click word/line selection, clipboard via `arboard`, blinking cursor.
- Widget state persists across frames keyed by a stable string id (`Ui::widget_state`, `take_widget_state`, `put_widget_state`) even though the widget itself is rebuilt every frame.
- Animation via exponential decay (`animate_towards`) plus a small set of easing curves and spring presets, unified under named half-life constants (`Motion::INSTANT`/`SNAPPY`/`FLUID`/`GENTLE`) so transitions feel consistent across widgets.
- Draggable auto-hiding scrollbars shared by `ScrollView` and `TextArea`.
- Optional accessibility tree (AT-SPI on Linux via `accesskit`) — see [Accessibility](#accessibility).

## Requirements

- Rust 1.85+ (2024 edition).
- A GPU/driver backend `wgpu` supports (Vulkan, Metal, DirectX 12, or OpenGL ES).
- On Linux: a running Wayland or X11 session. Headless environments need a virtual display (e.g. `xvfb`) to create a window surface.

## Installation

### From crates.io

```bash
cargo add glacex
```

or in `Cargo.toml`:

```toml
[dependencies]
glacex = "0.1.6"
```

### From GitHub (main branch)

```toml
[dependencies]
glacex = { git = "https://github.com/artemtsitronov/glacex.git", branch = "main" }
```

### Linux build dependencies

```bash
# Debian / Ubuntu
sudo apt install libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxkbcommon-dev libwayland-dev

# Fedora
sudo dnf install libX11-devel libXcursor-devel libXrandr-devel libXi-devel libxkbcommon-devel wayland-devel

# Arch Linux
sudo pacman -S libx11 libxcursor libxrandr libxi libxkbcommon wayland
```

## Quick Start

A minimal counter:

```rust
use glacex::{App, Button, Color, Label, Ui, Widget, column};

struct Counter {
    count: u32,
}

impl Widget for Counter {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(18, 18, 22));

        let mut label = Label::new("count_label", format!("Count: {}", self.count));
        let mut button = Button::new("increment_btn", "Increment");

        column![&mut label, &mut button]
            .spacing(12.0)
            .arrange_at([40.0, 40.0], ui);

        if button.clicked() {
            self.count += 1;
        }
    }
}

fn main() {
    App::new(Counter { count: 0 }).run();
}
```

`App::new(root).run()` opens a window, sets up the GPU pipelines, and runs the event loop.

## Core Concepts

### App

`App<W: Widget>` owns the `winit` window and event loop:

```rust
App::new(root_widget)
    .update(|root| {
        // Runs once per frame before rendering. Good place for
        // app-level state changes based on the previous frame's input.
    })
    .run();
```

### Widget and Measurable

Every UI element implements `Widget`:

```rust
pub trait Widget {
    type Output;
    fn ui(&mut self, ui: &mut Ui) -> Self::Output;
}
```

Widgets that can be placed inside `row!`/`column!` also implement `Measurable`:

```rust
pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;
}
```

- `measure` returns the widget's natural size.
- `arrange` does hit-testing, input handling, and issues draw calls at the resolved rect.

### Ui

`Ui` is the per-frame context passed into every widget:
- Input: `mouse_position()`, `mouse_pressed()`, `mouse_right_pressed()`, `click_count()`, `key_pressed()`, `ctrl_held()`, `shift_held()`.
- Cursor: `set_cursor_icon(CursorIcon)`.
- Tooltips: `show_tooltip(text)`, `show_tooltip_at(text, pos)`.
- State: `widget_state::<T>(id)`, `take_widget_state::<T>(id)`, `put_widget_state(id, state)`.
- Focus: `request_focus(id)`, `is_focused(id)`, `advance_focus(backward)`.
- Clipping: `push_clip(rect)`, `pop_clip()`, `push_input_block(rect)`.
- Drawing: `draw_rect(...)`, `draw_text(...)`, `measure_text(...)`, `line_height()`.
- Window: `set_title(&str)`, `set_bgcolor(Color)`.

### Layout

```rust
use glacex::{Alignment, Label, column, row};

column![
    &mut Label::new("header", "System Header"),
    &mut row![
        &mut Label::new("left_item", "Left Item"),
        &mut Label::new("right_item", "Right Item"),
    ]
    .align(Alignment::Center)
    .spacing(12.0),
]
.align(Alignment::Start)
.spacing(8.0)
.arrange_at([20.0, 20.0], ui);
```

- `.align(Alignment::Start | Alignment::Center | Alignment::End)` sets cross-axis alignment.
- `.spacing(px)` sets the gap between children.
- `.arrange_at([x, y], ui)` measures and arranges the tree in one call.
- `.size([w, h])` / `.width(px)` / `.height(px)` override the row/column's own size instead of hugging its content — pairs with `.align()` to center/end-align children in the extra space.
- `.padding([x, y])` insets children from the row/column's own bounds.

## Widgets

Most widgets take a stable string id as their first constructor argument (or a `.id(...)` builder for the ones that don't) — that's what ties per-frame widget state back to the same logical widget across frames. `Card`, `Container`, `Divider`, and `ProgressBar` only need an id if you're relying on animated/persistent state or want them addressable in the accessibility tree; leaving it off is fine for purely decorative instances.

### Controls

#### Button
```rust
let mut btn = Button::new("deploy_btn", "Deploy")
    .tooltip("Triggers a deployment event")
    .primary();

if btn.clicked() {
    println!("clicked");
}
```
Variants: `.primary()`, `.outline()`, `.ghost()`, `.danger()`.

#### Checkbox
```rust
let mut check = Checkbox::new("enable_feature").default_checked(true);
let is_checked = ui.widget_state::<CheckboxState>("enable_feature").checked;
```

#### RadioButton
```rust
row![
    &mut RadioButton::new("theme_group", "dark"),
    &mut Label::new("dark_label", "Dark Theme"),
];

let selected = ui.selected_option("theme_group").unwrap_or("dark");
```

#### Switch
```rust
let mut sw = Switch::new("network_stream").default_enabled(true);
let enabled = ui.widget_state::<SwitchState>("network_stream").enabled;
```

#### Slider
```rust
let mut slider = Slider::new("volume", 0.0, 100.0).width(240.0).default_value(50.0);
let val = ui.widget_state::<SliderState>("volume").value;
```

#### TextInput
```rust
let mut input = TextInput::new("username").width(260.0).placeholder("Enter a username");
let text = ui.widget_state::<TextEditState>("username").text().to_string();
```

#### TextArea
```rust
let mut notes = TextArea::new("notes").size([300.0, 120.0]);
```

#### SelectBox
```rust
use glacex::{SelectBox, SelectOption};

let options = vec![
    SelectOption::new("light", "Light"),
    SelectOption::new("dark", "Dark"),
    SelectOption::new("mocha", "Catppuccin Mocha"),
    SelectOption::new("tokyo", "Tokyo Night"),
];

let mut sel = SelectBox::new("theme_picker", options)
    .placeholder("Choose a theme…")
    .width(220.0)
    .show_clear(true)
    .tooltip("Switch the active colour theme");

sel.arrange_at([40.0, 40.0], ui);

let selected = ui
    .widget_state::<SelectBoxState>("theme_picker")
    .selected
    .clone();
```

Enable live filtering with `.searchable()`:

```rust
let mut sel = SelectBox::new("country", countries)
    .placeholder("Select a country…")
    .width(260.0)
    .searchable()
    .show_clear(true);
```

The dropdown opens with a spring animation, closes on Escape or outside-click, supports `↑`/`↓` keyboard navigation and `Enter` to confirm. Use `.show_clear(true)` to add an X button that clears the selection.

### Containers

#### Card
```rust
let mut content = Label::new("card_label", "Inside Card");
let mut card = Card::new(&mut content).padding([16.0, 16.0]);
```

#### ScrollView
```rust
ScrollView::new("log_view", &mut child_column)
    .size([300.0, 150.0])
    .arrange_at([20.0, 20.0], ui);
```

#### Container & Divider
- `Container::new(&mut child).size([w, h])` — fixed-size wrapper.
- `Divider::horizontal(width)` / `Divider::vertical(height)` — separator rule, `.faint()` for a subtler hairline.

### Displays

- `Label::new(id, text)` — text, with `.secondary()`, `.muted()`, `.accent()`, size presets (`.caption()`, `.heading()`, `.metric()`, ...), and `.mono()`.
- `Badge::new(text)` — status pill, `.secondary()` / `.outline()` / `.success()` / `.warning()` / `.error()`.
- `ProgressBar::new(ratio)` — completion bar; give it a stable `.id(...)` if you want the fill to animate instead of snapping.

## Styling

### Style Structs

```rust
use glacex::{Button, ButtonStyle, Color, Fill, ShadowStyle};

let save_btn = Button::new("save_btn", "Save").style(ButtonStyle {
    fill: Fill::Solid(Color::hex_str("#4f46e5")),
    hover_fill: Fill::Solid(Color::hex_str("#6366f1")),
    pressed_fill: Fill::Solid(Color::hex_str("#4338ca")),
    border_width: 1.0,
    border_color: Color::WHITE.with_alpha(0.2),
    corner_radius: 8.0,
    shadow: Some(ShadowStyle {
        color: Color::hex_str("#4f46e5").with_alpha(0.4),
        blur_radius: 12.0,
        offset: [0.0, 3.0],
    }),
    sharp: false,
});
```

| Style Struct | Target Widget | Key Fields |
|---|---|---|
| `ButtonStyle` | `Button` | `fill`, `hover_fill`, `pressed_fill`, `border_width`, `border_color`, `corner_radius`, `shadow`, `sharp` |
| `CheckboxStyle` | `Checkbox` | `fill`, `hover_fill`, `checked_fill`, `border_width`, `border_color`, `corner_radius`, `shadow` |
| `TextInputStyle` | `TextInput` | `fill`, `border_width`, `border_color`, `focus_border_color`, `corner_radius`, `selection_color`, `cursor_color`, `shadow` |
| `TextAreaStyle` | `TextArea` | `fill`, `border_color`, `focus_border_color`, `thumb_fill`, `thumb_dragging_fill` |
| `ScrollViewStyle` | `ScrollView` | `thumb_fill`, `thumb_dragging_fill`, `thumb_corner_radius` |
| `CardStyle` | `Card` | `fill`, `border_width`, `border_color`, `corner_radius`, `padding`, `shadow` |
| `SelectBoxStyle` | `SelectBox` | `fill`, `hover_fill`, `focus_fill`, `border_color`, `focus_border_color`, `corner_radius`, `height`, `dropdown_fill`, `dropdown_shadow`, `item_height`, `item_hover_fill`, `item_active_fill`, `searchable` |

### ShadowStyle

```rust
pub struct ShadowStyle {
    pub color: Color,
    pub blur_radius: f32,
    pub offset: [f32; 2],
}
```

### Color

`Color` is `#[repr(C)]` and derives `bytemuck::Pod`/`Zeroable` so it can go straight into a GPU vertex buffer:

```rust
Color::rgb(255, 128, 0);
Color::rgba(255, 128, 0, 0.5);
Color::hex_str("#4f46e5");
Color::hex(0x4f46e5);
Color::hsv(240.0, 0.8, 0.9);

let tinted = Color::WHITE.with_alpha(0.3);
let blended = Color::RED.lerp(Color::BLUE, 0.5);
let dark = Color::RED.darken(0.2);
let light = Color::RED.lighten(0.2);
```

### Fill and Gradients

```rust
use glacex::{Color, Fill, Gradient, GradientKind, GradientStop};

let sunset = Fill::Gradient(Gradient {
    kind: GradientKind::Linear { angle: 90.0 },
    stops: vec![
        GradientStop { position: 0.0, color: Color::hex_str("#ff7e5f") },
        GradientStop { position: 1.0, color: Color::hex_str("#feb47b") },
    ],
});
```

Supported kinds: `GradientKind::Linear { angle }`, `Radial { center, radius }`, `Conic { center }`. Gradients are cached in a GPU ramp atlas by content hash, so reusing the same definition across frames is free.

### Theme

9 built-in palettes, switchable at runtime with `ui.set_theme(...)`. Defaults to a light, shadcn-inspired theme.

```rust
ui.set_theme(Theme::LIGHT);
ui.set_theme(Theme::DARK);
ui.set_theme(Theme::CATPPUCCIN_MOCHA);
ui.set_theme(Theme::CATPPUCCIN_LATTE);
ui.set_theme(Theme::TOKYO_NIGHT);
ui.set_theme(Theme::GRUVBOX_DARK);
ui.set_theme(Theme::GRUVBOX_LIGHT);
ui.set_theme(Theme::NORD);
ui.set_theme(Theme::ROSE_PINE);
```

#### Presets

| Preset | Mode | Canvas | Accent |
|---|---|---|---|
| `Theme::LIGHT` *(default)* | Light | `#ffffff` | `#18181b` |
| `Theme::DARK` | Dark | `#09090b` | `#4f46e5` |
| `Theme::CATPPUCCIN_MOCHA` | Dark | `#1e1e2e` | `#cba6f7` |
| `Theme::CATPPUCCIN_LATTE` | Light | `#eff1f5` | `#8839ef` |
| `Theme::TOKYO_NIGHT` | Dark | `#1a1b26` | `#7aa2f7` |
| `Theme::GRUVBOX_DARK` | Dark | `#282828` | `#fe8019` |
| `Theme::GRUVBOX_LIGHT` | Light | `#fbf1c7` | `#af3a03` |
| `Theme::NORD` | Dark | `#2e3440` | `#88c0d0` |
| `Theme::ROSE_PINE` | Dark | `#191724` | `#eb6f92` |

#### Design tokens

| Token | Default (`LIGHT`) | Used for |
|---|---|---|
| `bg_canvas` | `#ffffff` | Window background |
| `surface` | `#ffffff` | Cards, panels |
| `surface_subtle` | `#f4f4f5` | Inset panels, control tracks |
| `surface_elevated`| `#ffffff` | Modals, tooltips, dropdowns |
| `idle` | `#f4f4f5` | Resting button/control fill |
| `hovered` | `#e4e4e7` | Hover state |
| `pressed` | `#d4d4d8` | Pressed state |
| `active` | `#18181b` | Primary action |
| `border_faint` | `rgba(0,0,0,0.04)` | Hairline dividers |
| `border` | `rgba(0,0,0,0.08)` | Standard borders |
| `border_strong` | `rgba(0,0,0,0.16)` | Focused/emphasized borders |
| `text_primary` | `#09090b` | Body text |
| `text_secondary`| `#71717a` | Captions |
| `text_muted` | `#a1a1aa` | Placeholders, metadata |
| `success` | `#16a34a` | Success state |
| `warning` | `#d97706` | Warning state |
| `error` | `#e11d48` | Error state |

### Window Title and Background

```rust
ui.set_title("My App");
ui.set_theme(Theme::LIGHT); // also updates the window background color
```

## Accessibility

glacex can expose its widget tree to assistive technology through [`accesskit`](https://accesskit.dev/) — AT-SPI on Linux, UIA on Windows, NSAccessibility on macOS. It's disabled by default; turn it on when building the app:

```rust
App::new(root_widget)
    .accessibility_enabled(true)
    .run();
```

Widgets pick up a role and label automatically where it makes sense (`Button`, `Checkbox`, `TextInput`, ...). It's been checked against Orca and Accerciser on Linux.

One thing worth knowing if you're testing on Linux: `accesskit`'s AT-SPI backend only activates once the desktop's `ScreenReaderEnabled` flag is on — which normally happens when Orca (or another screen reader) starts, *not* just because the accessibility bus is running. If a tool like Accerciser isn't picking up your app, that flag is the first thing to check:

```bash
busctl --user get-property org.a11y.Bus /org/a11y/bus org.a11y.Status ScreenReaderEnabled
# if it prints "b false", flip it on for testing:
busctl --user set-property org.a11y.Bus /org/a11y/bus org.a11y.Status ScreenReaderEnabled b true
```

## How Rendering Works

1. Widgets call `Ui::draw_rect` / `Ui::draw_text` to queue primitives.
2. Rects sharing a scissor rect batch into one instanced draw call.
3. `shader.wgsl` evaluates corner rounding, borders, and soft drop shadows per-fragment as an SDF.
4. Glyphs are cached into a texture atlas by `glyphon` and drawn with per-widget scissor bounds.
5. Everything submits in a single GPU command buffer per frame.

## Examples

```bash
# Dashboard-style demo (buttons, inputs, switches, sliders, logs)
cargo run --example demo

# Color/theme preview
cargo run --example example1

# Style playground
cargo run --example example2
```

## Project Layout

```
glacex/
├── docs/                 # Extended documentation
│   ├── architecture.md   # Rendering pipeline and SDF shaders
│   ├── layout.md         # Flexbox and measurement system
│   └── widgets.md        # Widget reference
├── examples/             # Runnable examples
├── src/
│   ├── lib.rs            # App runner and window lifecycle
│   ├── accessibility.rs  # accesskit tree, action/activation handlers
│   ├── animation.rs      # Motion constants, springs, easing
│   ├── ui.rs             # Per-frame state, focus, clipping, drawing
│   ├── widget.rs         # Widget and Measurable traits
│   ├── layout.rs         # row! and column! macros (taffy)
│   ├── button.rs         # Button widget
│   ├── checkbox.rs       # Checkbox widget
│   ├── radio_button.rs   # RadioButton widget
│   ├── switch.rs         # Switch widget
│   ├── slider.rs         # Slider widget
│   ├── text_input.rs     # Single-line text input
│   ├── text_area.rs      # Multi-line text editor
│   ├── scroll_view.rs    # ScrollView container
│   ├── card.rs           # Card container
│   ├── select_box.rs     # SelectBox / Combobox widget
│   ├── theme.rs          # Theme palettes and design tokens
│   ├── painter.rs        # wgpu + glyphon rendering backend
│   └── shader.wgsl       # Instanced SDF quad shader
├── CHANGELOG.md
├── CONTRIBUTING.md
└── Cargo.toml
```

## Known Limitations

- `GradientKind::Mesh` is reserved but not implemented yet — it currently falls back to transparent. Use Linear, Radial, or Conic.
- Pre-1.0, so the API still moves around between releases.

## Documentation

- [Architecture & Rendering](docs/architecture.md) — `wgpu`, SDF shaders, the frame lifecycle.
- [Widget Reference](docs/widgets.md) — every widget, its constructor, and its builder methods.
- [Layout Guide](docs/layout.md) — flexbox mechanics with `taffy`.

## Contributing

Bug reports and PRs are welcome — see [CONTRIBUTING.md](CONTRIBUTING.md) for coding standards and the PR checklist.

## License

MIT. See [LICENSE](LICENSE).

Copyright (c) 2026 Artem Tsitronov and Soumalya Das.
