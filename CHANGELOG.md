# Changelog

All notable changes to this project are documented in this file.

## [0.2.0]

### Added
- `Separator`, `Skeleton`, `Spinner`, `Kbd`, `Avatar`, and `Alert` display primitives.
- `Empty`, `Toggle`, `Tabs`, and `Accordion` with persistent immediate-mode state.
- Overlay `Dialog` and timed `Toast` primitives using the existing overlay renderer and input blocking.
- `Typography`, `AspectRatio`, `Field`, and `ButtonGroup` foundations for form, layout, and navigation components.
- `Breadcrumb`, `Pagination`, and selectable `Table` primitives for application data surfaces.
- `Carousel` and compact bar `Chart` primitives for demo and dashboard surfaces.
- Keyboard-operable `CommandPalette` with query filtering, arrow navigation, Enter selection, and Escape dismissal.
- `Calendar`, `DatePicker`, `InputOtp`, and `ToggleGroup` stateful controls with click and keyboard input.
- Shared `Menu` primitive, plus `DropdownMenu` and `ContextMenu` entry points with keyboard navigation and selection.
- `Sheet`, `Drawer`, and `Sidebar` navigation surfaces with side placement, Escape dismissal, and selected-item state.
- `Resizable` drag handle, `HoverCard`, `Progress`, and `ScrollArea` public entry points.
- `Item`, `Bubble`, `Message`, `Menubar`, and `NavigationMenu` content/navigation entry points.
- `Attachment` and `Marker` content primitives for files, tags, and compact status metadata.
- Completed the public 0.2 component-name audit: all 64 requested names are now exported, with documented aliases where existing native widgets already provide the behavior.
- Dark-theme contrast hardening: active-surface foregrounds now derive from luminance, with contrast-ratio regression tests for primary, secondary, and active text.
- Added `examples/components.rs` as a runnable 0.2 component gallery.
- Fixed standalone `Tooltip` so it only announces while hovered instead of scheduling a tooltip every frame.
- Added calendar/date regression tests for leap years, month lengths, and weekday indexing.
- Accordion disclosure now animates height and content opacity through the shared motion timing helper.
- README now points to the full 0.2 widget reference and runnable component/theme/contrast showcases.
- Added `examples/contrast.rs` for visual dark-mode contrast checks alongside the full `components` gallery.
- Added grouped runnable galleries: `foundations`, `data`, and `overlays`, covering the 0.2 component families without one oversized demo.
- Hardened overlays with valid text metrics, flat themed surfaces, explicit close buttons, mouse dismissal, and anchored menu positioning.
- Added AccessKit nodes and stable labels for accordion, calendar, OTP, menu, and sidebar controls.
- Replaced the weakest `InputGroup` and `Popover` compatibility aliases with real stateful/rendering components.
- Replaced the `Questionnaire` compatibility alias with a real multi-step question/answer control with persistent answers and arrow-key navigation.
- `Ui::time()` for lightweight ambient motion.

### Changed
- Version bumped to `0.2.0`.
- New primitives inherit the active semantic theme tokens and restrained Geist/zinc defaults.
- Layout containers now measure and center themselves when used directly through `ui()`, while preserving explicit `arrange_at()` positioning for advanced layouts.
- Demo composition now uses responsive widths, centered sub-layouts, and tighter panel sizing instead of fixed oversized dashboard columns.
- Corrected sRGB surface configuration so display-space theme colors render at their authored values instead of washing dark surfaces out.
- Switch thumbs and radio selection dots now use theme-aware active foreground colors for reliable dark-theme visibility.
- Added all-preset contrast regression coverage for primary text, secondary text, and active controls.
- Demo now opens with the light theme, presents light presets before dark presets, and uses a centered branded header bar.
- Added bundled `GeistMono Nerd Font Mono` weights for monospace labels and Nerd Font icon glyphs.
- Tab surfaces now have a visible border and stronger inactive labels; the demo uses theme-native primary and outline buttons instead of a fixed gradient.
- Consolidated active, danger, checkbox, switch, slider, radio, badge, and bubble foreground selection through contrast-aware theme tokens.
- Marker defaults now resolve its accent from the active theme instead of capturing the light-theme constant.
- Added the public `NerdIcon` helper with common navigation/action glyphs and `TextStyle` support for mono Nerd Font text, italic, underline, and strikethrough rendering.
- Added the complete generated Nerd Fonts glyph catalog, `NerdIcon::named`, `NerdIcon::all`, and `Ui::draw_nerd_icon` for theme-colored icon rendering.

## [0.1.6]

### Added
- `SelectBox` / Combobox widget (`src/select_box.rs`). Spring-animated dropdown (stiffness 420, damping 28) that slides open/closed with an ease-out-cubic curve on the clip rect. Features: trigger button with animated hover/focus states and a chevron icon that rotates 180° on open; keyboard navigation (↑/↓ arrow keys, Enter to select, Escape to close); optional type-to-filter search bar (`.searchable()`); scroll inside long option lists via mouse wheel or auto-scroll to keep the keyboard selection visible; outside-click-to-close; full theme integration via `SelectBoxStyle::from_theme()`; accessibility role `ComboBox`.
- `SelectBoxStyle` with full customization of trigger, dropdown card, item rows, and search bar. Defaults to a neumorphic zero-border design: soft surface fill, shadow-based depth, rounded corners (14px), no visible borders.
- `SelectOption` — lightweight `{ value: String, label: String }` item type.
- `SelectBoxState` — persistent widget state tracking `selected: Option<String>`, open/close spring, per-item hover animations, keyboard index, and search text.
- `Ui::draw_overlay_rect` and `Ui::draw_overlay_text_styled` — public wrappers around the existing painter overlay pass so widgets like `SelectBox` can draw their own floating surfaces without a tooltip.

### Fixed
- **Critical SelectBox click bug**: dropdown items were never clickable because the widget called `push_input_block()` to protect the dropdown area from underlying widgets, but then the item hit-test checked `is_input_blocked()` — which returned `true` for the same area, causing every item hover and click to be silently ignored. The overlay item hit-test now directly tests mouse position bounds against the items clip rect (the overlay widget owns those interactions).
- SelectBox trigger text, chevron color, and focus ring opacity all polish-tuned for softer, premium feel.
- Demo rebuilt with soft neumorphic cards — no harsh border colors, clean typography hierarchy, SelectBox-based theme picker embedded in the header.

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
