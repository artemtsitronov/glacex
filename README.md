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

GPU-accelerated, immediate-mode UI library built entirely from scratch in Rust on `wgpu`, `winit`, and `taffy`.

Built by **Artem Tsitronov** and **Soumalya Das**.

</div>

> ⚠️ **Status**: Early / active development. APIs are evolving and may change between releases. See [ROADMAP.md](ROADMAP.md) for future release goals.

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
- [How Rendering Works](#how-rendering-works)
- [Examples](#examples)
- [Project Layout](#project-layout)
- [Known Limitations](#known-limitations)
- [Documentation & Docs Directory](#documentation)
- [Contributing](#contributing)
- [License](#license)

## What is glacex?

`glacex` is a GPU-accelerated, immediate-mode UI library for Rust. It draws every pixel itself without relying on any external GUI engine or HTML/CSS runtime:

- **`winit`** owns the native window lifecycle and cross-platform event loop.
- **`wgpu`** renders every shape as an instanced signed-distance-field (SDF) quad directly on the GPU.
- **`glyphon`** (+ `swash`) shapes, rasterizes, and caches glyph atlases with per-widget clipping.
- **`taffy`** computes flexbox layout calculations for declarative rows and columns.

There is no retained widget tree, no XML/HTML markup, and no hidden reactivity engine: you describe your UI in clean Rust code every frame, and `glacex` measures, lays out, hit-tests, animates, and renders it in real time.

## Features

- **Custom GPU Renderer**: Instanced rectangles with rounded corners (anti-aliased SDF), borders, and soft blurred drop shadows submitted in a single draw call per shared scissor clip rect.
- **Sub-Pixel Text Rendering**: Powered by `glyphon` 0.12 with independent clipping boundaries and text metrics.
- **Advanced Fill System**: Solid colors and procedural gradients (Linear, Radial, Conic) cached into a GPU atlas texture.
- **First-Class GPU Color**: `#[repr(C)]` `Pod`/`Zeroable` `Color` struct directly compatible with GPU vertex buffers. Supports hex, RGB, HSV, alpha blending, linear interpolation (`lerp`), lightening, and darkening.
- **Dynamic Native Cursors**: Contextual OS cursor changes (`Pointer`, `Text`, `EwResize`, `Default`) via `winit`.
- **Floating Tooltip Engine**: Elevated, viewport-clamped overlay cards with drop shadows and typography metrics.
- **Comprehensive Widget Set**: `Button`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `ProgressBar`, `TextInput`, `TextArea`, `ScrollView`, `Card`, `Container`, `Badge`, `Divider`, `Label`.
- **Flexbox Layout**: Declarative `row![]` and `column![]` macros backed by `taffy` 0.13 with alignment, gap spacing, and automatic child arrangement.
- **Rich Interaction Model**: Hover/press/click tracking, secondary/middle mouse buttons, Tab/Shift+Tab focus traversal, double/triple-click word/line selection, clipboard integration via `arboard`, and cursor blinking.
- **Persistent State Tracking**: Stateless syntax with stateful continuity: widget state is keyed by stable IDs (`Ui::widget_state`, `take_widget_state`, `put_widget_state`).
- **Physics & Easing Motion Engine**: Frame-rate independent exponential decay (`animate_towards`), standard easing curves (`Ease`), and `Spring` simulations. Named half-life constants via `Motion::INSTANT` (30ms), `Motion::SNAPPY` (45ms), `Motion::FLUID` (60ms), and `Motion::GENTLE` (90ms) provide a shared motion language across all widgets for tactile, Apple/Vercel-quality transitions.
- **Precision Scrolling**: Draggable, momentum-free, auto-hiding scrollbars shared across `ScrollView` and `TextArea`.

## Requirements

- **Rust**: Version **1.85+** (Rust 2024 Edition).
- **GPU Backend**: Any modern GPU/driver supported by `wgpu` (Vulkan, Metal, DirectX 12, or OpenGL ES).
- **Linux Requirements**: A running Wayland or X11 session. On headless environments, a virtual display (e.g. `xvfb`) is needed to create window surfaces.

## Installation

### From crates.io

Add the latest release to your project:

```bash
cargo add glacex
```

Or specify it manually inside your `Cargo.toml`:

```toml
[dependencies]
glacex = "0.1.4"
```

### From GitHub (Latest Development)

To use the bleeding-edge main branch:

```toml
[dependencies]
glacex = { git = "https://github.com/artemtsitronov/glacex.git", branch = "main" }
```

### Linux Development Dependencies

On Debian/Ubuntu, Fedora, or Arch Linux, ensure graphics and windowing development libraries are present:

```bash
# Debian / Ubuntu
sudo apt install libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxkbcommon-dev libwayland-dev

# Fedora
sudo dnf install libX11-devel libXcursor-devel libXrandr-devel libXi-devel libxkbcommon-devel wayland-devel

# Arch Linux
sudo pacman -S libx11 libxcursor libxrandr libxi libxkbcommon wayland
```

## Quick Start

Here is a minimal, self-contained interactive counter application:

```rust
use glacex::{App, Button, Color, Label, Ui, Widget, column};

struct Counter {
    count: u32,
}

impl Widget for Counter {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(18, 18, 22));

        let mut label = Label::new(format!("Count: {}", self.count));
        let mut button = Button::new("Increment");

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

Run `App::new(root).run()` to open a native window, initialize GPU pipelines, and run the event loop.

## Core Concepts

### App

`App<W: Widget>` owns the native `winit` window and application event loop:

```rust
App::new(root_widget)
    .update(|root| {
        // Runs once per frame prior to rendering. Ideal for app-level
        // orchestrations and input inspections before layout passes.
    })
    .run();
```

### Widget and Measurable

Every UI element implements the `Widget` trait:

```rust
pub trait Widget {
    type Output;
    fn ui(&mut self, ui: &mut Ui) -> Self::Output;
}
```

Widgets that support flexible measurement and placement within layouts implement `Measurable`:

```rust
pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;
}
```

- `measure`: Calculates intrinsic width and height dimensions.
- `arrange`: Performs hit-testing, input handling, and submits drawing primitives at the calculated rect.

### Ui

`Ui` is the per-frame context object passed to all widgets:
- **Input Inspection**: `mouse_position()`, `mouse_pressed()`, `mouse_right_pressed()`, `click_count()`, `key_pressed()`, `ctrl_held()`, `shift_held()`.
- **Cursor Management**: `set_cursor_icon(CursorIcon)`.
- **Tooltips**: `show_tooltip(text)`, `show_tooltip_at(text, pos)`.
- **Persistent State**: `widget_state::<T>(id)`, `take_widget_state::<T>(id)`, `put_widget_state(id, state)`.
- **Focus Management**: `request_focus(id)`, `is_focused(id)`, `advance_focus(backward)`.
- **Clipping**: `push_clip(rect)`, `pop_clip()`, `push_input_block(rect)`.
- **Primitives**: `draw_rect(...)`, `draw_text(...)`, `measure_text(...)`, `line_height()`.
- **Window Controls**: `set_title(&str)`, `set_bgcolor(Color)`.

### Layout

Flexbox layouts are constructed with the `row![]` and `column![]` macros:

```rust
use glacex::{Alignment, Label, column, row};

column![
    &mut Label::new("System Header"),
    &mut row![&mut Label::new("Left Item"), &mut Label::new("Right Item")]
        .align(Alignment::Center)
        .spacing(12.0),
]
.align(Alignment::Start)
.spacing(8.0)
.arrange_at([20.0, 20.0], ui);
```

- `.align(Alignment::Start | Alignment::Center)` sets cross-axis alignment.
- `.spacing(px)` defines the gap between child items.
- `.arrange_at([x, y], ui)` executes measurement and arranges children in one call.

## Widgets

### Controls

#### Button
Interactive button with hover/press states, customizable style, and tooltips:
```rust
let mut btn = Button::new("Deploy Trigger")
    .tooltip("Triggers a deployment event");

if btn.clicked() {
    println!("Triggered!");
}
```

#### Checkbox
Persistent boolean toggle:
```rust
use glacex::CheckboxState;

let mut check = Checkbox::new("enable_feature");
let is_checked = ui.widget_state::<CheckboxState>("enable_feature").checked;
```

#### RadioButton
Mutually exclusive selection within a shared group:
```rust
row![
    &mut RadioButton::new("theme_group", "dark"),
    &mut Label::new("Dark Theme")
];

let selected = ui.selected_option("theme_group").unwrap_or("dark");
```

#### Switch
Compact animated toggle control:
```rust
use glacex::SwitchState;

let mut sw = Switch::new("network_stream");
let enabled = ui.widget_state::<SwitchState>("network_stream").enabled;
```

#### Slider
Continuous numerical slider with interactive dragging knob and active track:
```rust
use glacex::SliderState;

let mut slider = Slider::new("volume", 0.0, 100.0, 240.0);
let val = ui.widget_state::<SliderState>("volume").value;
```

#### TextInput
Single-line text box with caret blinking, selection dragging, word jumping, and clipboard:
```rust
use glacex::TextEditState;

let mut input = TextInput::new("username", 260.0);
let text = ui.widget_state::<TextEditState>("username").text().to_string();
```

#### TextArea
Multi-line editor with vertical scrolling, word wrapping, newlines, and arrow navigation:
```rust
let mut notes = TextArea::new("notes", 300.0, 120.0);
```

### Containers

#### Card
Elevated surface container with rounded corners, padding, and soft drop shadow:
```rust
let mut content = Label::new("Inside Card");
let mut card = Card::new(&mut content);
```

#### ScrollView
Dual-axis scrolling container with interactive draggable scrollbar thumbs:
```rust
ScrollView::new("log_view", [300.0, 150.0], &mut child_column)
    .arrange_at([20.0, 20.0], ui);
```

#### Container & Divider
- `Container`: Fixed-dimension viewport wrapper.
- `Divider`: Visual separation rule (`Divider::horizontal(width)` or `Divider::vertical(height)`).

### Displays

- `Label`: Plain or dynamic text rendering.
- `Badge`: Semantic pills (`BadgeVariant::Default`, `Outline`, `Success`, `Warning`, `Error`).
- `ProgressBar`: Continuous completion indicator.

## Styling

### Style Structs

Widgets support granular, type-safe styles with sensible defaults:

```rust
use glacex::{Button, ButtonStyle, Color, Fill, ShadowStyle};

let save_btn = Button::new("Save").style(ButtonStyle {
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

| Style Struct | Target Widget | Key Configuration Fields |
|---|---|---|
| `ButtonStyle` | `Button` | `fill`, `hover_fill`, `pressed_fill`, `border_width`, `border_color`, `corner_radius`, `shadow`, `sharp` |
| `CheckboxStyle` | `Checkbox` | `fill`, `hover_fill`, `checked_fill`, `border_width`, `border_color`, `corner_radius`, `shadow` |
| `TextInputStyle` | `TextInput` | `fill`, `border_width`, `border_color`, `focus_border_color`, `corner_radius`, `selection_color`, `cursor_color`, `shadow` |
| `TextAreaStyle` | `TextArea` | `fill`, `border_color`, `focus_border_color`, `thumb_fill`, `thumb_dragging_fill` |
| `ScrollViewStyle` | `ScrollView` | `thumb_fill`, `thumb_dragging_fill`, `thumb_corner_radius` |
| `CardStyle` | `Card` | `fill`, `border_width`, `border_color`, `corner_radius`, `padding`, `shadow` |

### ShadowStyle

Shared soft drop shadow configuration:

```rust
pub struct ShadowStyle {
    pub color: Color,
    pub blur_radius: f32,
    pub offset: [f32; 2],
}
```

### Color

`Color` is `#[repr(C)]` and derives `bytemuck::Pod` / `Zeroable` for direct zero-cost GPU vertex buffer utilization:

```rust
Color::rgb(255, 128, 0);           // 0-255 RGB
Color::rgba(255, 128, 0, 0.5);     // 0-255 RGB with alpha
Color::hex_str("#4f46e5");         // Hex string (#RRGGBB or #RRGGBBAA)
Color::hex(0x4f46e5);              // Hex integer
Color::hsv(240.0, 0.8, 0.9);       // Hue, Saturation, Value

let tinted = Color::WHITE.with_alpha(0.3);
let blended = Color::RED.lerp(Color::BLUE, 0.5);
let dark = Color::RED.darken(0.2);
let light = Color::RED.lighten(0.2);
```

### Fill and Gradients

Fills support solid colors or procedural gradients cached directly into a GPU ramp atlas texture:

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

Gradient modes supported: `GradientKind::Linear { angle }`, `Radial { center, radius }`, and `Conic { center }`.

### Theme Engine & Design Tokens

Glacex includes a complete runtime theming engine with **9 meticulously calibrated presets**. By default, Glacex boots into a pristine, luxurious **shadcn / Apple-grade white theme** (`Theme::LIGHT`), while also supporting instant switching to community favorites from r/unixporn.

```rust
// Switch the entire application palette dynamically at runtime
ui.set_theme(Theme::LIGHT);            // Default: Apple / shadcn pure white
ui.set_theme(Theme::DARK);             // Linear / Vercel dark mode
ui.set_theme(Theme::CATPPUCCIN_MOCHA); // Warm dark pastel
ui.set_theme(Theme::CATPPUCCIN_LATTE); // Cozy light pastel
ui.set_theme(Theme::TOKYO_NIGHT);      // Midnight cyberpunk
ui.set_theme(Theme::GRUVBOX_DARK);     // Warm retro dark
ui.set_theme(Theme::GRUVBOX_LIGHT);    // Warm retro light
ui.set_theme(Theme::NORD);             // Arctic cool blue
ui.set_theme(Theme::ROSE_PINE);        // Moody minimalist aesthetic
```

#### Built-in Theme Presets

| Preset | Mode | Canvas | Accent | Vibe |
|---|---|---|---|---|
| `Theme::LIGHT` *(Default)* | Light | `#ffffff` | `#18181b` | Apple & shadcn/ui minimal luxury |
| `Theme::DARK` | Dark | `#09090b` | `#4f46e5` | Linear & Vercel deep charcoal |
| `Theme::CATPPUCCIN_MOCHA` | Dark | `#1e1e2e` | `#cba6f7` | Soothing pastel warmth |
| `Theme::CATPPUCCIN_LATTE` | Light | `#eff1f5` | `#8839ef` | Soft, creamy daylight aesthetic |
| `Theme::TOKYO_NIGHT` | Dark | `#1a1b26` | `#7aa2f7` | Cyberpunk neon midnight |
| `Theme::GRUVBOX_DARK` | Dark | `#282828` | `#fe8019` | Retro warm groove |
| `Theme::GRUVBOX_LIGHT` | Light | `#fbf1c7` | `#af3a03` | Paper-textured warm daylight |
| `Theme::NORD` | Dark | `#2e3440` | `#88c0d0` | Arctic frost & cool slate |
| `Theme::ROSE_PINE` | Dark | `#191724` | `#eb6f92` | Atmospheric vintage rose |

#### Core Design Tokens

| Token Field | Default (`LIGHT`) | Purpose |
|---|---|---|
| `bg_canvas` | `#ffffff` | Clean root window canvas |
| `surface` | `#ffffff` | Standard elevated card/panel |
| `surface_subtle` | `#f4f4f5` (Zinc 100) | Inset panels, control tracks |
| `surface_elevated`| `#ffffff` | Modals, tooltips, dropdowns |
| `idle` | `#f4f4f5` (Zinc 100) | Resting button/control fill |
| `hovered` | `#e4e4e7` (Zinc 200) | Interactive hover state |
| `pressed` | `#d4d4d8` (Zinc 300) | Tactile pressed state |
| `active` | `#18181b` (Zinc 900) | High-contrast primary action |
| `border_faint` | `rgba(0,0,0,0.04)` | Hairline internal dividers |
| `border` | `rgba(0,0,0,0.08)` | Standard component border |
| `border_strong` | `rgba(0,0,0,0.16)` | Focused / emphasized border |
| `text_primary` | `#09090b` (Zinc 950) | High-contrast body typography |
| `text_secondary`| `#71717a` (Zinc 500) | Subdued captions & descriptors |
| `text_muted` | `#a1a1aa` (Zinc 400) | Micro metadata & placeholders |
| `success` | `#16a34a` (Emerald 600)| Success status badge/progress |
| `warning` | `#d97706` (Amber 600) | Warning status badge/progress |
| `error` | `#e11d48` (Rose 600) | Error status badge/progress |

### Window Title and Background

```rust
ui.set_title("Glacex Application");
ui.set_theme(Theme::LIGHT); // Automatically synchronizes window background color
```

## How Rendering Works

Every geometric element in `glacex` (such as a button surface, card frame, text caret, or scrollbar) is rendered as an instanced signed-distance-field (SDF) quad:

1. **Primitive Queueing**: Widgets call `Ui::draw_rect` and `Ui::draw_text`.
2. **Batching**: Rectangles sharing scissor boundaries are packed into single instanced GPU draw calls.
3. **SDF Evaluation**: `shader.wgsl` computes pixel-perfect anti-aliased corner curves, border strokes, and smooth soft drop shadows in fragment pipelines.
4. **Text Pipeline**: Glyphs are cached into a multi-texture atlas by `glyphon` and drawn with per-widget scissor bounds.
5. **GPU Presentation**: All rendering passes submit within a single GPU command buffer.

## Examples

Run any of the included examples directly via `cargo`:

```bash
# Full dashboard demonstration (buttons, inputs, switches, sliders, logs)
cargo run --example demo

# Dynamic color and theme preview
cargo run --example example1

# Style playground & interactive live updater
cargo run --example example2
```

## Project Layout

```
glacex/
├── docs/                 # Extended documentation and architectural guides
│   ├── architecture.md   # Rendering pipeline and SDF shaders
│   ├── layout.md         # Flexbox and measurement system
│   └── widgets.md        # Comprehensive widget reference
├── examples/             # Runnable demo examples
├── src/
│   ├── lib.rs            # App runner and window lifecycle
│   ├── animation.rs      # Motion constants, physics springs, easings, frame-rate independent animations
│   ├── ui.rs             # Ui per-frame state, focus, clipping, drawing
│   ├── widget.rs         # Widget and Measurable traits
│   ├── layout.rs         # row! and column! macros (Taffy flexbox)
│   ├── button.rs         # Button widget and ButtonStyle
│   ├── checkbox.rs       # Checkbox widget
│   ├── radio_button.rs   # RadioButton widget
│   ├── switch.rs         # Switch toggle widget
│   ├── slider.rs         # Range slider widget
│   ├── text_input.rs     # Single-line text input
│   ├── text_area.rs      # Multi-line text editor
│   ├── scroll_view.rs    # ScrollView container
│   ├── card.rs           # Elevated Card container
│   ├── theme.rs          # Modern dark palette
│   ├── painter.rs        # wgpu + glyphon rendering backend
│   └── shader.wgsl       # Instanced SDF quad WGSL shader
├── CHANGELOG.md          # Release history and updates
├── CONTRIBUTING.md       # Contribution guidelines
├── ROADMAP.md            # Long-term feature milestones
└── Cargo.toml
```

## Known Limitations

- **Mesh Gradients**: `GradientKind::Mesh` is currently reserved in the type system and falls back to transparent. Use Linear, Radial, or Conic gradients.
- **Accessibility & IME**: Full screen-reader ARIA trees and IME composition for complex East Asian scripts are on the roadmap for upcoming 0.x releases.
- **API Stability**: Pre-1.0 APIs are subject to iterative refinements.

## Documentation

Detailed architectural and design guides are available in the [`docs/`](docs/) directory:
- [**Architecture & Rendering**](docs/architecture.md): Deep dive into `wgpu`, SDF shaders, and rendering pipelines.
- [**Widget Reference**](docs/widgets.md): Complete list of all widgets, configurations, and callbacks.
- [**Layout Guide**](docs/layout.md): Flexbox mechanics, constraints, and alignment with `taffy`.

## Contributing

Contributions, feedback, and bug reports are welcome! Please review [CONTRIBUTING.md](CONTRIBUTING.md) for coding standards, pull request processes, and development guidelines.

## License

This project is licensed under the [MIT License](LICENSE).

Copyright (c) 2026 Artem Tsitronov and Soumalya Das.
