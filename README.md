<div align="center">

![demo](screenshots/demo.png)

# glacex

<p align="center">
  <a href="https://crates.io/crates/glacex"><img src="https://img.shields.io/crates/v/glacex?style=for-the-badge&logo=rust&logoColor=cdd6f4&label=crates.io&labelColor=181825&color=cba6f7" alt="crates.io"></a>
  <a href="https://docs.rs/glacex"><img src="https://img.shields.io/docsrs/glacex?style=for-the-badge&logo=docsdotrs&logoColor=cdd6f4&label=docs.rs&labelColor=181825&color=89b4fa" alt="docs.rs"></a>
  <a href="https://github.com/artemtsitronov/glacex/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-a6e3a1?style=for-the-badge&logo=opensourceinitiative&logoColor=cdd6f4&labelColor=181825" alt="MIT"></a>
  <img src="https://img.shields.io/badge/rustc-1.85+-fab387?style=for-the-badge&logo=rust&logoColor=cdd6f4&label=rustc&labelColor=181825" alt="rustc 1.85+">
  <a href="https://github.com/artemtsitronov/glacex/stargazers"><img src="https://img.shields.io/github/stars/artemtsitronov/glacex?style=for-the-badge&logo=github&logoColor=cdd6f4&label=stars&labelColor=181825" alt="GitHub stars"></a>
</p>

**a small immediate-mode ui library for rust**

built with `wgpu`, `winit`, `taffy`, and `glyphon`.

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

[ADVICE]: If you want to laugh, look at the code, you will find some funny comments :)
I hope you laugh from them and not from the code itself.

## What is glacex?

`glacex` is an immediate-mode UI library for Rust that draws its own pixels instead of wrapping a native toolkit or a browser engine:

- **`winit`** owns the window and the cross-platform event loop.
- **`wgpu`** renders every shape as an instanced signed-distance-field (SDF) quad on the GPU.
- **`glyphon`** shapes and rasterizes text into glyph atlases with per-widget clipping.
- **`taffy`** does the flexbox math for `row!`/`column!` layouts.
- **`image`** decodes the images.
- **`kurbo`** is responsible for paths.

There's no retained widget tree and no markup — you describe the UI in plain Rust every frame, and glacex measures, lays out, hit-tests, animates, and renders it.

## Features

