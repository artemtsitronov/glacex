# Changelog

All notable changes to this project are documented in this file.

## [0.1.4]

### Added
- `Motion` struct in `src/animation.rs` with named half-life timing constants: `INSTANT` (30ms), `SNAPPY` (45ms), `FLUID` (60ms), `GENTLE` (90ms). Provides a shared motion language across all widgets.
- `EaseOutQuart` easing curve.
- `Spring::with_physics(initial, stiffness, damping)` constructor and `Spring::is_settled()` check.
- `ProgressBar` now supports smooth animated fill via `ProgressBarState` and the `StatefulWidget` trait. Assign a stable `.id("...")` to enable fill interpolation across frames.
- `Button` tactile press depth: 1px Y offset and shadow compression on press for physical elevation feedback. Border brightens on hover via `Theme::BORDER_STRONG`.
- `Checkbox` progressive stroke animation: left leg draws from 0 to 35%, right leg from 30 to 100%, with slight overlap for a fluid feel.
- `Switch` micro thumb shadow, 3-way idle/hover/active track color blend, and border transition toward `Theme::ACTIVE` when enabled.
- `Slider` thumb scale-up (+2px) on drag for tactile feedback. Hover glow halo expands to 10px radius. Shadow grows during drag.
- `TextInput` and `TextArea` animated focus ring: border width grows 0.5px on focus, focus glow shadow color and blur radius animate in/out via `Motion::GENTLE` (90ms).
- `RadioButton` border transitions on hover and selection, using `Motion::FLUID` for the dot and `Motion::SNAPPY` for hover.

### Changed
- Default spring stiffness tuned to 320/26 (from 280/24) for a snappier out-of-the-box feel.
- `Switch` hover state now uses `hover_t` for a distinct idle/hover/active three-way blend.
- `Slider` active halo glow radius increased to 10px on hover (was 8px).

## [0.1.2]

### Added
- Physics and easing animation system (`src/animation.rs`): `animate_towards`, `Ease` curves, and `Spring`.
- Per-frame delta time tracking in `Ui` (`ui.dt()`).
- Window size builder and configuration on `App` (`.window_size(w, h)`).
- Smooth interactive transitions across controls:
  - `Button`: smooth color interpolation on hover and press.
  - `Checkbox`: diagonal vector checkmark with animated progressive stroke drawing and smooth fill transitions.
  - `RadioButton`: smooth animated dot scaling and background blend.
  - `Switch`: gliding knob animation and smooth track color transitions.
  - `Slider`: animated hover glow halo on thumb.
- Window attributes builder and custom title configuration.
- Issue and pull request templates under `.github/`.

### Changed
- Refactored `demo.rs` for 1080p dashboard layout with dynamic cluster region status updates.
- Refined and cleaned all documentation in `docs/` and `README.md`.

## [0.1.1]

### Added
- `Badge`, `Card`, `Divider`, `ProgressBar`, and `Slider` widgets.
- New color palette (Zinc/Indigo-inspired dark theme).

### Changed
- Refactored `Button`, `Checkbox`, and `RadioButton` for consistent
  styling and shadow support.
- Updated `TextEditState` with `set_text`/`clear`.
- Updated the example and screenshots.

### Fixed
- `Alignment::End` was missing from the `Alignment` enum.

## [0.1.0]

Initial release.
