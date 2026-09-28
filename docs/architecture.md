# The architecture and rendering pipeline

Soo here we talk about the internal architecture, rendering pipelines, and other stuff related to the underlying work of `glacex`

## 1. What's the architecture?

Glacex's an immediate-mode library, but with persistent state.
It operates on its own, without any underlying GUI frameworks:

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

Our wonderful Glacex renders every shape as a SDF quad.
The only CPU-side thing that happens is the flattening.

### But why all the hassle?
- **No CPU tesselation**: Corner radius, borders are resolved in the shader itself with zero polygon overhead. Your CPU is minimally used.
- **Sharp Anti-Aliasing**: `smoothstep` on the SDF deliver very clean edges.
- **Single pass Shadows**: Shadows are evalueated from the same SDF without extra render passes.

### Shape Instance Data (`src/shapes.rs`)
Each shape submitted to the GPU has:
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

All transitions use frame-rate independent math!

### `Motion` - Named timing constants
`Motion` is a unit struct that exposes half-life constants:

| `Motion::MICRO` | 16ms | Single-frame color snaps |
| `Motion::INSTANT` | 30ms | Press feedback, immediate state snaps |
| `Motion::SNAPPY` | 45ms | Hover transitions, border highlights |
| `Motion::FLUID` | 60ms | Knob slides, dot scaling, progress fill |
| `Motion::GENTLE` | 90ms | Focus rings, glow shadows |

### `animate_towards(current, target, dt, half_life) -> f32`
Exponential decay toward `target`. `half_life` is seconds to close half the gap.
Used by every animated widget state (`hover_t`, `press_t`, `dot_t`, `anim_progress`, `drag_t`, `focus_t`).

### Easing Curves (`Ease`)
Static easing functions for use in timed sequences:
`EaseOutCubic`, `EaseInOutCubic`, `EaseOutExpo`, `EaseOutBack`, `EaseOutQuad`, `EaseInOutQuad`, `EaseOutQuart`, `Linear`.

### `Spring`
Physics-based spring simulation (`stiffness`, `damping`) using semi-implicit Euler integration.
Construct with `Spring::with_physics(initial, stiffness, damping)`. Check `spring.is_settled()` to skip updates when at rest.
Presets:
- `Motion::standard_spring()` (400 stiffness, 25 damping): Framer Motion standard UI preset.
- `Motion::snappy_spring()` (450 stiffness, 32 damping): Fast with zero overshoot.
- `Motion::fluid_spring()` (300 stiffness, 26 damping): Apple fluid control feel.

### `lerp(a, b, t) -> f32`
Standard linear interpolation helper.

### `dt` on `Ui`
`Ui::dt()` returns elapsed seconds since the previous frame (clamped to 1..=100ms).
Widgets read `dt` once at the top of `arrange` before borrowing mutable state.

## 4. Scissor rects and draw batching

- Widgets push and pop scissor rectangles via `ui.push_clip()` and `ui.pop_clip()`
- Rects share the same clip bounds
- `glyphon` clip independently, preventing text overflow

## 5. Filling system

Gradient and images bake onto a dedicated GPU ramp texture alias:
- New gradients/images are sampled into an atlas row on first use
- Gradients/images cache by content hash.

`Fill` has three options:
- `Solid(Color)`: Solid filling
- `Gradient(Gradient)`: Gradient fillinf
- `Image(ImageHandle)`: Image filling

## 8. Design Token System & Themes (`src/theme.rs`)

Glacex ships 9 built-in theme presets, that can be swapped at runtime with `ui.set_theme(theme)`

### Surface Elevation Hierarchy
- **Canvas (`theme.bg_canvas`)**: Root window backdrop.
- **Surface (`theme.surface`)**: Standard panel, card, and container background.
- **Subtle (`theme.surface_subtle`)**: Grouped sub-containers, inputs, and control tracks.
- **Elevated (`theme.surface_elevated`)**: Tooltips, popovers, and floating overlays.

### Shadow Architecture (`src/shadow.rs`)
Depth is expressed through multi-layered shadows combining an ambient layer (wide, soft) with a key light layer (tight, crisp):
- `Shadow::sm()` -- resting controls (buttons, inputs)
- `Shadow::md()` -- cards and panels
- `Shadow::lg()` -- elevated tooltips and overlays
- `theme.shadow_sm()`, `theme.shadow_md()`, `theme.shadow_lg()` dynamically tailor blur and alpha to the current palette.

## 9. Accessibility (`src/accessibility.rs`)

Accessibility is opt-in (`App::accessibility_enabled(true)`) and layered on top of the normal frame loop rather than baked into it, via [`accesskit`](https://accesskit.dev/) / `accesskit_winit`.

- **`SharedTree`**: an `Arc<Mutex<Vec<(NodeId, Node)>>>` shared between the render thread and `accesskit`'s platform adapter thread. Every widget that calls `Ui::register_accessible` during `arrange` appends itself here for the current frame. `Ui::begin_frame` clears the list, so it never accumulates stale entries from previous frames.
- **`build_update`**: wraps the current node list in a synthetic `Role::Window` root, sets that root's `children` to every registered node id (accesskit requires every non-root node to be reachable from the root), and returns a `TreeUpdate`.
- **`AccessibilityActivationHandler`** / **`AccessibilityDeactivationHandler`**: called by the platform backend when an assistive technology attaches or detaches. On Linux this only happens once the desktop's `ScreenReaderEnabled` AT-SPI flag is set (normally by a running screen reader), which is a common point of confusion when testing with an inspector like Accerciser instead of an actual screen reader.
- **`AccessibilityActionHandler`**: receives action requests from the AT client (e.g. "invoke this button"). Currently just logged — wiring it back into widget state is app-specific and left to the caller.
- 
Each widget's `NodeId` is a hash of its own id string (`hash_id`, `src/widget.rs`), so two widgets sharing an id — including two anonymous `Card`/`Container`/`Divider`/`ProgressBar` instances that both fall back to the same default id — collide in the tree.

## 10. ID-ing system

Each widget has an ID, that defaults to a [`uuid`](https://github.com/uuid-rs/uuid). It can be set to a user-defined one via the `.id()` builder.

[ATTENTION]: For non-user-defined IDs, `Widget::new()` generate a UUID every time it's called. So this means that the persistent state doesn't work anymore for widgets!
To avoid this, set an explicit ID for stateful widgets.
