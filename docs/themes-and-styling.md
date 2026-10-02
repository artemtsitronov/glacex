# themes & styling

glacex has two layers of looks: **themes** (one line, repaints everything) and **styles** (per-widget costumes for control freaks -- said affectionately). plus colors, gradients, shadows, and image fills underneath it all.

## themes: the one-liner

```rust
fn ui(&mut self, ui: &mut Ui) {
    ui.set_theme(Theme::DARK);  // first line. everything below follows.
    // ...
}
```

call it every frame (it's cheap -- just copies a struct). switching themes mid-run is fully supported; `examples/themes.rs` lets you click through all nine live.

### the nine presets

| theme | species |
| ----- | ------- |
| `Theme::LIGHT` | default. clean, indigo accent |
| `Theme::DARK` | default after dark. teal accent |
| `Theme::CATPPUCCIN_MOCHA` | pastel purple haze |
| `Theme::CATPPUCCIN_LATTE` | pastel, but make it daytime |
| `Theme::TOKYO_NIGHT` | blue neon, 2am energy |
| `Theme::GRUVBOX_DARK` / `Theme::GRUVBOX_LIGHT` | retro warm, like a well-worn keyboard |
| `Theme::NORD` | frosty arctic minimalism |
| `Theme::ROSE_PINE` | soft rosy gloom |

helpers: `Theme::all()` returns all nine as a slice (perfect for a theme picker), `Theme::from_name("nord")` parses a string (accepts aliases like `"mocha"`, `"latte"`, `"white"`), `Theme::light()` / `Theme::dark()` / ... per-theme constructors.

### what a theme actually is

a flat struct of colors -- surfaces, interaction states, borders, text, semantic colors, shadows, selection:

```rust
let t: Theme = *ui.theme();
t.bg_canvas        // window backdrop
t.surface          // cards, panels
t.surface_subtle   // inputs, quiet boxes
t.surface_elevated // tooltips, popovers
t.idle / t.hovered / t.pressed   // control states
t.active           // the accent color (buttons, toggles, sliders)
t.border / t.border_strong / t.border_faint
t.text_primary / t.text_secondary / t.text_muted
t.success / t.warning / t.error
t.selection        // text-selection highlight
```

`Theme::LIGHT` doubles as a set of associated constants (`Theme::BG_CANVAS`, `Theme::BORDER`, `Theme::RADIUS_MD`...), handy for constants in your own drawing code. plus a spacing/radius scale: `RADIUS_XS/SM/MD/LG/FULL` (3/5/7/12/huge) and `SPACE_1..SPACE_8` (4/8/12/16/24/32). use them and your ui will look consistent by accident. consistency by accident is the best kind.

custom theme? it's just a struct literal -- copy `Theme::DARK`, change the fields, `ui.set_theme(mine)`. go on. nobody's stopping you.

## styles: per-widget costumes

every widget resolves its look from a `*Style` struct. no `.style()` given? it asks the current theme (`theme.button_style()`, `theme.card_style()`, ...), so **unstyled widgets automatically follow theme switches**. custom-styled widgets don't -- you hardcoded it, you own it.

two ways to customize:

**1. variant shortcuts** -- one word, theme-aware, always safe:

```rust
Button::new("go").primary()     // .outline() .ghost() .danger()
Badge::new("ok").success()      // .secondary() .outline() .warning() .error()
Card::new(&mut body).elevated() // .subtle()
Label::new("hey").heading()     // .caption() .title() .mono() .secondary() ...
Divider::horizontal(200.0).faint()
ProgressBar::new(0.5).warning()
```

prefer these. they survive theme changes and they were designed by someone with taste.

**2. full `*Style` structs** -- every field, your responsibility:

```rust
let mut btn = Button::new("fancy").style(ButtonStyle {
    fill: Fill::Solid(Color::hex_str("#4f46e5")),
    hover_fill: Fill::Solid(Color::hex_str("#6366f1")),
    pressed_fill: Fill::Solid(Color::hex_str("#4338ca")),
    text_color: Color::WHITE,
    border_width: 1.0,
    border_color: Color::WHITE.with_alpha(0.2),
    padding: [14.0, 8.0],
    shadow: Some(ShadowStyle { color: Color::rgba(79, 70, 229, 76), blur_radius: 16.0, offset: [0.0, 4.0] }),
    sharp: false,
    path: Path::rect([Theme::RADIUS_MD; 4]),  // rounded-rect shape
});
```

start from `..Default::default()` or from `theme.button_style()` and override a field or two -- much less typing, much less regret. `examples/example2.rs` is an interactive playground for exactly this: type style attributes, watch widgets change live.

the `path` field deserves a callout: it's a `Path`, a size-aware shape factory. `Path::rect([r; 4])` gives per-corner rounding, `Path::ellipse(degrees)` gives an ellipse (radio buttons use this), `Path::from_fn(|pos, size| ...)` gives anything you can compute. change a button from rounded to pill to circle by swapping one field. power.

style structs exist for: `ButtonStyle`, `CardStyle`, `CheckboxStyle`, `RadioButtonStyle`, `SwitchStyle`, `SliderStyle`, `TextInputStyle`, `TextAreaStyle`, `SelectBoxStyle`, `TabsStyle`, `BadgeStyle`, `ProgressBarStyle`, `ScrollViewStyle`. each widget's page in the [widget reference](widgets.md) shows its fields in context.

## color: the fun part

`Color` is linear 0–1 RGBA with more constructors than strictly necessary:

```rust
Color::rgb(79, 70, 229)          // 0-255, opaque
Color::rgba(79, 70, 229, 0.5)   // with float alpha
Color::hex_str("#4f46e5")       // from a string ("#" optional)
Color::hex(0x4F46E5)            // from a number
Color::hsv(244.0, 0.65, 0.9)    // hue 0-360. for the adventurous.
Color::WHITE / Color::BLACK / Color::TRANSPARENT / Color::RED / ...
```

and modifiers that return new colors (everything is `Copy`, nothing is precious):

```rust
base.with_alpha(0.5)   // same color, new transparency
base.lighten(0.15)     // nudge toward white (hover states love this)
base.darken(0.2)       // nudge toward black (pressed states love this)
a.lerp(b, 0.5)         // halfway between a and b (animations live on this)
```

the classic hover/pressed trio, as seen in `examples/example1.rs`:

```rust
fill:         Fill::Solid(color),
hover_fill:   Fill::Solid(color.lighten(0.15)),
pressed_fill: Fill::Solid(color.darken(0.2)),
```

three lines, works for literally any base color, in any theme. memorize it.

## fills: solid, gradient, image

every shape-owning style takes a `Fill`:

```rust
Fill::Solid(Color::hex_str("#4f46e5"))

Fill::Gradient(Gradient {
    kind: GradientKind::Linear { angle: 45.0 },   // degrees. also Radial / Conic / Mesh
    stops: vec![
        GradientStop { position: 0.0, color: Color::hex_str("#6366f1") },
        GradientStop { position: 1.0, color: Color::hex_str("#ec4899") },
    ],
})

Fill::Image(logo_handle)  // from ui.load_image("assets/logo.png") -- see below
```

* linear gradients take an angle in degrees. radial takes a `center` + `radius`. conic sweeps around a `center`. mesh (4-corner blend) exists as a type but currently renders transparent -- it's on the roadmap, not in your window.
* gradients are baked to a gpu atlas row on first use and cached by content hash. free.

### images

```rust
// once, in on_start (runs a single time before the first frame):
fn on_start(&mut self, ui: &mut Ui) {
    self.logo = Some(ui.load_image("assets/logo.png"));
}

// every frame, use it as a fill:
Card::new(&mut label).style(CardStyle {
    fill: Fill::Image(self.logo.unwrap()),
    ..CardStyle::subtle()
})
```

`ImageHandle` knows its size: `.width()`, `.height()`, `.size()`, `.scale(0.2)` (proportional). current limit: **one image at a time** (single shared atlas, no eviction yet). `examples/demo.rs` shows a webp-filled card; the loader handles png/jpeg/webp via the `image` crate.

## shadows

one soft layer:

```rust
ShadowStyle { color: Color::rgba(30, 33, 54, 20), blur_radius: 12.0, offset: [0.0, 3.0] }
```

or theme-matched pairs -- ambient (wide, soft) + key (tight, crisp):

```rust
theme.shadow_sm()  // resting controls: buttons, inputs
theme.shadow_md()  // cards and panels
theme.shadow_lg()  // floating things: tooltips, popovers, your ego
```

each returns `[ShadowStyle; 2]`. pass one (or both, via `draw_shadow_layers`) into any style's `shadow: Some(...)` -- or `None` for the flat-design purists.

## free shapes (bezier, no widgets)

sometimes you don't want a widget -- you want a star. `Ui::draw_shape` draws any `MeasurablePath` directly:

```rust
use kurbo::BezPath;

let mut star = BezPath::new();
star.move_to((160.0, 90.0));
star.line_to((188.0, 152.0));
star.line_to((132.0, 152.0));
star.close_path();

ui.draw_shape(
    MeasurablePath::free(star),
    Fill::Solid(Color::hex_str("#f59e0b")),
    0.0,                 // border width
    Color::TRANSPARENT,  // border color
    0.0,                 // blur
    false,               // sharp corners?
    0.0,                 // rotation (radians)
);
```

`MeasurablePath::open(centerline, thickness)` draws stroke-only lines (progress tracks, squiggles) with a `reveal: 0.0..=1.0` fraction via `ui.draw_open_shape`. free paths render through a separate tessellated-mesh pipeline: solid + gradient fills and borders work; image fills, blur, and shadows don't (yet). full tour: `cargo run --example free_shape`.

---

tldr: `ui.set_theme()` for the big lever, variant shortcuts for the small ones, `*Style` structs when you're particular, `Color` math for hover states, `Fill` for gradients/images, `draw_shape` when widgets feel restrictive. next: build something that isn't in this list -- [custom widgets](custom-widgets.md).