- Custom renderer: instanced shapes (anti-aliased SDF), borders, and soft drop shadows, batched into one draw call per shared clip rect.
- Arbitrary Bezier paths (`MeasurablePath::Free`) via a second tessellated-mesh pipeline, for shapes beyond rect/ellipse — see [Custom Shapes / Paths](#custom-shapes--paths).
- Text rendering via `glyphon` with independent clip bounds per widget.
- Fills: solid colors, gradients (linear, radial, conic), and images, cached into a GPU atlas.
- `Color` is `#[repr(C)]` + `Pod`/`Zeroable`, so it maps straight onto GPU vertex buffers. Hex, RGB, HSV, alpha blending, `lerp`, lighten/darken.
- Cursor changes (pointer, text, resize, default) driven by hover state.
- A floating tooltip layer that clamps to the viewport.
- Widgets: `Button`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `ProgressBar`, `TextInput`, `TextArea`, `ScrollView`, `Card`, `Container`, `Badge`, `Divider`, `Label`, `SelectBox`, `Tabs`.
- `row![]` / `column![]` macros backed by `taffy`, with alignment and spacing.
- Interaction: hover/press/click, secondary/middle mouse buttons, Tab/Shift+Tab focus order, double/triple-click word/line selection, clipboard via `arboard`, blinking cursor.
- Widget state persists across frames keyed by a stable string id, even though the widget itself is rebuilt every frame. Every stateful widget exposes this via `.state(ui)`/`.take_state(ui)`/`.put_state(ui, state)` (from the `StatefulWidget` trait), plus small typed accessors for the common case (`checkbox.is_checked(ui)`, `slider.value(ui)`, etc.) — see [Widgets](#widgets). `Ui::widget_state`/`take_widget_state`/`put_widget_state` are the lower-level primitives underneath, for state that isn't tied to a single widget's id.
- Animation via exponential decay (`animate_towards`) plus a small set of easing curves and spring presets, unified under named half-life constants (`Motion::INSTANT`/`SNAPPY`/`FLUID`/`GENTLE`) so transitions feel consistent across widgets.
- Draggable auto-hiding scrollbars shared by `ScrollView` and `TextArea`.
- Optional accessibility tree (AT-SPI on Linux via `accesskit`) — see [Accessibility](#accessibility).

## Requirements

- Rust 1.85+ (2024 edition).
- A GPU/driver backend `wgpu` supports (Vulkan, Metal, DirectX 12, or OpenGL ES).
- On Linux: a running Wayland or X11 session. Headless environments need a virtual display (e.g. `xvfb`) to create a window surface.

## Installation

### From crates.io

## what it is

glacex lets you build native desktop interfaces directly in rust.

```toml
[dependencies]
glacex = "0.2.0-alpha"
```

no markup. no native widget toolkit.

## features

* gpu rendering with `wgpu`
* immediate-mode widgets
* flexbox layout with `taffy`
* text rendering with `glyphon`
* buttons, inputs, sliders, tabs, selects, scroll views, cards, and more
* persistent widget state
* keyboard and mouse interaction
* tooltips and focus handling
* solid colors, gradients, and images
* custom bezier paths
* themes and per-widget styling
* animations and transitions
* optional accessibility through `accesskit`

## install

```bash
cargo add glacex
```

or:

```toml
[dependencies]
glacex = "0.2.0-alpha"
```

## quick start

```rust
use glacex::{App, Button, Color, Label, Ui, Widget, column};

struct Counter {
    count: u32,
}

impl Widget for Counter {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(18, 18, 22));

        let mut label = Label::new(format!("count: {}", self.count));
        let mut button = Button::new("increment").id("increment");

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

## building

glacex currently targets systems supported by `wgpu`.

you need:

* rust 1.85+
* a supported gpu backend
* wayland or x11 on linux

linux dependencies:

```bash
# debian / ubuntu
sudo apt install libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxkbcommon-dev libwayland-dev

# fedora
sudo dnf install libX11-devel libXcursor-devel libXrandr-devel libXi-devel libxkbcommon-devel wayland-devel

# arch
sudo pacman -S libx11 libxcursor libxrandr libxi libxkbcommon wayland
```

## widgets

glacex currently includes:

`Button` · `Checkbox` · `RadioButton` · `Switch` · `Slider` · `ProgressBar` · `TextInput` · `TextArea` · `SelectBox` · `Tabs` · `ScrollView` · `Card` · `Container` · `Badge` · `Divider` · `Label`

widgets are regular rust values, so custom widgets can implement the same `Widget` trait.

## styling

styles can be changed per widget or through built-in themes.

included themes:

`light` · `dark` · `catppuccin mocha` · `catppuccin latte` · `tokyo night` · `gruvbox dark` · `gruvbox light` · `nord` · `rose pine`

fills support:

* solid colors
* linear gradients
* radial gradients
* conic gradients
* images

custom shapes can be drawn with `kurbo` paths.

## accessibility

glacex can expose widgets through `accesskit` for assistive technologies on supported platforms.

```rust
App::new(root)
    .accessibility_enabled(true)
    .run();
```

## examples

```bash
cargo run --example demo
cargo run --example themes
cargo run --example example1
cargo run --example example2
cargo run --example reminder
cargo run --example free_shape
```

## documentation

* [architecture](docs/architecture.md)
* [widgets](docs/widgets.md)
* [layout](docs/layout.md)

for the complete api, see [docs.rs](https://docs.rs/glacex).

## contributing

bug reports, improvements, and pull requests are welcome.

see [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## license

MIT. see [LICENSE](LICENSE).

copyright © 2026 artem tsitronov and glacex contributors.
