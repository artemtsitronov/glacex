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

a gpu-rendered, immediate-mode ui library for rust.

built from scratch with `wgpu`, `winit`, `taffy`, and `glyphon`.

**by artem tsitronov and soumalya das**

</div>

> [!warning]
> glacex is experimental. the api can change between 0.x releases. pin your dependencies if you value a quiet afternoon.

## what is glacex?

glacex lets you build desktop interfaces in rust without wrapping a browser or a native widget toolkit.

you describe your interface in rust each frame. glacex handles layout, input, persistent widget state, animation, and rendering.

* **`wgpu`** renders shapes on the gpu using instanced sdf geometry.
* **`winit`** handles the window and event loop.
* **`taffy`** handles flexbox layout.
* **`glyphon`** handles text rendering.
* **`kurbo`** provides bezier paths.

no markup, no retained widget tree, and no electron-sized dependency on a browser. just rust and a gpu doing their jobs.

## features

* gpu-rendered shapes with anti-aliasing, borders, and shadows
* text rendering with per-widget clipping
* solid colors, linear/radial/conic gradients, and image fills
* custom bezier paths
* buttons, checkboxes, switches, sliders, text inputs, text areas, and more
* flexbox layouts with `row![]` and `column![]`
* persistent widget state across frames
* hover, click, keyboard focus, and clipboard support
* spring-based animations and easing curves
* tooltips and auto-hiding scrollbars
* nine built-in themes
* optional accessibility through `accesskit`

## requirements

* rust 1.85 or newer
* a gpu backend supported by `wgpu`
* linux users need a running wayland or x11 session

## installation

from crates.io:

```bash
cargo add glacex
```

or add it to `Cargo.toml`:

```toml
[dependencies]
glacex = "0.1.95"
```

to use the latest development version:

```toml
[dependencies]
glacex = { git = "https://github.com/artemtsitronov/glacex.git", branch = "main" }
```

### linux dependencies

install the relevant development packages for your distribution.

<details>
<summary>debian / ubuntu</summary>

```bash
sudo apt install libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxkbcommon-dev libwayland-dev
```

</details>

<details>
<summary>fedora</summary>

```bash
sudo dnf install libX11-devel libXcursor-devel libXrandr-devel libXi-devel libxkbcommon-devel wayland-devel
```

</details>

<details>
<summary>arch linux</summary>

```bash
sudo pacman -S libx11 libxcursor libxrandr libxi libxkbcommon wayland
```

</details>

## quick start

a counter, because every ui library needs to prove it can count to two.

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
        let mut button = Button::new("increment").id("increment-button");

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

`App::new(root).run()` creates the window, initializes the renderer, and starts the event loop.

## core concepts

### app

`App<W: Widget>` owns the window and event loop.

```rust
App::new(root_widget)
    .update(|root| {
        let _ = root;
    })
    .run();
```

### widgets

every ui element implements `Widget`:

```rust
pub trait Widget {
    type Output;

    fn ui(&mut self, ui: &mut Ui) -> Self::Output;
}
```

widgets used in `row![]` and `column![]` also implement `Measurable`, which provides `measure` and `arrange`.

### ui context

`Ui` provides access to the current frame's input, drawing, focus, clipping, window controls, and widget state.

use it when building custom widgets or when the built-in widgets aren't enough.

### layout

use `row![]` and `column![]` for flexbox layouts powered by `taffy`.

see the [layout guide](docs/layout.md) for sizing, spacing, and alignment.

## widgets

### controls

| widget        | purpose                                 |
| ------------- | --------------------------------------- |
| `Button`      | clickable actions                       |
| `Checkbox`    | boolean selection                       |
| `RadioButton` | grouped selection                       |
| `Switch`      | on/off control                          |
| `Slider`      | numeric input                           |
| `TextInput`   | single-line text                        |
| `TextArea`    | multiline text                          |
| `SelectBox`   | dropdown selection with optional search |
| `Tabs`        | switch between sections                 |

example:

```rust
let mut button = Button::new("deploy")
    .id("deploy")
    .primary();

if button.clicked() {
    println!("deployed");
}
```

buttons support `.primary()`, `.outline()`, `.ghost()`, and `.danger()`.

stateful widgets use stable ids to preserve their state between frames. use `.state(ui)`, `.take_state(ui)`, and `.put_state(ui, state)` when you need explicit control.

### containers

