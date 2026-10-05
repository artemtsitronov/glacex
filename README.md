<div align="center">

![demo](screenshots/demo.png)

# glacex

<p align="center">
  <a href="https://crates.io/crates/glacex"><img src="https://img.shields.io/crates/v/glacex?style=for-the-badge&logo=rust&logoColor=cdd6f4&label=crates.io&labelColor=181825&color=cba6f7" alt="Crates.io Version"></a>
  <a href="https://docs.rs/glacex"><img src="https://img.shields.io/docsrs/glacex?style=for-the-badge&logo=docsdotrs&logoColor=cdd6f4&label=docs.rs&labelColor=181825&color=89b4fa" alt="docs.rs"></a>
  <a href="https://github.com/artemtsitronov/glacex/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-a6e3a1?style=for-the-badge&logo=opensourceinitiative&logoColor=cdd6f4&labelColor=181825" alt="License"></a>
  <img src="https://img.shields.io/badge/rustc-1.88+-fab387?style=for-the-badge&logo=rust&logoColor=cdd6f4&labelColor=181825" alt="Rustc Version">
  <img src="https://img.shields.io/badge/wgpu-30.0-f38ba8?style=for-the-badge&logo=webgpu&logoColor=cdd6f4&labelColor=181825" alt="wgpu 30.0">
  <a href="https://github.com/artemtsitronov/glacex/stargazers"><img src="https://img.shields.io/github/stars/artemtsitronov/glacex?style=for-the-badge&logo=github&logoColor=cdd6f4&label=stars&labelColor=181825&color=f9e2af" alt="GitHub Stars"></a>
</p>

GPU-accelerated, immediate-mode UI library built from scratch in Rust on top of `wgpu`, `winit`, and `taffy`.

Built by **Artem Tsitronov** and **Soumalya Das**.
For every question or concern, you can contact the author at this email: <artemtsitronov@gmail.com>

</div>

