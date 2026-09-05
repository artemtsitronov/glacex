# Changelog

All notable changes to this project are documented in this file.

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