| widget       | purpose                          |
| ------------ | -------------------------------- |
| `Card`       | padded content with styling      |
| `Container`  | fixed-size wrapper               |
| `ScrollView` | scrollable content               |
| `Divider`    | horizontal or vertical separator |

### displays

| widget        | purpose                          |
| ------------- | -------------------------------- |
| `Label`       | text with size and color presets |
| `Badge`       | status indicator                 |
| `ProgressBar` | progress indicator               |

## styling and themes

widgets can use built-in styles or custom style structs.

```rust
use glacex::{Button, ButtonStyle, Color, Fill};

let button = Button::new("save")
    .id("save")
    .style(ButtonStyle {
        fill: Fill::Solid(Color::hex_str("#4f46e5")),
        hover_fill: Fill::Solid(Color::hex_str("#6366f1")),
        pressed_fill: Fill::Solid(Color::hex_str("#4338ca")),
        ..Default::default()
    });
```

`Color` supports rgb, rgba, hex, hsv, alpha, blending, and lightening/darkening operations.

### themes

glacex ships with nine built-in palettes:

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

### gradients and images

gradients support linear, radial, and conic modes.

```rust
use glacex::{Color, Fill, Gradient, GradientKind, GradientStop};

let gradient = Fill::Gradient(Gradient {
    kind: GradientKind::Linear { angle: 90.0 },
    stops: vec![
        GradientStop {
            position: 0.0,
            color: Color::hex_str("#ff7e5f"),
        },
        GradientStop {
            position: 1.0,
            color: Color::hex_str("#feb47b"),
        },
    ],
});
```

images can be loaded once and reused as fills:

```rust
let logo = ui.load_image("assets/logo.png");
let fill = Fill::Image(logo);
```

supported image formats depend on the enabled formats in the `image` crate.

### custom paths

`MeasurablePath::Free` supports arbitrary bezier shapes through `kurbo`.

```rust
use glacex::{Color, Fill, MeasurablePath};
use kurbo::BezPath;

let mut path = BezPath::new();
path.move_to((160.0, 90.0));
path.line_to((188.0, 152.0));
path.line_to((132.0, 152.0));
path.close_path();

ui.draw_shape(
    MeasurablePath::free(path),
    Fill::Solid(Color::hex_str("#f59e0b")),
    0.0,
    Color::WHITE,
    0.0,
    false,
    0.0,
);
```

free paths use a separate tessellated-mesh pipeline. they support solid and gradient fills, but not image fills, blur, or shadows. see the [architecture guide](docs/architecture.md) for the rendering details.

## accessibility

glacex can expose widgets to assistive technologies through `accesskit`.

```rust
App::new(root_widget)
    .accessibility_enabled(true)
    .run();
```

on linux, this uses at-spi. windows and macos use their respective accessibility APIs.

## examples

run the examples with cargo:

```bash
cargo run --example demo
cargo run --example example1
cargo run --example example2
cargo run --example reminder
cargo run --example free_shape
cargo run --example themes
```

* `demo`: interactive controls and dashboard
* `example1`: color and theme preview
* `example2`: style playground
* `reminder`: simple application
* `free_shape`: custom paths
* `themes`: theme showcase

## known limitations

glacex is still under development. some corners are, in fact, still corners.

* mesh gradients are not implemented and currently render transparent.
* image fills currently support only one image at a time.
* free paths do not support image fills, shadows, holes, or self-intersections reliably.
* free paths render after regular shapes, so their paint order cannot yet be interleaved with rectangles and ellipses.
* the api may change between releases.

## documentation

* [architecture and rendering](docs/architecture.md)
* [widget reference](docs/widgets.md)
* [layout guide](docs/layout.md)
* [contributing](CONTRIBUTING.md)
* [api documentation](https://docs.rs/glacex)

## contributing

bug reports, testing, and pull requests are welcome.

please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. small, focused changes are easier to review and less likely to summon the debugger.

## forks

experimental fork by [soumalya das](https://github.com/programmersd21/glacex).

the fork may contain independent changes, redesigns, and ai-assisted contributions. these are not maintained or endorsed by artem tsitronov. check the fork's own history and documentation for its current state.

## license

mit. see [LICENSE](LICENSE).

copyright (c) 2026 artem tsitronov and glacex contributors.

---

if glacex is useful to you, consider starring the repository, trying it in a project, or reporting a bug. testing an experimental ui library is a great way to discover how many ways a button can misbehave.

thanks for giving it a look.