> **Status**: 0.2.0 is a more stable release, big API changes are not planned.
Every big API change/redesign will be warned.

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
  - [Fills, Gradients & Images](#fills-gradients--images)
  - [Custom Shapes / Paths](#custom-shapes--paths)
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

If you'd like an experimental version of glacex, try visiting programmersd21 fork: https://github.com/programmersd21/glacex [DISCLAIMER]: Artem Tsitronov is not responsible for programmersd21 fork. That includes drastic changes, redesigns, and AI-generated content

## What is glacex?

`glacex` is an immediate-mode UI library for Rust that draws its own pixels instead of wrapping a native toolkit or a browser engine:

- **`winit`** owns the window and the cross-platform event loop.
- **`wgpu`** renders every shape as an instanced signed-distance-field (SDF) quad on the GPU.
- **`glyphon`** shapes and rasterizes text into glyph atlases with per-widget clipping.
- **`taffy`** does the flexbox math for `row!`/`column!` layouts.
- **`image`** decodes the images.
- **`kurbo`** is responsible for paths.

There's no retained widget tree and no markup, you describe the UI in plain Rust every frame, and glacex measures, lays out, hit-tests, animates, and renders it.

## Features

- Custom renderer: instanced shapes (anti-aliased SDF), borders, and soft drop shadows, batched into one draw call per shared clip rect.
- Arbitrary Bezier paths (`MeasurablePath::Free`) via a second tessellated-mesh pipeline, for shapes beyond rect/ellipse, see [Custom Shapes / Paths](#custom-shapes--paths).
- Text rendering via `glyphon` with independent clip bounds per widget.
- Fills: solid colors, gradients (linear, radial, conic), and images, cached into a GPU atlas.
- `Color` is `#[repr(C)]` + `Pod`/`Zeroable`, so it maps straight onto GPU vertex buffers. Hex, RGB, HSV, alpha blending, `lerp`, lighten/darken.
- Cursor changes (pointer, text, resize, default) driven by hover state.
- A floating tooltip layer that clamps to the viewport.
- Widgets: `Button`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `ProgressBar`, `TextInput`, `TextArea`, `ScrollView`, `Card`, `Container`, `Badge`, `Divider`, `Label`, `SelectBox`, `Tabs`.
- `row![]` / `column![]` macros backed by `taffy`, with alignment and spacing.
- Interaction: hover/press/click, secondary/middle mouse buttons, Tab/Shift+Tab focus order, double/triple-click word/line selection, clipboard via `arboard`, blinking cursor.
- Widget state persists across frames keyed by a stable string id, even though the widget itself is rebuilt every frame. Every stateful widget exposes this via `.state(ui)`/`.take_state(ui)`/`.put_state(ui, state)` (from the `StatefulWidget` trait), plus small typed accessors for the common case (`checkbox.is_checked(ui)`, `slider.value(ui)`, etc.), see [Widgets](#widgets). `Ui::widget_state`/`take_widget_state`/`put_widget_state` are the lower-level primitives underneath, for state that isn't tied to a single widget's id.
- Animation via exponential decay (`animate_towards`) plus a small set of easing curves and spring presets, unified under named half-life constants (`Motion::INSTANT`/`SNAPPY`/`FLUID`/`GENTLE`) so transitions feel consistent across widgets.
- Draggable auto-hiding scrollbars shared by `ScrollView` and `TextArea`.
- Optional accessibility tree (AT-SPI on Linux via `accesskit`), see [Accessibility](#accessibility).

## Requirements

- Rust 1.88+ (2024 edition).
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
glacex = "0.2.0"
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

        let mut label = Label::new(format!("Count: {}", self.count));
        let mut button = Button::new("Increment").id("increment-button");

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
- State: `widget_state::<T>(id)`, `take_widget_state::<T>(id)`, `put_widget_state(id, state)`, the primitives behind every widget's `.state(ui)`/`.take_state(ui)`/`.put_state(ui, state)`.
- Focus: `request_focus(id)`, `is_focused(id)`, `advance_focus(backward)`.
- Clipping: `push_clip(rect)`, `pop_clip()`, `push_input_block(rect)`.
- Drawing: `draw_shape(...)`, `draw_text(...)`, `measure_text(...)`, `line_height()`.
- Window: `set_title(&str)`, `set_bgcolor(Color)`.

### Layout

For layout please see `docs/layout.md`.

## Widgets

### Controls

#### Button
```rust
let mut btn = Button::new("Deploy")
    .id("button")
    .tooltip("Triggers a deployment event")
    .primary();

if btn.clicked() {
    println!("clicked");
}
```
Variants: `.primary()`, `.outline()`, `.ghost()`, `.danger()`.

#### Checkbox
```rust
let mut check = Checkbox::new().id("checkbox").default_checked(true);
let is_checked = check.is_checked(ui);
```

#### RadioButton
```rust
row![
    &mut RadioButton::new("theme_group").id("dark"),
    &mut Label::new("Dark Theme"),
];

let selected = ui.selected_option("theme_group").unwrap_or("dark");
```

#### Switch
```rust
let mut sw = Switch::new().id("switch").default_enabled(true);
let enabled = sw.enabled(ui);
```

#### Slider
```rust
let mut slider = Slider::new(0.0, 100.0).id("slider").width(240.0).default_value(50.0);
let val = slider.value(ui);
```

#### TextInput
```rust
let mut input = TextInput::new().id("username-input").width(260.0).placeholder("Enter a username");
let text = input.text(ui);
```

#### TextArea
```rust
let mut notes = TextArea::new().id("notes").size([300.0, 120.0]);
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

let mut sel = SelectBox::new(options)
    .id("select-box")
    .placeholder("Choose a theme…")
    .width(220.0)
    .tooltip("Switch the active colour theme");

sel.arrange_at([40.0, 40.0], ui);

let selected = sel.selected(ui);
```

Enable live filtering with `.searchable()`:

```rust
let mut sel = SelectBox::new(countries)
    .id("searchable")
    .placeholder("Select a country…")
    .width(260.0)
    .searchable();
```

The dropdown opens with a spring animation, closes on Escape or outside-click, supports `↑`/`↓` keyboard navigation and `Enter` to confirm.

#### Tabs
```rust
use glacex::{Tabs, TabItem};

let mut tabs = Tabs::new(
    vec![
        TabItem::new("General").id("tab1"),
        TabItem::new("Account").id("tab2"),
        TabItem::new("Billing").id("tab3"),
    ],
).id("tabs");

let selected = tabs.selected(ui);
```

The active pill slides and resizes to the selected tab with a spring animation; clicking a tab updates the selection immediately. Selection is stored the same way as `RadioButton` groups, so `ui.selected_option("tabs")` also works.

### Containers

#### Card
```rust
let mut content = Label::new("Inside Card");
let mut card = Card::new(&mut content).padding([16.0, 16.0]);
```

#### ScrollView
```rust
ScrollView::new(&mut child_column)
    .id("scroll-view")
    .size([300.0, 150.0])
    .arrange_at([20.0, 20.0], ui);
```

#### Container & Divider
- `Container::new(&mut child).size([w, h])`: fixed-size wrapper.
- `Divider::horizontal(width)` / `Divider::vertical(height)`: separator rule, `.faint()` for a subtler hairline.

### Displays

- `Label::new(text)`: text, with `.secondary()`, `.muted()`, `.accent()`, size presets (`.caption()`, `.heading()`, `.metric()`, ...), and `.mono()`.
- `Badge::new(text)`: status pill, `.secondary()` / `.outline()` / `.success()` / `.warning()` / `.error()`.
- `ProgressBar::new(ratio)`: completion bar

## Styling

### Style Structs

Every widget has a `*Style` struct that controls everything it draws: fills, borders, shadows, padding, fonts, shapes, animation timings and the hover cursor. Start from a theme preset and override only what you need with `..`:

```rust
use glacex::{Button, ButtonStyle, Color, CursorIcon, Fill, Path, ShadowStyle, Stateful};

let save_btn = Button::new("Save").id("btn").style(ButtonStyle {
    fill: Stateful::new(Color::hex_str("#4f46e5"))
        .hovered(Color::hex_str("#6366f1"))
        .pressed(Color::hex_str("#4338ca"))
        .into(),
    border_color: Color::WHITE.with_alpha(0.2).into(),
    path: Path::rect([8.0; 4]).into(),
    shadow: ShadowStyle::new(Color::hex_str("#4f46e5").with_alpha(0.4), 12.0, [0.0, 3.0]).into(),
    scale: Stateful::new(1.0).hovered(1.03).pressed(0.97),
    cursor: CursorIcon::Pointer,
    ..ButtonStyle::primary(ui.theme())
});
```

### Per-state values: `Stateful<T>`

Any property that can change with interaction is a `Stateful<T>`: an `idle` value plus optional `hovered`, `pressed`, `focused`, `selected` and `selected_hovered` values. States you don't set fall back to the layer below them. A plain value converts into a `Stateful` that looks the same in every state, so the simple case stays simple:

```rust
Button::new("Save").fill(Color::rgb(200, 40, 40))

Button::new("Save").fill(
    Stateful::new(Color::rgb(200, 40, 40)).hovered(Color::rgb(230, 60, 60))
)
```

`Button`, `Card`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `TextInput`, `TextArea`, `SelectBox`, `Badge` and `ProgressBar` also have `.fill(..)`, `.border_color(..)`, `.border_width(..)` and `.shadow(..)` builder methods that take either a plain value or a `Stateful`. `Label` has `.fill(..)`, `.text_color(..)`, `.border_color(..)`, `.border_width(..)` and `.padding(..)`, and `Divider`'s `.color(..)` and `.thickness(..)` accept a `Stateful` too.

States are layered in a fixed order: `idle`, then `hovered`, then `selected` (with `selected_hovered` while hovered), then `focused`, then `pressed`. Each layer blends in by its own animated weight. `selected` is "checked" for `Checkbox`, "on" for `Switch`, the chosen option for `RadioButton`, the active tab in `Tabs` and the chosen item in `SelectBox`. `pressed` is also used while a slider or scrollbar thumb is being dragged.

Blending is per type: numbers, colors, solid fills, shadows and text size/color interpolate smoothly; gradients, images, paths, font weight and family switch at the halfway point. Color blends from a fully transparent value keep the target's hue instead of fading through black. Any type that implements the `Blend` trait can be stored in a `Stateful`.

Conventions shared by all styles:

- **States**: fields marked † below are `Stateful<T>` (see above). Everything is stateful except animation timings (`*_motion`, spring stiffness/damping), on/off flags (`sharp`, `clip_content`, …), enums and alignment, strings, `syntax`, `scroll_speed` and `cursor_blink_period`. `Badge`, `Label`, `Divider` and `ProgressBar` only have the `hovered` state. Size-like fields (padding, heights, sizes) are measured with the previous frame's state, so layout follows one frame behind.
- **Sub-parts**: the select box dropdown and search bar react to hovering the dropdown, its items and each tab react to their own hover and to being selected, the tabs indicator reacts to hovering the active tab, and scrollbar tracks/thumbs react to their own hover and drag.
- **Shape**: `path` is any `Path` (`Path::rect(radii)`, `Path::ellipse(rot)`, `Path::from_fn(..)`); `sharp` disables anti-aliasing.
- **Text**: `text_style` sets font family, size, line height, weight and color. A widget-level `.text_style(..)` still overrides it; if it has no color, the style's color is used.
- **Animation**: every `*_motion` field is a half-life in seconds (`Motion::INSTANT`, `SNAPPY`, `FLUID`, `GENTLE`, or any `f32`). `0.0` makes the change instant.
- **Cursor**: `cursor` picks the hover cursor (`glacex::CursorIcon`).

| Style Struct | Target Widget | Fields |
|---|---|---|
| `ButtonStyle` | `Button` | `fill`†, `border_width`†, `border_color`†, `padding`†, `min_size`†, `text_style`†, `text_align`, `shadow`†, `sharp`, `path`†, `scale`†, `offset`†, `hover_motion`, `press_motion`, `cursor`† |
| `BadgeStyle` | `Badge` | `fill`†, `border_width`†, `border_color`†, `padding`†, `height`†, `min_width`†, `text_style`†, `text_align`, `shadow`†, `sharp`, `path`†, `hover_motion` |
| `CardStyle` | `Card` | `fill`†, `border_width`†, `border_color`†, `padding`†, `min_size`†, `shadow`†, `sharp`, `clip_content`, `path`†, `hover_motion`, `cursor`† |
| `CheckboxStyle` | `Checkbox` | `fill`†, `check_color`†, `check_thickness`†, `check_scale`†, `border_width`†, `border_color`†, `shadow`†, `sharp`, `path`†, `size`†, `pop_scale`†, `check_motion`, `hover_motion`, `pop_motion`, `cursor`† |
| `RadioButtonStyle` | `RadioButton` | `fill`†, `border_width`†, `border_color`†, `shadow`†, `sharp`, `dot_color`†, `dot_inset`†, `dot_path`†, `path`†, `size`†, `dot_motion`, `hover_motion`, `cursor`† |
| `SwitchStyle` | `Switch` | `track_fill`†, `thumb_fill`†, `thumb_border_width`†, `thumb_border_color`†, `thumb_padding`†, `thumb_size`†, `thumb_shadow`†, `thumb_path`†, `trail_color`†, `trail_stretch`†, `trail_max`†, `border_width`†, `border_color`†, `shadow`†, `sharp`, `path`†, `size`†, `toggle_motion`, `hover_motion`, `cursor`† |
| `SliderStyle` | `Slider` | `track_fill`†, `filled_fill`†, `thumb_fill`†, `thumb_border_width`†, `thumb_border_color`†, `border_width`†, `border_color`†, `track_height`†, `thumb_size`†, `vertical_padding`†, `shadow`†, `trail_color`†, `trail_stretch`†, `trail_max`†, `sharp`, `path`†, `filled_path`†, `thumb_path`†, `hover_motion`, `drag_motion`, `velocity_motion`, `cursor`† |
| `ProgressBarStyle` | `ProgressBar` | `track_fill`†, `progress_fill`†, `border_width`†, `border_color`†, `progress_border_width`†, `progress_border_color`†, `progress_inset`†, `min_progress_width`†, `size`†, `shadow`†, `sharp`, `path`†, `progress_path`†, `progress_motion`, `hover_motion` |
| `TextInputStyle` | `TextInput` | `fill`†, `text_style`†, `placeholder_color`†, `border_width`†, `border_color`†, `padding`†, `selection_color`†, `selection_radius`†, `cursor_color`†, `cursor_width`†, `cursor_blink_period`, `shadow`†, `sharp`, `path`†, `hover_motion`, `focus_motion`, `cursor`† |
| `TextAreaStyle` | `TextArea` | `fill`†, `text_style`†, `syntax`, `border_width`†, `border_color`†, `padding`†, `selection_color`†, `selection_radius`†, `current_line_fill`†, `cursor_color`†, `cursor_width`†, `cursor_blink_period`, `thumb_fill`†, `thumb_path`†, `scrollbar`†, `scroll_speed`, `shadow`†, `sharp`, `path`†, `hover_motion`, `focus_motion`, `cursor`† |
| `ScrollViewStyle` | `ScrollView` | `fill`†, `border_width`†, `border_color`†, `background_path`†, `track_fill`†, `thumb_fill`†, `scrollbar`†, `always_show_scrollbars`, `scroll_speed`, `clip_content`, `sharp`, `path`†, `track_path`†, `hover_motion`, `cursor`† |
| `SelectBoxStyle` | `SelectBox` | `fill`†, `text_style`†, `placeholder_color`†, `border_width`†, `border_color`†, `shadow`†, `height`†, `padding_x`†, `show_chevron`, `chevron_color`†, `chevron_size`†, `chevron_thickness`†, `chevron_open_angle`†, `chevron_space`†, `dropdown_fill`†, `dropdown_border_width`†, `dropdown_border_color`†, `dropdown_padding`†, `dropdown_max_height`†, `dropdown_screen_margin`†, `dropdown_shadow`†, `item_height`†, `item_inset_x`†, `item_padding_x`†, `item_fill`†, `item_text_style`†, `item_corner_radius`†, `show_check`, `check_color`†, `check_thickness`†, `searchable`, `search_height`†, `search_gap`†, `search_padding_x`†, `search_fill`†, `search_border_width`†, `search_border_color`†, `search_corner_radius`†, `search_text_style`†, `search_placeholder`, `sharp`, `path`†, `hover_motion`, `focus_motion`, `chevron_motion`, `item_hover_motion`, `open_stiffness`, `open_damping`, `cursor`† |
| `TabsStyle` | `Tabs` | `height`†, `tab_padding_x`†, `tab_min_width`†, `gap`†, `padding_x`†, `fill`†, `border_width`†, `border_color`†, `shadow`†, `text_style`†, `tab_fill`†, `indicator_fill`†, `indicator`, `indicator_border_width`†, `indicator_border_color`†, `indicator_shadow`†, `pill_inset_y`†, `sharp`, `path`†, `indicator_path`†, `tab_path`†, `hover_motion`, `indicator_motion`, `cursor`† |
| `DividerStyle` | `Divider` | `fill`†, `thickness`†, `margin`†, `inset`†, `sharp`, `path`†, `hover_motion` |
| `LabelStyle` | `Label` | `padding`†, `align_x`, `align_y`, `fill`†, `text_color`†, `border_width`†, `border_color`†, `sharp`, `path`†, `hover_motion` |

Presets: `ButtonStyle::{from_theme, primary, outline, ghost, danger}`, `BadgeStyle::{.., tinted(theme, color)}`, `CardStyle::{.., interactive}`, `CheckboxStyle::circle`, `RadioButtonStyle::outlined`, `SwitchStyle::square`, `SliderStyle::thick`, `ProgressBarStyle::colored(theme, color)`, `TextInputStyle::filled`, `TextAreaStyle::code`, `ScrollViewStyle::panel`, `SelectBoxStyle::filled`, `TabsStyle::{segmented, underline}`, `DividerStyle::{faint, strong, rounded}`, `LabelStyle::highlighted`.

### ShadowStyle

```rust
pub struct ShadowStyle {
    pub color: Color,
    pub blur_radius: f32,
    pub offset: [f32; 2],
    pub spread: f32,
}

let shadow = ShadowStyle::new(Color::BLACK.with_alpha(0.3), 12.0, [0.0, 4.0]).spread(2.0);
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

### Fills, Gradients & Images

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

Images work the same way as a solid color or gradient, load once, then use anywhere a `Fill` is expected:

```rust
let logo = ui.load_image("assets/logo.png"); // PNG/JPEG/WebP via the `image` crate
let fill = Fill::Image(logo);
```

`ImageHandle::width()` / `height()` / `size()` return the image's natural pixel dimensions, handy for sizing a widget to match it.

### Custom Shapes / Paths

Every built-in widget draws a rounded rect or an ellipse (`MeasurablePath::Rect` / `Ellipse`), but the renderer also supports arbitrary shapes via `MeasurablePath::Free(BezPath)`, using [`kurbo`](https://docs.rs/kurbo)'s `BezPath` for the outline:

```rust
use glacex::{Color, Fill, MeasurablePath};
use kurbo::BezPath;

let mut star = BezPath::new();
star.move_to((160.0, 90.0));
star.line_to((188.0, 152.0));
// ...remaining points...
star.close_path();

ui.draw_shape(
    MeasurablePath::free(star),
    Fill::Solid(Color::hex_str("#f59e0b")),
    4.0,                          // border_width
    Color::hex_str("#1e293b"),    // border_color
    0.0,                          // blur_radius, not supported for Free paths, ignored
    false,                        // sharp
    0.0,                          // rotation, radians
);
```

Free paths are tessellated on the CPU (flatten + ear-clip) into a triangle mesh and rendered through a second, non-instanced pipeline, separate from the SDF pipeline that draws every rect/ellipse. Solid and gradient fills are supported (gradient color is evaluated per vertex, so it's exact for `Linear` and a close approximation for `Radial`/`Conic`, finer curves tessellate into more vertices and sharpen it further); `Fill::Image` isn't supported on a `Free` path yet and renders transparent, same as `GradientKind::Mesh`. Borders are drawn as a separately tessellated stroke outline (via `kurbo::stroke`), so they read as a classic SVG-style outline around the shape rather than the inset border used by rect/ellipse widgets. Free-path shapes are batched and clipped the same way as everything else, but currently always draw after all rect/ellipse shapes within the same frame, paint-order interleaving with them isn't implemented yet. Only the outer contour of a self-intersecting or multi-hole path is filled correctly; simple (non-self-intersecting) contours are the supported case.

### Theme

9 built-in palettes, switchable at runtime with `ui.set_theme(...)`.

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

glacex can expose its widget tree to assistive technology through [`accesskit`](https://accesskit.dev/), AT-SPI on Linux, UIA on Windows, NSAccessibility on macOS. It's disabled by default; turn it on when building the app:

```rust
App::new(root_widget)
    .accessibility_enabled(true)
    .run();
```

Widgets pick up a role and label automatically where it makes sense (`Button`, `Checkbox`, `TextInput`, ...). It's been checked against Orca and Accerciser on Arch Linux.

## Examples

```bash
# Dashboard-style demo (buttons, inputs, switches, sliders, logs)
cargo run --example demo

# Color/theme preview
cargo run --example example1

# Style playground
cargo run --example example2

# Simple demo application
cargo run --example reminder

# Demonstation of free shapes
cargo run --example free_shape

# Theming demonstration
cargo run --example themes
```

## Known Limitations

- `GradientKind::Mesh` is reserved but not implemented yet: it currently falls back to transparent. Use Linear, Radial, or Conic.
- `Fill::Image` shares one atlas slot with no packing/eviction yet, so only one image is supported at a time. A real atlas packer is planned for 0.2.0.
- `MeasurablePath::Free` (see [Custom Shapes / Paths](#custom-shapes--paths)) doesn't support `Fill::Image`, blur/shadow, or holes/self-intersection, and doesn't interleave paint order with rect/ellipse shapes sharing the same clip rect: it always draws after them.

## Documentation

- [Architecture & Rendering](docs/architecture.md): `wgpu`, SDF shaders, the frame lifecycle.
- [Widget Reference](docs/widgets.md): every widget, its constructor, and its builder methods.
- [Layout Guide](docs/layout.md): flexbox mechanics with `taffy`.

## Contributing

Bug reports and PRs are welcome, see [CONTRIBUTING.md](CONTRIBUTING.md) for coding standards and the PR checklist.

## License

MIT. See [LICENSE](LICENSE).

Copyright (c) 2026 Artem Tsitronov and Glacex contibutors.

## If you scrolled far enough to see this

Thank you! Please, star this project, or download it in cargo.
Either way it makes my day!
I would love to hear feedback from you, or criticism.
Any help is highly appreciated.

- Artem Tsitronov
