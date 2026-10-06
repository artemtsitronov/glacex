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

> glacex is early software. the api may change before 1.0.

## what it is

glacex lets you build native desktop interfaces directly in rust.

the ui is described every frame with normal rust code. layout, input, widget state, animation, and rendering are handled by the library.

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
