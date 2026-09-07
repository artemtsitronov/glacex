# Changelog

All notable changes to this project are documented in this file.

## [0.1.5]

### Added
- Accessibility support via `AccessKit` (AT-SPI on Linux, UIA on Windows, NSAccessibility on macOS). Opt-in via `App::accessibility_enabled(true)`, off by default.
- An overlay system — `Painter` is now split into two render passes (normal + overlay) so tooltips and other floating layers draw on top of everything else.
- `.size()` / `.width()` / `.height()` and `.padding()` on `Row` and `Column`, matching the other widgets. A row/column still hugs its content by default; an explicit `.size()` gives `.align()` real cross-axis space to center/end-align children within.
- Every widget now takes an id (optional or required, depending on the widget) so it can be addressed in the accessibility tree and by persistent state.

### Fixed
- `bind_rect_pipeline` was accidentally removed at some point — restored.
- Assorted glyphon rendering glitches.
- `Button`'s state id.
- Accessibility tree crashing on the first frame.
- Tooltip text not showing up.
- `ScrollView`'s thumb was always visible whenever content overflowed, even though hover/drag/linger state was already tracked — visibility just wasn't gated on it. It now only shows while the pointer is on the track (or dragging), lingering for `ScrollConfig::linger_seconds` (0.5s, was 0.8s) after the pointer leaves. Same fix for `TextArea`'s scrollbar.
- `TextInput`'s selection highlight and cursor weren't clipped to the field's bounds, so a selection in overflowing text could paint past the input's edges.
- `Badge` rendered invisible (no background or border at all). The rounded-rect SDF shader never clamped `corner_radius` against the shape's own half-size, and `Badge` uses `Theme::RADIUS_FULL` (9999.0) as its "fully round" sentinel — uncapped, that blew up the distance field. Clamped the radius in `shader.wgsl` so an oversized radius degrades into a pill shape instead.
- A few unnecessary borrows cleaned up along the way.

### Removed
- `ROADMAP.md` — wasn't being kept up to date and had drifted from what's actually planned.

## [0.1.4]

### Added
- Theme engine with 9 built-in palettes (`src/theme.rs`): `LIGHT` (default), `DARK`, `CATPPUCCIN_MOCHA`, `CATPPUCCIN_LATTE`, `TOKYO_NIGHT`, `GRUVBOX_DARK`, `GRUVBOX_LIGHT`, `NORD`, `ROSE_PINE`. Switch at runtime with `Ui::set_theme()`, read back with `Ui::theme()`. Each theme exposes style factories (`.button_style()`, `.card_style()`, `.checkbox_style()`, etc.) and a full token set — surface ladder (`bg_canvas`/`surface`/`surface_subtle`/`surface_elevated`), border tokens, text color hierarchy, status colors, spacing, and radius scale. `examples/themes.rs` shows all 9 side by side.
- Two-layer shadow system in `src/shadow.rs` (wide ambient layer + tight key light), with `Shadow::sm()/md()/lg()` presets.
- `Ui::draw_text_colored` / `Painter::draw_text_colored` for per-widget text colors.
- `Motion` timing constants in `src/animation.rs` — `MICRO`/`INSTANT`/`SNAPPY`/`FLUID`/`GENTLE` — plus spring presets (`standard_spring`, `snappy_spring`, `fluid_spring`) and an `EaseOutQuart` curve.
- Bundled Geist and Geist Mono fonts (`assets/fonts/`), embedded via `include_bytes!` and included in the crates.io package via `Cargo.toml`'s `include`. `FontWeight::Regular/Medium/SemiBold/Bold`, `.mono()` on `Label`, 14px/20px line height metrics.
- Style variants: `Button` gets `.primary()/.outline()/.ghost()/.danger()` plus a 1px press-depth offset; `Label` gets `.color()/.secondary()/.muted()/.accent()`; `Badge` gets tinted surfaces per status; `Card` gets `.subtle()`/`.elevated()`; `Divider` gets `.faint()`.
- Focus-visible ring on `TextInput`/`TextArea` (border + glow, animated via `Motion::GENTLE`), plus `.placeholder()` on `TextInput`.
- `ProgressBar` now animates its fill through `ProgressBarState` when given a stable `.id()`, and has `.success()/.warning()/.error()` variants.
- Widget colors (`Button`, `Badge`, `Card`, `ProgressBar`, `Label` variants) now resolve against the active theme every frame instead of being cached, so `set_theme()` actually repaints everything, including scrollbars.
- Redesigned floating tooltip: Geist Medium 12px, 8px radius, elevated surface, soft shadow.
- Reworked `examples/demo.rs` into a stat-card dashboard layout with a two-column config panel.

## [0.1.2]

### Added
- Physics/easing animation system (`src/animation.rs`): `animate_towards`, `Ease` curves, `Spring`.
- Per-frame delta time on `Ui` (`ui.dt()`).
- Window size builder on `App` (`.window_size(w, h)`).
- Hover/press animations across `Button`, `Checkbox` (animated stroke checkmark), `RadioButton`, `Switch`, `Slider`.
- Window attributes builder and custom title.
- Issue/PR templates under `.github/`.

### Changed
- Refactored `demo.rs` for a 1080p dashboard layout with dynamic cluster status updates.
- Cleaned up `docs/` and `README.md`.

## [0.1.1]

### Added
- `Badge`, `Card`, `Divider`, `ProgressBar`, and `Slider` widgets.
- New color palette (Zinc/Indigo-inspired dark theme).

### Changed
- Refactored `Button`, `Checkbox`, and `RadioButton` for consistent styling and shadow support.
- Updated `TextEditState` with `set_text`/`clear`.
- Updated the example and screenshots.

### Fixed
- `Alignment::End` was missing from the `Alignment` enum.

## [0.1.0]

Initial release.
