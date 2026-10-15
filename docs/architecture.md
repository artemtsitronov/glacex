# The architecture and rendering pipeline

Soo this page is about how `glacex` works on the inside. The architecture, the rendering pipelines and other stuff like that.

## 1. What's the architecture?

Glacex is an immediate mode library, but it keeps state between frames.
It doesn't sit on top of any other GUI framework, it does everything by itself.

```
[ Your code ]
          | (every frame)
          v
       [ Ui ]  <-- Mouse / Keyboard / Clipboard / Window
          |
  +-------+--------+
  | Layout (Taffy) |
  +-------+--------+
          | (position & size bounds)
          v
      [ Widget ]
   (Hit-testing, animation step, draw calls)
          |
   Queues a shape or text
          |
          v
     [ Painter ]
  +------+------------------------------+
  v                                     v
[ wgpu SDF Shape Pipeline ]   [ glyphon Text Renderer ]
          |                                |
          +----------------+---------------+
                           v
                  [ GPU Command Buffer ]
                           |
                           v
                  [ Your Window Surface ]
```

## 2. SDF Shape Pipeline

Every shape in Glacex gets drawn as an SDF quad.
The only thing done on the CPU is the flattening.

### But why all the hassle?
- **No CPU tessellation**. Corner radius and borders are worked out in the shader, so there are no polygons to build and the CPU barely does anything.
- **Nice anti aliasing**. Running `smoothstep` on the SDF gives really clean edges.
- **Shadows in one pass**. Shadows come from the same SDF, so there are no extra render passes.

### Shape Instance Data (`src/shapes.rs`)
Every shape sent to the GPU has these fields
- `position`: `[f32; 2]`
- `size`: `[f32; 2]`
- `color`: `Color` (`src/color.rs`)
- `corner_radius`: `[f32; 4]`
- `border_color`: `Color`
- `rander_params`: `[f32; 4]`
- `shape_params`: `[f32; 4]`
- `gradient_center`: `[f32; 2]`
- `image_uv`: `[f32; 4]`
- `sdf_uv`: `[f32; 4]`
- `path_params`: `[f32; 4]`

## 3. Animation (`src/animation.rs`)

All the animations work the same no matter what your frame rate is.

### `Motion` and the timing constants
`Motion` is a unit struct with a bunch of half life constants

| `Motion::MICRO` | 16ms | Single-frame color snaps |
| `Motion::INSTANT` | 30ms | Press feedback, immediate state snaps |
| `Motion::SNAPPY` | 45ms | Hover transitions, border highlights |
| `Motion::FLUID` | 60ms | Knob slides, dot scaling, progress fill |
| `Motion::GENTLE` | 90ms | Focus rings, glow shadows |

### `animate_towards(current, target, dt, half_life) -> f32`
Moves toward `target` with exponential decay. `half_life` is how many seconds it takes to get halfway there.
Every animated bit of widget state uses it (`hover_t`, `press_t`, `dot_t`, `anim_progress`, `drag_t` and `focus_t`).

### Easing Curves (`Ease`)
Plain easing functions for when you want a timed animation.
You get `EaseOutCubic`, `EaseInOutCubic`, `EaseOutExpo`, `EaseOutBack`, `EaseOutQuad`, `EaseInOutQuad`, `EaseOutQuart` and `Linear`.

### `Spring`
A spring with `stiffness` and `damping`, simulated with semi implicit Euler.
Make one with `Spring::with_physics(initial, stiffness, damping)`. If `spring.is_settled()` is true it's not moving anymore and you can skip updating it.
There are some presets too
- `Motion::standard_spring()` has 400 stiffness and 25 damping. It's the standard UI preset from Framer Motion.
- `Motion::snappy_spring()` has 450 stiffness and 32 damping. It's fast and doesn't overshoot at all.
- `Motion::fluid_spring()` has 300 stiffness and 26 damping. It feels like Apple's fluid controls.

### `lerp(a, b, t) -> f32`
Just normal linear interpolation.

### `dt` on `Ui`
`Ui::dt()` gives you how many seconds passed since the last frame. It's clamped between 1 and 100ms.
Widgets grab `dt` once at the start of `arrange`, before they borrow their state as mutable.

## 4. Scissor rects and draw batching

- Widgets push and pop scissor rects with `ui.push_clip()` and `ui.pop_clip()`
- Rects with the same clip bounds get drawn together
- `glyphon` clips text on its own, so text can't spill out

## 5. Filling system

Gradients and images get baked into their own GPU texture.
- The first time a gradient or image is used it gets sampled into a row of the atlas
- They're cached by their content, so the same one is only baked once

`Fill` can be one of three things
- `Solid(Color)` for a solid color
- `Gradient(Gradient)` for a gradient
- `Image(ImageHandle)` for an image

## 8. Design Token System & Themes (`src/theme.rs`)

Glacex comes with 9 themes and you can swap them at runtime with `ui.set_theme(theme)`

### Surface Elevation Hierarchy
- **Canvas (`theme.bg_canvas`)** is the background of the whole window.
- **Surface (`theme.surface`)** is the normal background for panels, cards and containers.
- **Subtle (`theme.surface_subtle`)** is for things grouped inside other things, inputs and control tracks.
- **Elevated (`theme.surface_elevated`)** is for tooltips, popovers and anything floating.

### Shadow Architecture (`src/shadow.rs`)
Depth comes from shadows with two layers. One is wide and soft (ambient) and the other is tight and crisp (key light).
- `Shadow::sm()` is for controls like buttons and inputs
- `Shadow::md()` is for cards and panels
- `Shadow::lg()` is for tooltips and overlays
- `theme.shadow_sm()`, `theme.shadow_md()` and `theme.shadow_lg()` change the blur and alpha to fit the current theme.

## 9. Accessibility (`src/accessibility.rs`)

Accessibility is off unless you turn it on with `App::accessibility_enabled(true)`. It runs on top of the normal frame loop instead of being built into it, and it uses [`accesskit`](https://accesskit.dev/) and `accesskit_winit`.

- **`SharedTree`** is an `Arc<Mutex<Vec<(NodeId, Node)>>>` that the render thread and the `accesskit` adapter thread both use. Any widget that calls `Ui::register_accessible` in `arrange` adds itself to it for that frame. `Ui::begin_frame` clears it every frame, so old entries don't pile up.
- **`build_update`** puts all the current nodes under a fake `Role::Window` root and makes every registered node a child of it. accesskit needs every node to be reachable from the root. Then it returns a `TreeUpdate`.
- **`AccessibilityActivationHandler`** and **`AccessibilityDeactivationHandler`** get called when a screen reader or similar tool connects or disconnects. On Linux that only happens once the desktop sets the `ScreenReaderEnabled` AT-SPI flag, which usually means a screen reader is running. This trips people up a lot when they test with something like Accerciser instead of a real screen reader.
- **`AccessibilityActionHandler`** gets action requests from the assistive tool, like "click this button". For now it only logs them, because hooking them up to widget state depends on your app so that part is up to you.

Each widget's `NodeId` is a hash of its id string (`hash_id` in `src/widget.rs`). So if two widgets have the same id they'll clash in the tree. `Card`, `Container`, `Divider` and `ProgressBar` without an id don't get registered at all.

## 10. ID-ing system

Every widget has an id. By default it's a random [`uuid`](https://github.com/uuid-rs/uuid), but you can set your own with `.id()`.

[ATTENTION] If you don't set an id, `Widget::new()` makes a new UUID every time you call it. That means the widget can't find its state from last frame and persistent state just stops working!
So give stateful widgets their own id.
