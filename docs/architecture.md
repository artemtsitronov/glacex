# architecture

how glacex turns your `ui()` function into pixels. you don't need to know any of this to use the library -- but if you've ever wondered what your `.spacing(8.0)` goes through, welcome. this is the factory tour.

## the big picture

```
[ your ui() ]            runs every frame. builds widgets, lays out, reacts.
      |
      v
[ Ui ]                   frame state: mouse, keyboard, clipboard, focus,
      |                  widget memory (the HashMap), clip stack, theme.
      v
[ layout (taffy) ]       measure → flexbox solve → arrange. positions, nothing else.
      |
      v
[ widgets ]              hit-test input, step animations, queue draw commands.
      |                  (shapes + text -- no pixels yet, just descriptions)
      v
[ Painter ] ──┬── [ wgpu SDF pipeline ] ── shapes, borders, shadows, gradients
              └── [ glyphon ] ───────────── text
                              |
                              v
                    [ GPU command buffer → your window ]
```

immediate mode with a memory: your code re-describes the ui every frame, widget state persists in `Ui` between frames, and the `Painter` turns the accumulated draw queue into gpu work once per frame. no retained tree, no dom diffing, no virtual anything. the "diff" is that there is no diff -- just draw it again, it's 2026, gpus are fast.

event flow runs the other way: `winit` events land in `App`'s `ApplicationHandler`, which feeds them into `Ui` (`update_mouse_position`, `set_mouse_pressed`, `push_typed_text`, `mark_key_pressed`...). widgets then read that state during `arrange`. input is always exactly one frame old by the time widgets see it -- fresh enough that no human has ever noticed.

## the SDF shape pipeline (why corners are free)

almost every shape -- buttons, cards, checkboxes, sliders, badges -- is drawn as a **single gpu quad**, and the actual geometry (rounded corners, borders, ellipses) is evaluated **in the fragment shader from a signed-distance function**.

why:

* **no cpu tessellation.** corner radius and borders cost zero polygons. your cpu barely wakes up.
* **perfect anti-aliasing.** `smoothstep` on the distance field gives clean edges at any scale, any dpi.
* **single-pass shadows.** the shadow is evaluated from the same SDF -- no extra render passes, no blur pyramid.

each submitted shape becomes one `ShapeInstance` (see `src/shapes.rs`): position, size, color, corner radii, border color/width, blur, gradient/image references, clip rect, rotation, reveal fraction. instanced rendering means hundreds of widgets cost roughly one draw call per fill-type. the per-frame cpu work is essentially "fill a buffer, submit".

free bezier paths (`MeasurablePath::Free`) are the exception: they're flattened and ear-clip triangulated on the cpu and drawn through a second mesh pipeline (`vs_mesh`/`fs_mesh` in `shader.wgsl`). solid and gradient fills plus borders work; image fills, blur, and shadows don't -- SDF quads can't express arbitrary paths, and meshing a blur is nobody's idea of fun.

## text

`glyphon` handles shaping, atlas caching, and rendering. glacex adds:

* a **text-shape cache**: laid-out buffers keyed by `(text, size, line height, weight, mono)` hash, reused across the measure/draw passes instead of re-shaping the same string three times a frame. entries idle 180 frames get evicted.
* **per-widget clipping**: text is clipped to each widget's bounds independently of the shape scissor rects, so long strings don't paint over their neighbors.

fonts (Geist + Geist Mono) are embedded at compile time via `include_bytes!` -- no system-font roulette, no "works on my machine" typography.

## fills: the gradient atlas

gradients and images are baked into gpu atlas textures on first use and **cached by content hash** -- identical gradients share one atlas row forever. `Fill::Solid` needs no atlas (it's just a color in the instance data). images currently share a single atlas slot with no eviction, hence the one-image-at-a-time limit. packing multiple images is roadmap work, not physics.

## animation: exponential decay + springs

every animated widget stores a few `f32`s approaching targets (`hover_t`, `press_t`, `dot_t`, `anim_progress`, `drag_t`, `focus_t`...) stepped by:

```rust
animate_towards(current, target, dt, half_life)
```

framerate-independent exponential decay -- `half_life` is seconds to close half the gap. named presets on the `Motion` unit struct:

| preset | half-life | used for |
| ------ | --------- | -------- |
| `MICRO` | 16ms | single-frame snaps |
| `INSTANT` | 30ms | press feedback |
| `SNAPPY` | 45ms | hovers, borders |
| `FLUID` | 60ms | knobs, dots, progress |
| `GENTLE` | 90ms | focus rings, glows |

plus `Ease` curves (out-cubic, in-out, expo, back...) for timed sequences and a `Spring` (semi-implicit Euler, stiffness/damping, `is_settled()`) with three presets: standard (400/25), snappy (450/32, no overshoot), fluid (300/26). `Ui::dt()` supplies clamped delta time (1–100ms, so alt-tab pauses don't teleport your springs).

## clipping & input blocking

* `ui.push_clip(rect)` / `pop_clip()` -- nested scissor rects (intersected automatically). `ScrollView` clips its contents; text clips per-widget. cleared every `begin_frame`, so a forgotten `pop` can't poison the next frame... well, it can poison *this* frame. pop your clips.
* `ui.push_input_block(rect)` -- marks a region as "an overlay is handling this". dropdowns and modals block clicks to widgets beneath. hit-testing checks it (`Interaction::update` refuses to report hover through a blocked region), which is why clicks pass *around* open menus instead of through them.

## themes & surfaces

`Theme` is a flat color struct (see [themes & styling](themes-and-styling.md)); `ui.set_theme()` swaps it per frame. elevation is expressed structurally: `bg_canvas` → `surface` → `surface_subtle` → `surface_elevated`, with paired ambient+key shadows (`shadow_sm/md/lg`) doing the depth work. dark themes flip `on_active()` text to black when the accent is near-white, so checkmarks stay visible. small detail, large dignity.

## accessibility (opt-in)

`App::accessibility_enabled(true)` layers an `accesskit` tree on top of the normal loop:

* every widget's `arrange` calls `ui.register_accessible(self, bounds)`, appending a node for the frame. `begin_frame` clears the list -- no stale nodes, ever.
* `SharedTree` (`Arc<Mutex<...>>`) hands the node list to the platform adapter; `build_update` wraps it in a synthetic `Role::Window` root.
* activation/deactivation handlers track screen-reader attach/detach (linux: AT-SPI, which only fires once the desktop's `ScreenReaderEnabled` flag is set -- testing with an inspector instead of a real reader is the classic confusion here).
* node ids are hashes of widget ids -- which is another reason stable `.id()`s matter: two widgets sharing an id collide in the tree. anonymous `Card`/`Container`/`Divider`/`ProgressBar` instances share a fallback id, so name them if you enable accesskit.

action requests from assistive tech (e.g. "invoke this button") currently log and stop -- wiring them into widget state is app-specific and left to you.

## ids: the one structural rule

widget state lives in `Ui`'s `HashMap<String, Box<dyn Any>>`, keyed by id string. `.id("x")` sets it; omitted, `new()` mints a random uuid **per call** (i.e. per frame). random id per frame = new empty state per frame = a checkbox with anterograde amnesia. the [getting started guide](getting-started.md#the-only-rule-that-matters-give-stateful-widgets-stable-ids) says this louder.

---

that's the machine: describe → measure → hit-test → queue → shade → present, sixty times a second. total new concepts to hold in your head while using it: zero. the factory tour is over; the gift shop is `cargo run --example demo`.
