# Changelog

All notable changes to this project are documented in this file.

## [0.1.4]

### Added
- Dynamic Theme Engine & 9 Built-in Presets (`src/theme.rs`):
  - White-by-default Apple & shadcn-grade luxury aesthetic (`Theme::LIGHT`).
  - 9 curated, pixel-perfect theme palettes:
    - `Theme::LIGHT` (pure white canvas `#ffffff`, zinc borders, charcoal accent).
    - `Theme::DARK` (deep linear near-black `#09090b`, electric indigo accent).
    - `Theme::CATPPUCCIN_MOCHA` (soothing dark pastel warmth).
    - `Theme::CATPPUCCIN_LATTE` (cozy daylight pastel warmth).
    - `Theme::TOKYO_NIGHT` (cyberpunk neon midnight).
    - `Theme::GRUVBOX_DARK` (retro warm groove charcoal and orange).
    - `Theme::GRUVBOX_LIGHT` (warm retro light paper canvas).
    - `Theme::NORD` (arctic cool frost slate).
    - `Theme::ROSE_PINE` (moody vintage rose aesthetic).
  - Dynamic palette switching via `Ui::set_theme(theme)` and query via `Ui::theme()`.
  - Component style factories on `Theme`: `.button_style()`, `.primary_button_style()`, `.outline_button_style()`, `.ghost_button_style()`, `.danger_button_style()`, `.card_style()`, `.card_subtle_style()`, `.card_elevated_style()`, `.checkbox_style()`, `.switch_style()`, `.slider_style()`, `.input_style()`, `.text_area_style()`.
  - Dedicated `examples/themes.rs` demonstration showcasing all 9 palettes.
- Complete design token system in `src/theme.rs`:
  - 4-step surface ladder (`bg_canvas`, `surface`, `surface_subtle`, `surface_elevated`).
  - Distinct `pressed` control state between hover and active.
  - Semi-transparent border tokens (`border_faint`, `border`, `border_strong`, `focus_border`).
  - Typography color hierarchy (`text_primary`, `text_secondary`, `text_muted`).
  - Semantic status colors (`success`, `warning`, `error`).
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
  - Smooth border brightening to `theme.border_strong` on hover.
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

- Bundled Authentic Geist & Geist Mono Fonts (`assets/fonts/`, `src/painter.rs`):
  - Bundled official Vercel Geist TTFs (`Geist-Regular.ttf`, `Geist-Medium.ttf`, `Geist-SemiBold.ttf`, `Geist-Bold.ttf`, `GeistMono-Regular.ttf`) embedded via `include_bytes!`.
  - Configured explicit `include` in `Cargo.toml` so fonts are included for users in downstream applications as well as crates.io package archives.
  - Multi-weight typography engine supporting `FontWeight::Regular`, `Medium`, `SemiBold`, and `Bold`.
  - Monospace switching with `.mono()` helper on `Label`.
  - Standardized font metrics: 14px font size with 20px line height (shadcn standard).
- Full Dynamic Theme Reactivity:
  - Eliminated static color caching on widget variants: `ButtonVariant`, `BadgeVariant`, `CardVariant`, `ProgressBarVariant`, and `LabelVariant` dynamically resolve colors against active `ui.theme()` in every `arrange()` pass.
  - Theme-aware scrollbars in `ScrollView` and `TextArea`: dynamic thumb and dragging fills ensure scrollbars are crisp and visible across both dark and light modes.
  - Added `.padding()` builder on `Card` to allow custom spacing without blowing away dynamic theme colors.
- shadcn-Grade Floating Tooltip Engine:
  - Redesigned with authentic Geist Medium 12px caption typography, 8px corner radius, theme-aware elevated surface, crisp border stroke, and 14px soft drop shadow.
- Refactored `examples/demo.rs`:
  - Authentic shadcn/ui KPI stat card layout with large metric numbers (`$1,250.00`, `1,234`, `45,678`, `4.5%`), trending growth indicator pills, and subtitle captions matching modern SaaS dashboards.
  - Two-column configuration panels with switches, sliders, inputs, and Geist Mono diagnostic logs.

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
