# Changelog

All notable changes to the **Glacex** UI toolkit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Dynamic Mouse Cursor Feedback**: Native OS cursor icon management via `winit::window::CursorIcon`:
  - `Pointer` (Hand) cursor on [`Button`], [`Switch`], [`Checkbox`], [`RadioButton`], and [`ScrollView`] scrollbar thumbs.
  - `Text` (I-beam) cursor on [`TextInput`] and [`TextArea`].
  - `EwResize` (horizontal resize arrows) on [`Slider`] thumb and during active thumb dragging.
  - Per-frame cursor state reset restoring `CursorIcon::Default` when hovering non-interactive regions.
- **Floating Tooltip Overlay System**:
  - Added `ui.show_tooltip(text)` and `ui.show_tooltip_at(text, position)`.
  - Added fluent `.tooltip("...")` builder to [`Button`].
  - Tooltips render on top of the scene with automatic viewport boundary clamping, dark elevated background, strong hairline border, drop shadow, and typography padding.
- **Secondary & Middle Mouse Button Support**:
  - Added input tracking for `MouseButton::Right` and `MouseButton::Middle` in event loop handling.
  - New `Ui` methods: `mouse_right_pressed()`, `mouse_right_released()`, `mouse_right_pressed_this_frame()`, `mouse_right_released_this_frame()`, `mouse_middle_pressed()`, `mouse_middle_released()`, `mouse_middle_pressed_this_frame()`, and `mouse_middle_released_this_frame()`.
- **Project Documentation & Guidelines**:
  - Added comprehensive [`ROADMAP.md`](ROADMAP.md) detailing milestones from v0.1.0 to v1.0.0 LTS.
  - Added [`CONTRIBUTING.md`](CONTRIBUTING.md) with guidelines for PRs, coding conventions, architecture patterns, and testing.

### Fixed
- **Slider UI Borrow Concurrency**: Fixed double mutable borrow of `*ui` in `Slider::arrange` using `take_widget_state` / `put_widget_state`.
- **Demo Layout Borrow Checks**: Refactored `examples/demo.rs` and other examples to avoid holding overlapping mutable borrows across row/column macros and widget queries.

---

## [0.1.1] - 2026-09-04

### Added
- **Repository Metadata**: Added repository URL (`https://github.com/artemtsitronov/glacex`) and package configuration in `Cargo.toml`.
- **Documentation**: Updated `README.md` with updated widget listings and architecture notes.

---

## [0.1.0] - 2026-09-03

### Added
- **Expanded Widget Suite**:
  - [`Badge`]: Semantic status badges (`Default`, `Outline`, `Success`, `Warning`, `Error`).
  - [`Card`]: Elevated surface container with padding, rounded corners, and drop shadows.
  - [`Checkbox`]: Toggle checkbox with checked/hover/idle styles and state tracking.
  - [`RadioButton`]: Group-aware single-selection radio controls.
  - [`Switch`]: Compact animated toggle switch with pill thumb geometry.
  - [`Slider`]: Continuous numerical range slider with draggable knob and active track fill.
  - [`ProgressBar`]: Horizontal progress indicator.
  - [`Divider`]: Horizontal and vertical layout separator rules.
  - [`ScrollView`]: Dual-axis scroll container with draggable interactive scrollbar thumbs.
  - [`TextArea`]: Multi-line text editor with line-aware cursor movement, scrolling, and selection.
  - [`TextInput`]: Single-line text input with clipboard copy/paste, word jump, and cursor blinking.
- **Modern Dark Theme (Linear / Vercel Aesthetic)**:
  - Deep canvas (`#09090b`), surface colors (`#121216`, `#18181b`, `#202026`).
  - Crisp electric indigo primary accents (`#4f46e5`, `#6366f1`).
  - Subtle hairline borders and translucent drop shadows.
- **Interactive State & Focus Management**:
  - Stable widget focus IDs (`FocusId`).
  - Tab navigation focus traversal (`advance_focus`).
  - Native clipboard integration via `arboard`.
  - Multi-click word and line selection in text controls.
- **Rendering & Pipeline**:
  - `wgpu` 30.0 GPU-accelerated rendering pipeline with WGSL SDF shaders for anti-aliased rounded boxes, borders, and blurs.
  - Sub-pixel text rendering and layout with `glyphon` 0.12 and `swash`.
  - Flexbox layout engine powered by `taffy` 0.13.
  - Macro-based declarative row and column building (`row!`, `column!`).
