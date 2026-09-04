# Glacex Architecture and Rendering Pipeline

This document details the internal architecture, GPU execution model, and rendering pipeline of `glacex`.

## 1. High-Level Architecture

Glacex operates as an immediate-mode, GPU-accelerated UI framework:

```
[ User Application Code ]
          | (every frame via Widget::ui)
          v
       [ Ui ]  <-- Mouse / Keyboard / Clipboard / dt (winit + arboard)
          |
  +-------+--------+
  | Layout (Taffy) |
  +-------+--------+
          | (Position & Size bounds)
          v
      [ Widget ]
   (Hit-testing, animation step, draw calls)
          |
   Ui::draw_rect() / Ui::draw_text()
          |
          v
     [ Painter ]
  +------+------------------------------+
  v                                     v
[ wgpu SDF Quad Pipeline ]   [ glyphon Text Renderer ]
          |                                |
          +----------------+---------------+
                           v
                  [ GPU Command Buffer ]
                           |
                           v
                  [ Native Window Surface ]
```

## 2. SDF Instanced Quad Pipeline

Glacex renders every shape (button backgrounds, card surfaces, checkmarks, scrollbar thumbs) as a Signed Distance Field quad evaluated per-fragment in `src/shader.wgsl`. No CPU-side polygon tessellation happens.

### Why SDF Quad Rendering
- **No CPU Tessellation**: Corner rounding and borders are resolved in the fragment shader with zero polygon overhead.
- **Sharp Anti-Aliasing**: `smoothstep` over the SDF gradient delivers sub-pixel-clean edges.
- **Single-Pass Soft Shadows**: Drop shadows evaluate from the same SDF without extra blur render passes.

### Quad Instance Data (`src/shapes.rs`)
Each rectangle submitted to the GPU contains:
- `position`: `[f32; 2]`
- `size`: `[f32; 2]`
- `color`: `Color` (solid or gradient base)
- `corner_radius`: `f32`
- `border_width`: `f32`
- `border_color`: `Color`
- `blur_radius`: drop shadow soft radius
- `fill_kind`: `0.0` solid, `1.0` linear, `2.0` radial, `3.0` conic
- `gradient_angle`, `gradient_row`, `gradient_center`: gradient parameters

## 3. Animation System (`src/animation.rs`)

All widget transitions use frame-rate independent math — no hardcoded frame counts.

### `animate_towards(current, target, dt, half_life) -> f32`
Exponential decay toward `target`. `half_life` is seconds to close half the gap.
Used by every animated widget state (`hover_t`, `press_t`, `dot_t`, `anim_progress`).

### Easing Curves (`Ease`)
Static easing functions for use in timed sequences:
`EaseOutCubic`, `EaseInOutCubic`, `EaseOutExpo`, `EaseOutBack`, `EaseOutQuad`, `EaseInOutQuad`, `Linear`.

### `Spring`
Physics-based spring simulation (`stiffness`, `damping`) using semi-implicit Euler integration.
Use for overshooting, elastic, or bouncy effects beyond the built-in decay.

### `lerp(a, b, t) -> f32`
Standard linear interpolation helper.

### `dt` on `Ui`
`Ui::dt()` returns elapsed seconds since the previous frame (clamped to 1..=100ms).
Widgets read `dt` once at the top of `arrange` before borrowing mutable state.

## 4. Widget Animation Pattern

All animated widgets follow this pattern:
1. Cache `let dt = ui.dt()` before borrowing state.
2. Get or create state struct (e.g. `CheckboxState`, `SwitchState`, `ButtonState`).
3. Advance animation fields with `animate_towards(current, target, dt, half_life)`.
4. Copy animated scalars out of the state borrow.
5. Use the animated scalars to interpolate fill colors (`Color::lerp`) and layout values.

## 5. Scissor Rects and Draw Batching

- Widgets push and pop scissor rectangles with `ui.push_clip(rect)` / `ui.pop_clip()`.
- Rectangles sharing the same clip bounds pack into a single instanced `draw` call.
- `glyphon` text submissions clip independently, preventing overflow outside `ScrollView` or `Card` boundaries.

## 6. Gradient Atlas System

Gradient fills bake onto a dedicated GPU ramp texture atlas:
- New gradients are sampled into an atlas row on first use.
- Gradients cache by content hash. Reusing the same definition across frames costs nothing.

## 7. Frame Lifecycle (`src/lib.rs`)

Each `WindowEvent::RedrawRequested`:
1. `ui.begin_frame()` — clears clip stack, focus registers, computes `dt`.
2. `App::update` callback runs (optional, for app-level state changes).
3. `root_widget.ui(ui)` — measures, lays out, animates, and queues all draw calls.
4. Tab navigation and floating tooltip compositing resolve.
5. `ui.render()` — flushes `Painter`, submits GPU command buffer, presents surface.
6. `ui.end_frame()` — clears per-frame input buffers and flags.
7. `window.request_redraw()` — schedules the next frame immediately (uncapped, vsync-limited by the OS compositor).
