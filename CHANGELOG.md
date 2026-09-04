# Changelog

All notable changes to Glacex are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Native OS cursor icon switching via `winit::window::CursorIcon`:
  - `Pointer` on Button, Switch, Checkbox, RadioButton, and ScrollView thumbs.
  - `Text` on TextInput and TextArea.
  - `EwResize` on Slider thumbs and active drag operations.
  - Automatic per-frame cursor restoration to `CursorIcon::Default`.
- Floating tooltip system:
  - Added `ui.show_tooltip(text)` and `ui.show_tooltip_at(text, position)`.
  - Added `.tooltip(...)` builder method to `Button`.
  - Tooltips auto-clamp to viewport edges and render above active widgets.
- Secondary and middle mouse button tracking:
  - Handled `MouseButton::Right` and `MouseButton::Middle` events.
  - New `Ui` queries: `mouse_right_pressed()`, `mouse_right_released()`, `mouse_right_pressed_this_frame()`, `mouse_middle_pressed()`, etc.
- Added `docs/` documentation set: `architecture.md`, `widgets.md`, and `layout.md`.
- Added `ROADMAP.md` and `CONTRIBUTING.md`.

### Fixed
- Fixed double mutable borrow of `*ui` in `Slider::arrange` via `take_widget_state` / `put_widget_state`.
- Refactored demo and example layouts to eliminate borrow checker conflicts across child macros.

## [0.1.1] - 2026-09-04

### Added
- Added repository URL and crate metadata to `Cargo.toml`.
- Documentation updates and cleanups in `README.md`.

## [0.1.0] - 2026-09-03

### Added
- Widgets: `Badge`, `Card`, `Checkbox`, `RadioButton`, `Switch`, `Slider`, `ProgressBar`, `Divider`, `ScrollView`, `TextArea`, `TextInput`, `Label`.
- Modern dark theme palette with electric indigo accents.
- Instanced SDF quad rendering pipeline in `wgpu` with rounded corners, borders, and drop shadows.
- Text rendering and layout via `glyphon` and `swash`.
- Flexbox layout engine via `taffy` with `row![]` and `column![]` macros.
- Stable widget focus IDs, Tab navigation, and clipboard integration via `arboard`.
