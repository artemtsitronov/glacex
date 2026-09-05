# Changelog

All notable changes to this project are documented in this file.

## [0.1.4]

### Added
- Complete design token system in `src/theme.rs`:
  - 4-step dark-first surface ladder (`BG_CANVAS` #09090b, `SURFACE` #0f0f12, `SURFACE_SUBTLE` #141418, `SURFACE_ELEVATED` #1c1c22) inspired by Linear and Vercel.
  - Distinct `Theme::PRESSED` (#2d2c36) control state between hover and active.
  - Semi-transparent border tokens (`BORDER_FAINT` 5% alpha, `BORDER` 8% alpha, `BORDER_STRONG` 16% alpha).
  - Typography color hierarchy (`TEXT_PRIMARY` off-white #f2f2f5, `TEXT_SECONDARY` Zinc 400, `TEXT_MUTED` Zinc 500).
  - Semantic status colors (`SUCCESS` Emerald 500, `WARNING` Amber 500, `ERROR` Rose 500).
  - 4px base grid spacing constants and shadcn-aligned corner radius scale (`RADIUS_XS` through `RADIUS_LG`).
- Multi-layer shadow system in `src/shadow.rs`:
  - Ambient (wide, soft) + key light (tight, crisp) two-layer architecture.
  - Named elevation presets: `Shadow::sm()`, `Shadow::md()`, `Shadow::lg()`, and `draw_shadow_layers`.
- Text color rendering pipeline:
  - `Ui::draw_text_colored` and `Painter::draw_text_colored` for custom per-widget text colors.
  - Sub-pixel typography rendering with custom alpha blending in `glyphon`.
- `Button` component enhancements:
  - Style variants: `.primary()`, `.outline()`, `.ghost()`, `.danger()`.
  - Tactile 1px press depth offset and shadow compression for physical elevation.
  - Smooth border brightening to `Theme::BORDER_STRONG` on hover.
- `Label` typography hierarchy:
  - `.color(Color)`, `.secondary()`, `.muted()`, and `.accent()` builder methods.
- `Badge` component enhancements:
  - Role-tinted surfaces (12% alpha fill, 32% alpha border) with matching high-contrast text colors for `Success`, `Warning`, and `Error`.
- `TextInput` and `TextArea` component enhancements:
  - Animated focus-visible ring via `Motion::GENTLE` (border grows 0.5px, glow shadow blur expands 6px).
  - Placeholder support on `TextInput` (`.placeholder("...")`) rendered in `Theme::TEXT_MUTED`.
  - Configurable `text_color` and `placeholder_color`.
- `ProgressBar` component enhancements:
  - Smooth animated fill via `ProgressBarState` and `StatefulWidget` trait with stable `.id("...")`.
  - Semantic status variants: `.success()`, `.warning()`, `.error()`.
- `Card` component enhancements:
  - Style variants: `.subtle()` (inset sub-panel) and `.elevated()` (floating modal/card).
- `Divider` component enhancements:
  - `.faint()` convenience builder for ultra-subtle hairline separators (`Theme::BORDER_FAINT`).
- `Motion` design system in `src/animation.rs`:
  - Named timing constants: `MICRO` (16ms), `INSTANT` (30ms), `SNAPPY` (45ms), `FLUID` (60ms), `GENTLE` (90ms).
  - Physics spring presets: `standard_spring` (Framer 400/25), `snappy_spring` (450/32), `fluid_spring` (Apple 300/26).
  - `EaseOutQuart` easing curve and `Spring::is_settled()` rest check.

### Changed
- Refactored `examples/demo.rs` to showcase the unified design system, typography hierarchy, button variants, and subtle dividers.
- Button press depth tuned to 1px with smooth cubic easing.
- Slider hover halo expanded to 10px with active drag scale feedback (+2px thumb).
- Switch knob given micro drop shadow for physical elevation over track.

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
