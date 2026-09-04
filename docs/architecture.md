# Glacex Architecture and Rendering Pipeline

This document details the internal architecture, GPU execution model, and rendering pipeline of `glacex`.

## 1. High-Level Architecture

Glacex operates as an immediate-mode, GPU-accelerated UI framework:

```
[ User Application Code ]
          │ (every frame via Widget::ui)
          ▼
       [ Ui ]  ◄── Mouse / Keyboard / Clipboard (winit + arboard)
          │
  ┌───────┴────────┐
  │ Layout (Taffy) │
  └───────┬────────┘
          │ (Position & Size bounds)
          ▼
      [ Widget ]
   (Hit-testing & draw calls)
          │
   Ui::draw_rect() / Ui::draw_text()
          │
          ▼
     [ Painter ]
  ┌───────┴────────────────────────┐
  ▼                                ▼
[ wgpu SDF Quad Pipeline ]   [ glyphon Text Renderer ]
          │                                │
          └────────────────┬───────────────┘
                           ▼
                  [ GPU Command Buffer ]
                           │
                           ▼
                  [ Native Window Surface ]
```

## 2. SDF Instanced Quad Pipeline

Unlike GUI frameworks that triangulate shapes into vertex meshes using CPU tessellation libraries, Glacex renders shapes using Signed Distance Fields (SDF) evaluated per fragment in `src/shader.wgsl`.

### Why SDF Quad Rendering
- **No CPU Tessellation**: No polygon generation overhead for rounded corners or borders.
- **Sharp Anti-Aliasing**: Evaluated directly in the fragment shader with sub-pixel screen derivatives (`fwidth` and `smoothstep`).
- **Single-Pass Soft Shadows**: Drop shadows and Gaussian blur falloffs calculate from the same distance field function without extra blur passes.

### Quad Instance Data Layout (`src/shapes.rs`)
Each rectangle submitted to the GPU contains:
- `rect_pos`: `[f32; 2]`
- `rect_size`: `[f32; 2]`
- `fill`: Solid color or gradient index parameters.
- `corner_radius`: `f32`
- `border_width`: `f32`
- `border_color`: `[f32; 4]`
- `blur_radius`: Drop shadow blur size.
- `clip_rect`: `[f32; 4]` (`[min_x, min_y, max_x, max_y]`)

## 3. Scissor Rects and Draw Batching

Drawing operations batch automatically by clipping region:
- Widgets push and pop nested scissor rectangles using `ui.push_clip(rect)` and `ui.pop_clip()`.
- Rectangles sharing the same clip bounds combine into a single instanced `wgpu::RenderPass::draw` call.
- Text submissions via `glyphon` take clipped bounding boxes, preventing text overflow outside container boundaries (such as inside a `ScrollView` or `Card`).

## 4. Gradient Atlas System

Gradient fills (`GradientKind::Linear`, `Radial`, `Conic`) bake onto a dedicated GPU ramp texture atlas:
- When a new gradient is registered, its color stops are sampled into an atlas row.
- Gradients cache by their content hash. Reusing the same gradient across widgets or frames avoids re-baking.

## 5. Frame Lifecycle (`lib.rs`)

1. **`WindowEvent::RedrawRequested`**:
   - `ui.begin_frame()` clears frame state (clip stack, focus registers, hit-test flags).
   - If registered, `App::update` runs with `&mut root_widget`.
   - `root_widget.ui(ui)` executes widget measurement, layout, and draw queues.
   - Cursor updates, Tab navigation, and floating tooltips are resolved.
   - `ui.render()` flushes the `Painter`, records GPU command encoders, and presents the surface.
   - `ui.end_frame()` clears input buffers and resets single-frame flags.
   - A redraw request is queued for the next frame.
