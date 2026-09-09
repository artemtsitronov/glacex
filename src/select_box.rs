//! Animated premium SelectBox / Combobox widget.
//!
//! Renders a trigger button that opens a spring-animated floating dropdown
//! with keyboard navigation (↑/↓ arrow keys, Enter to select, Escape to
//! close) and optional type-to-filter search.  Matches shadcn/ui / Apple HIG
//! quality and motion feel — soft, borderless, neumorphic.

use crate::animation::{Motion, Spring, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::contains;
use crate::painter::FontWeight;
use crate::shadow::ShadowStyle;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, FocusId, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::keyboard::{Key, NamedKey};
use winit::window::CursorIcon;

// ---------------------------------------------------------------------------
// Style
// ---------------------------------------------------------------------------

/// Visual style configuration for a [`SelectBox`].
#[derive(Debug, Clone)]
pub struct SelectBoxStyle {
    /// Trigger button resting fill.
    pub fill: Fill,
    /// Trigger button hover fill.
    pub hover_fill: Fill,
    /// Trigger button focused / open fill.
    pub focus_fill: Fill,
    /// Text color for the selected value label.
    pub text_color: Color,
    /// Placeholder text color (nothing selected yet).
    pub placeholder_color: Color,
    /// Trigger border width (0 = borderless neumorphic style).
    pub border_width: f32,
    /// Trigger resting border color.
    pub border_color: Color,
    /// Trigger hover border color.
    pub hover_border_color: Color,
    /// Trigger focus/open border color.
    pub focus_border_color: Color,
    /// Trigger corner radius.
    pub corner_radius: f32,
    /// Trigger height.
    pub height: f32,
    /// Horizontal padding inside the trigger.
    pub padding_x: f32,

    // Dropdown card
    /// Dropdown surface fill.
    pub dropdown_fill: Fill,
    /// Dropdown border width.
    pub dropdown_border_width: f32,
    /// Dropdown border color.
    pub dropdown_border_color: Color,
    /// Dropdown corner radius.
    pub dropdown_corner_radius: f32,
    /// Vertical gap between trigger bottom and dropdown top.
    pub dropdown_gap: f32,
    /// Maximum visible dropdown height before it clips/scrolls.
    pub dropdown_max_height: f32,
    /// Soft drop shadow on the dropdown card.
    pub dropdown_shadow: Option<ShadowStyle>,

    // Items
    /// Per-item height.
    pub item_height: f32,
    /// Horizontal padding inside items.
    pub item_padding_x: f32,
    /// Item resting text color.
    pub item_text_color: Color,
    /// Hovered item background.
    pub item_hover_fill: Color,
    /// Selected / active item background.
    pub item_active_fill: Color,
    /// Selected item text color.
    pub item_active_text_color: Color,
    /// Item corner radius.
    pub item_corner_radius: f32,

    // Search bar
    /// Show a search / filter input at the top of the dropdown.
    pub searchable: bool,
    /// Search bar height.
    pub search_height: f32,
    /// Search field background.
    pub search_fill: Color,
    /// Search placeholder text.
    pub search_placeholder: &'static str,
}

impl Default for SelectBoxStyle {
    fn default() -> Self {
        SelectBoxStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            focus_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            text_color: Theme::TEXT_PRIMARY,
            placeholder_color: Theme::TEXT_MUTED,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            hover_border_color: Color::TRANSPARENT,
            focus_border_color: Theme::FOCUS_BORDER,
            corner_radius: Theme::RADIUS_LG,
            height: 38.0,
            padding_x: 14.0,

            dropdown_fill: Fill::Solid(Theme::SURFACE_ELEVATED),
            dropdown_border_width: 0.0,
            dropdown_border_color: Color::TRANSPARENT,
            dropdown_corner_radius: 14.0,
            dropdown_gap: 6.0,
            dropdown_max_height: 300.0,
            dropdown_shadow: Some(ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.14,
                },
                blur_radius: 28.0,
                offset: [0.0, 8.0],
            }),

            item_height: 34.0,
            item_padding_x: 10.0,
            item_text_color: Theme::TEXT_PRIMARY,
            item_hover_fill: Theme::SURFACE_SUBTLE,
            item_active_fill: Theme::HOVERED,
            item_active_text_color: Theme::TEXT_PRIMARY,
            item_corner_radius: 9.0,

            searchable: false,
            search_height: 36.0,
            search_fill: Theme::SURFACE_SUBTLE,
            search_placeholder: "Search…",
        }
    }
}

impl SelectBoxStyle {
    /// Resolves a style from the active theme — the preferred way to style
    /// a `SelectBox` so it stays consistent with the rest of the UI.
    pub fn from_theme(theme: &Theme) -> Self {
        // Soft, inset neumorphic look: slightly darker surface on light,
        // slightly lighter on dark. No hard borders — depth via shadow only.
        let trigger_fill = if theme.is_dark {
            theme.surface_subtle
        } else {
            Color {
                r: theme.surface_subtle.r * 0.975,
                g: theme.surface_subtle.g * 0.975,
                b: theme.surface_subtle.b * 0.975,
                a: 1.0,
            }
        };
        let hover_fill = theme.hovered;

        SelectBoxStyle {
            fill: Fill::Solid(trigger_fill),
            hover_fill: Fill::Solid(hover_fill),
            focus_fill: Fill::Solid(trigger_fill),
            text_color: theme.text_primary,
            placeholder_color: theme.text_muted,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            hover_border_color: Color::TRANSPARENT,
            focus_border_color: theme.focus_border.with_alpha(0.4),
            corner_radius: Theme::RADIUS_LG,
            height: 38.0,
            padding_x: 14.0,

            dropdown_fill: Fill::Solid(theme.surface_elevated),
            dropdown_border_width: 0.0,
            dropdown_border_color: Color::TRANSPARENT,
            dropdown_corner_radius: 14.0,
            dropdown_gap: 6.0,
            dropdown_max_height: 300.0,
            dropdown_shadow: Some(ShadowStyle {
                color: if theme.is_dark {
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.70,
                    }
                } else {
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.13,
                    }
                },
                blur_radius: 32.0,
                offset: [0.0, 10.0],
            }),

            item_height: 34.0,
            item_padding_x: 10.0,
            item_text_color: theme.text_primary,
            item_hover_fill: theme.surface_subtle,
            item_active_fill: theme.hovered,
            item_active_text_color: theme.text_primary,
            item_corner_radius: 9.0,

            searchable: false,
            search_height: 36.0,
            search_fill: theme.surface_subtle,
            search_placeholder: "Search…",
        }
    }
}

// ---------------------------------------------------------------------------
// Option item
// ---------------------------------------------------------------------------

/// A single option entry inside a [`SelectBox`] dropdown.
#[derive(Debug, Clone)]
pub struct SelectOption {
    /// Machine-readable value (used as the selected identifier).
    pub value: String,
    /// Human-readable display label.
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        SelectOption {
            value: value.into(),
            label: label.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Per-widget persistent state for a [`SelectBox`].
#[derive(Clone)]
pub struct SelectBoxState {
    /// Currently selected value, if any.
    pub selected: Option<String>,

    // Animation
    /// `0.0` = closed, `1.0` = fully open.
    pub open_t: f32,
    /// Spring driving the open/close scale + opacity.
    pub open_spring: Spring,
    /// Hover interpolation on the trigger button.
    pub hover_t: f32,
    /// Per-item hover animation values (keyed by index).
    pub item_hover_ts: Vec<f32>,
    /// Focus ring glow intensity.
    pub focus_t: f32,
    /// Chevron rotation animation (degrees, 0 = down, -180 = up).
    pub chevron_angle: f32,

    // Keyboard / interaction
    /// Whether the dropdown is currently open.
    pub open: bool,
    /// Keyboard-highlighted item index (−1 = none).
    pub keyboard_index: i32,

    // Search
    pub search_text: String,
    /// Animation for search bar fade-in.
    pub search_focus_t: f32,

    // Scroll offset inside the dropdown (pixels from top)
    pub scroll_offset: f32,
}

impl Default for SelectBoxState {
    fn default() -> Self {
        SelectBoxState {
            selected: None,
            open_t: 0.0,
            open_spring: Spring::with_physics(0.0, 420.0, 28.0),
            hover_t: 0.0,
            item_hover_ts: Vec::new(),
            focus_t: 0.0,
            chevron_angle: 0.0,
            open: false,
            keyboard_index: -1,
            search_text: String::new(),
            search_focus_t: 0.0,
            scroll_offset: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Widget
// ---------------------------------------------------------------------------

/// Premium animated select / combobox widget.
///
/// # Example
/// ```no_run
/// use glacex::{SelectBox, SelectOption, Ui};
///
/// fn build(ui: &mut Ui) {
///     let options = vec![
///         SelectOption::new("light", "Light"),
///         SelectOption::new("dark", "Dark"),
///         SelectOption::new("mocha", "Catppuccin Mocha"),
///     ];
///
///     let mut sel = SelectBox::new("theme_picker", options)
///         .placeholder("Choose a theme…")
///         .width(220.0);
///
///     sel.arrange_at([40.0, 40.0], ui);
///
///     if let Some(val) = sel.selected() {
///         println!("selected: {val}");
///     }
/// }
/// ```
pub struct SelectBox {
    id: String,
    options: Vec<SelectOption>,
    placeholder: String,
    style: Option<SelectBoxStyle>,
    width: f32,
    tooltip: Option<String>,
    searchable: bool,
    show_clear: bool,
}

impl SelectBox {
    /// Create a new `SelectBox` with a stable id and a list of options.
    pub fn new(id: impl Into<String>, options: Vec<SelectOption>) -> Self {
        SelectBox {
            id: id.into(),
            options,
            placeholder: "Select an option…".into(),
            style: None,
            width: 220.0,
            tooltip: None,
            searchable: false,
            show_clear: false,
        }
    }

    /// Override the placeholder text shown when nothing is selected.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Override the trigger width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Enable type-to-filter search inside the dropdown.
    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn show_clear(mut self, show: bool) -> Self {
        self.show_clear = show;
        self
    }

    pub fn style(mut self, style: SelectBoxStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Show a tooltip when the trigger is hovered.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    // -----------------------------------------------------------------------
    // Read-back helpers — call after `ui()` / `arrange()`
    // -----------------------------------------------------------------------

    /// Returns the currently selected value, if any.
    pub fn selected(&self) -> Option<&str> {
        // We can't hold a reference into state after calling `arrange`, so the
        // caller should read directly via `ui.widget_state::<SelectBoxState>(id).selected`.
        // This is a convenience shim that always returns `None` on the widget
        // struct; use the state API for the real value.
        None
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn focus_id(&self) -> FocusId {
        FocusId::new(&self.id)
    }

    fn resolved_style(&self, theme: &Theme) -> SelectBoxStyle {
        let mut s = self
            .style
            .clone()
            .unwrap_or_else(|| SelectBoxStyle::from_theme(theme));
        if self.searchable {
            s.searchable = true;
        }
        s
    }

    /// Convenience arrange-at entry-point matching other widgets.
    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange(position, size, ui);
    }
}

impl Widget for SelectBox {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for SelectBox {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        [self.width, style.height]
    }

    #[allow(clippy::too_many_lines)]
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed_frame = ui.mouse_pressed_this_frame();
        let window_size = ui.window_size();

        // ------------------------------------------------------------------ //
        // 1. Load state
        // ------------------------------------------------------------------ //
        let mut state =
            ui.take_widget_state_or::<SelectBoxState>(&self.id, SelectBoxState::default());

        // Grow item_hover_ts if more options were added
        let num_options = self.options.len();
        while state.item_hover_ts.len() < num_options {
            state.item_hover_ts.push(0.0);
        }

        // ------------------------------------------------------------------ //
        // 2. Trigger hit-test (only blocked by other overlays, not self)
        // ------------------------------------------------------------------ //
        let trigger_blocked = ui.is_input_blocked(mouse_pos) && !state.open;
        let in_clip = ui.point_in_current_clip(mouse_pos);
        let trigger_hovered =
            !trigger_blocked && in_clip && contains(position, size, style.corner_radius, mouse_pos);

        // Register focus
        ui.register_focusable(self.focus_id());
        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        // ------------------------------------------------------------------ //
        // 3. Dropdown geometry (computed early so we can hit-test it)
        // ------------------------------------------------------------------ //
        let search_bar_h = if style.searchable {
            style.search_height + 8.0 // bar height + bottom padding
        } else {
            0.0
        };

        // Filter options by search text
        let search_lower = state.search_text.to_lowercase();
        let filtered_indices: Vec<usize> = (0..self.options.len())
            .filter(|&i| {
                if search_lower.is_empty() {
                    true
                } else {
                    self.options[i].label.to_lowercase().contains(&search_lower)
                        || self.options[i].value.to_lowercase().contains(&search_lower)
                }
            })
            .collect();

        let item_list_h = filtered_indices.len() as f32 * style.item_height;
        let inner_padding = 6.0; // top+bottom padding inside card
        let content_h = search_bar_h + item_list_h + inner_padding * 2.0;
        let dropdown_h = content_h.min(style.dropdown_max_height);

        // Position: prefer below trigger, fall back to above if not enough room
        let gap = style.dropdown_gap;
        let below_y = position[1] + size[1] + gap;
        let above_y = position[1] - gap - dropdown_h;
        let opens_upward = below_y + dropdown_h > window_size[1] - 8.0 && above_y > 8.0;
        let dropdown_y = if opens_upward { above_y } else { below_y };
        let dropdown_pos = [position[0], dropdown_y];
        let dropdown_size = [size[0], dropdown_h];

        // ------------------------------------------------------------------ //
        // 4. Click-outside-to-close
        // ------------------------------------------------------------------ //
        if state.open && mouse_pressed_frame {
            let in_trigger = contains(position, size, style.corner_radius, mouse_pos);
            let in_dropdown = mouse_pos[0] >= dropdown_pos[0]
                && mouse_pos[0] <= dropdown_pos[0] + dropdown_size[0]
                && mouse_pos[1] >= dropdown_pos[1]
                && mouse_pos[1] <= dropdown_pos[1] + dropdown_size[1];
            if !in_trigger && !in_dropdown {
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
                state.scroll_offset = 0.0;
                ui.clear_focus();
            }
        }

        // ------------------------------------------------------------------ //
        // 5. Trigger click → toggle open / close
        // ------------------------------------------------------------------ //
        if trigger_hovered && mouse_pressed_frame {
            if state.open {
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
                ui.clear_focus();
            } else {
                state.open = true;
                state.keyboard_index = -1;
                ui.request_focus(self.focus_id());
            }
        }

        // ------------------------------------------------------------------ //
        // 6. Keyboard navigation (only when focused/open)
        // ------------------------------------------------------------------ //
        let is_focused = ui.is_focused(self.focus_id());
        if is_focused && state.open {
            let count = filtered_indices.len() as i32;

            if ui.key_pressed(Key::Named(NamedKey::ArrowDown)) {
                state.keyboard_index = if state.keyboard_index < count - 1 {
                    state.keyboard_index + 1
                } else {
                    0
                };
            }
            if ui.key_pressed(Key::Named(NamedKey::ArrowUp)) {
                state.keyboard_index = if state.keyboard_index > 0 {
                    state.keyboard_index - 1
                } else {
                    count - 1
                };
            }
            if ui.key_pressed(Key::Named(NamedKey::Enter))
                && state.keyboard_index >= 0
                && state.keyboard_index < count
            {
                let opt_idx = filtered_indices[state.keyboard_index as usize];
                state.selected = Some(self.options[opt_idx].value.clone());
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
                state.scroll_offset = 0.0;
            }
            if ui.key_pressed(Key::Named(NamedKey::Escape)) {
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
                state.scroll_offset = 0.0;
            }

            // Type-to-search
            if style.searchable {
                let typed = ui.typed_text().to_owned();
                for ch in typed.chars() {
                    if ch == '\x08' {
                        // Backspace
                        state.search_text.pop();
                    } else if !ch.is_control() {
                        state.search_text.push(ch);
                    }
                }
            }

            // Scroll to keep keyboard selection visible
            if state.keyboard_index >= 0 && count > 0 {
                let ki = state.keyboard_index as f32;
                let item_top = search_bar_h + inner_padding + ki * style.item_height;
                let item_bot = item_top + style.item_height;
                let visible_h = dropdown_h - search_bar_h;
                let scroll_max = (content_h - dropdown_h).max(0.0);
                if item_top < state.scroll_offset {
                    state.scroll_offset = item_top;
                } else if item_bot > state.scroll_offset + visible_h {
                    state.scroll_offset = item_bot - visible_h;
                }
                state.scroll_offset = state.scroll_offset.clamp(0.0, scroll_max);
            }
        }

        // ------------------------------------------------------------------ //
        // 7. Open/close spring animation
        // ------------------------------------------------------------------ //
        state
            .open_spring
            .set_target(if state.open { 1.0 } else { 0.0 });
        state.open_spring.update(dt);
        let open_t = state.open_spring.value.clamp(0.0, 1.0);

        // Ease the open_t through a snappy cubic curve for even crisper feel
        let open_ease = 1.0 - (1.0 - open_t).powi(3); // ease-out-cubic

        // Chevron angle: 0° = pointing down (closed), 180° = pointing up (open)
        let chevron_target = if state.open { 180.0f32 } else { 0.0 };
        state.chevron_angle =
            animate_towards(state.chevron_angle, chevron_target, dt, Motion::SNAPPY);

        // Trigger animations
        let hover_target = if trigger_hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

        let focus_target = if is_focused || state.open {
            1.0f32
        } else {
            0.0
        };
        state.focus_t = animate_towards(state.focus_t, focus_target, dt, Motion::GENTLE);

        // ------------------------------------------------------------------ //
        // 8. Draw trigger button (neumorphic: soft shadow, no border)
        // ------------------------------------------------------------------ //
        let trigger_fill = if let (Fill::Solid(idle), Fill::Solid(hov), Fill::Solid(foc)) =
            (&style.fill, &style.hover_fill, &style.focus_fill)
        {
            let blended = idle.lerp(*hov, state.hover_t * 0.6);
            Fill::Solid(blended.lerp(*foc, state.focus_t * open_ease * 0.3))
        } else if state.open || is_focused {
            style.focus_fill.clone()
        } else if trigger_hovered {
            style.hover_fill.clone()
        } else {
            style.fill.clone()
        };

        // Soft inset shadow for neumorphic depth (drawn before trigger rect)
        if state.focus_t > 0.01 {
            let ring_alpha = state.focus_t * 0.22;
            let glow_size = [size[0] + 5.0, size[1] + 5.0];
            let glow_pos = [position[0] - 2.5, position[1] - 2.5];
            ui.draw_rect(
                glow_pos,
                glow_size,
                Fill::Solid(style.focus_border_color.with_alpha(ring_alpha)),
                style.corner_radius + 2.5,
                0.0,
                Color::TRANSPARENT,
                4.0,
                false,
                0.0,
            );
        }

        ui.draw_rect(
            position,
            size,
            trigger_fill,
            style.corner_radius,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );

        // ------------------------------------------------------------------ //
        // 9. Trigger label text
        // ------------------------------------------------------------------ //
        let label_text = state
            .selected
            .as_deref()
            .and_then(|val| self.options.iter().find(|o| o.value == val))
            .map(|o| o.label.as_str())
            .unwrap_or(self.placeholder.as_str());

        let text_color = if state.selected.is_some() {
            style.text_color
        } else {
            style.placeholder_color
        };

        let text_clip = [
            position[0] + style.padding_x,
            position[1],
            // Reserve space for chevron (16px icon + 8px gap + padding_x)
            position[0] + size[0] - style.padding_x - 24.0,
            position[1] + size[1],
        ];
        let text_y = position[1] + (size[1] - 20.0) / 2.0;

        ui.draw_text_styled(
            label_text,
            [position[0] + style.padding_x, text_y],
            text_clip,
            text_color,
            14.0,
            20.0,
            FontWeight::Regular,
            false,
        );

        let has_selection = state.selected.is_some();
        let mut clear_clicked = false;

        if self.show_clear && has_selection {
            let clear_x = position[0] + size[0] - style.padding_x - 28.0;
            let clear_y = position[1] + (size[1] - 16.0) / 2.0;
            let clear_pos = [clear_x, clear_y];
            let clear_size = [16.0, 16.0];

            let clear_hovered = contains(clear_pos, clear_size, 8.0, mouse_pos)
                && !ui.is_input_blocked(mouse_pos)
                && ui.point_in_current_clip(mouse_pos);

            if clear_hovered {
                ui.set_cursor_icon(CursorIcon::Pointer);
                let hover_bg = style.text_color.with_alpha(0.08);
                ui.draw_rect(
                    clear_pos,
                    clear_size,
                    Fill::Solid(hover_bg),
                    8.0,
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    false,
                    0.0,
                );

                if mouse_pressed_frame {
                    clear_clicked = true;
                }
            }

            let x_color = if clear_hovered {
                text_color
            } else {
                text_color.with_alpha(0.4)
            };
            let cx = clear_x + 8.0;
            let cy = clear_y + 8.0;
            let x_size = 3.2;
            draw_line(
                ui,
                [cx - x_size, cy - x_size],
                [cx + x_size, cy + x_size],
                1.6,
                x_color,
            );
            draw_line(
                ui,
                [cx - x_size, cy + x_size],
                [cx + x_size, cy - x_size],
                1.6,
                x_color,
            );
        }

        if clear_clicked {
            state.selected = None;
        }

        // Chevron icon
        let chevron_color =
            text_color.with_alpha(if state.open || is_focused { 0.85 } else { 0.5 });
        draw_chevron(
            ui,
            position,
            size,
            style.padding_x,
            state.chevron_angle,
            chevron_color,
        );

        // ------------------------------------------------------------------ //
        // 10. Tooltip
        // ------------------------------------------------------------------ //
        if trigger_hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
            if let Some(tip) = &self.tooltip {
                ui.show_tooltip(tip.clone());
            }
        }

        // ------------------------------------------------------------------ //
        // 11. Dropdown overlay (only when open_ease > 0)
        // ------------------------------------------------------------------ //
        if open_ease > 0.001 {
            // Block normal-pass input for the dropdown area so underlying
            // widgets don't receive clicks while the dropdown is visible.
            // NOTE: Items within this block do NOT use is_input_blocked(),
            // since this widget IS the overlay receiving those interactions.
            ui.push_input_block([
                dropdown_pos[0],
                dropdown_pos[1],
                dropdown_pos[0] + dropdown_size[0],
                dropdown_pos[1] + dropdown_size[1],
            ]);

            let full_clip = [0.0, 0.0, window_size[0], window_size[1]];

            // Animated clip: the dropdown "slides in" by revealing from top
            // (or bottom if opening upwards).
            let visible_h = dropdown_size[1] * open_ease;
            let clip_top = if opens_upward {
                // Slide upward: reveal from bottom edge
                dropdown_pos[1] + dropdown_size[1] - visible_h
            } else {
                dropdown_pos[1]
            };
            let dropdown_clip = [
                dropdown_pos[0] - 2.0,
                clip_top,
                dropdown_pos[0] + dropdown_size[0] + 2.0,
                dropdown_pos[1] + dropdown_size[1] + 2.0,
            ];
            // Intersect with full viewport
            let dropdown_clip = [
                dropdown_clip[0].max(0.0),
                dropdown_clip[1].max(0.0),
                dropdown_clip[2].min(window_size[0]),
                dropdown_clip[3].min(window_size[1]),
            ];

            // Ambient shadow (large, diffuse — key to neumorphic feel)
            if let Some(shadow) = &style.dropdown_shadow {
                let shadow_opacity = shadow.color.a * open_ease;
                let shadow_color = shadow.color.with_alpha(shadow_opacity);
                ui.draw_overlay_rect(
                    [
                        dropdown_pos[0] + shadow.offset[0] - 8.0,
                        dropdown_pos[1] + shadow.offset[1] - 4.0,
                    ],
                    [dropdown_size[0] + 16.0, dropdown_size[1] + 16.0],
                    Fill::Solid(shadow_color),
                    style.dropdown_corner_radius + 6.0,
                    0.0,
                    Color::TRANSPARENT,
                    shadow.blur_radius,
                    false,
                    full_clip,
                    0.0,
                );
            }

            // Dropdown card surface (no border — rely on shadow for depth)
            ui.draw_overlay_rect(
                dropdown_pos,
                dropdown_size,
                style.dropdown_fill.clone(),
                style.dropdown_corner_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                dropdown_clip,
                0.0,
            );

            // ----------------------------------------------------------------
            // Search bar (optional)
            // ----------------------------------------------------------------
            let mut content_y = dropdown_pos[1] + inner_padding;

            if style.searchable {
                let search_pos = [dropdown_pos[0] + 6.0, content_y];
                let search_size = [dropdown_size[0] - 12.0, style.search_height];

                ui.draw_overlay_rect(
                    search_pos,
                    search_size,
                    Fill::Solid(style.search_fill),
                    Theme::RADIUS_MD,
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    false,
                    dropdown_clip,
                    0.0,
                );

                let search_display = if state.search_text.is_empty() {
                    style.search_placeholder.to_string()
                } else {
                    state.search_text.clone()
                };
                let search_color = if state.search_text.is_empty() {
                    style.placeholder_color.with_alpha(open_ease)
                } else {
                    style.text_color.with_alpha(open_ease)
                };
                let search_text_y = search_pos[1] + (style.search_height - 20.0) / 2.0;
                ui.draw_overlay_text_styled(
                    &search_display,
                    [search_pos[0] + 10.0, search_text_y],
                    [
                        search_pos[0] + 10.0,
                        search_pos[1],
                        search_pos[0] + search_size[0] - 10.0,
                        search_pos[1] + search_size[1],
                    ],
                    search_color,
                    13.5,
                    20.0,
                    FontWeight::Regular,
                    false,
                );

                content_y += style.search_height + 8.0;
            }

            // Mouse wheel scroll handling
            let scroll_delta = ui.scroll_delta_y();
            let scroll_max = (content_h - dropdown_h).max(0.0);
            if scroll_delta.abs() > 0.01 {
                let in_dd = mouse_pos[0] >= dropdown_pos[0]
                    && mouse_pos[0] <= dropdown_pos[0] + dropdown_size[0]
                    && mouse_pos[1] >= dropdown_pos[1]
                    && mouse_pos[1] <= dropdown_pos[1] + dropdown_size[1];
                if in_dd {
                    state.scroll_offset =
                        (state.scroll_offset - scroll_delta * 24.0).clamp(0.0, scroll_max);
                }
            } else {
                state.scroll_offset = state.scroll_offset.clamp(0.0, scroll_max);
            }

            // ----------------------------------------------------------------
            // Items
            // ----------------------------------------------------------------
            let items_clip = [
                dropdown_pos[0] + 2.0,
                content_y.max(dropdown_clip[1]),
                dropdown_pos[0] + dropdown_size[0] - 2.0,
                (dropdown_pos[1] + dropdown_size[1] - inner_padding).min(dropdown_clip[3]),
            ];

            for (list_idx, &opt_idx) in filtered_indices.iter().enumerate() {
                let item_y = content_y + list_idx as f32 * style.item_height - state.scroll_offset;

                // Skip items fully outside the clip
                if item_y + style.item_height < items_clip[1] - 1.0 || item_y > items_clip[3] + 1.0
                {
                    continue;
                }

                let item_pos = [dropdown_pos[0] + 4.0, item_y];
                let item_size = [dropdown_size[0] - 8.0, style.item_height];

                // *** FIX: overlay items handle their OWN input — do NOT check
                // is_input_blocked here since this widget pushed that block itself.
                // We only need raw bounds hit-testing against the dropdown clip.
                let item_hovered = mouse_pos[0] >= item_pos[0]
                    && mouse_pos[0] <= item_pos[0] + item_size[0]
                    && mouse_pos[1] >= item_pos[1]
                    && mouse_pos[1] <= item_pos[1] + item_size[1]
                    && mouse_pos[1] >= items_clip[1]
                    && mouse_pos[1] <= items_clip[3];

                let is_keyboard_focused = state.keyboard_index == list_idx as i32;
                let is_selected = state
                    .selected
                    .as_deref()
                    .map(|s| s == self.options[opt_idx].value)
                    .unwrap_or(false);

                // Hover animation
                let item_hover_target = if item_hovered || is_keyboard_focused {
                    1.0f32
                } else {
                    0.0
                };
                if opt_idx < state.item_hover_ts.len() {
                    state.item_hover_ts[opt_idx] = animate_towards(
                        state.item_hover_ts[opt_idx],
                        item_hover_target,
                        dt,
                        Motion::SNAPPY,
                    );
                }
                let item_hover_t = state.item_hover_ts.get(opt_idx).copied().unwrap_or(0.0);

                // Click to select
                if item_hovered && mouse_pressed_frame {
                    state.selected = Some(self.options[opt_idx].value.clone());
                    state.open = false;
                    state.keyboard_index = -1;
                    state.search_text.clear();
                    state.scroll_offset = 0.0;
                }

                // Draw item background
                let bg_alpha = if is_selected {
                    item_hover_t * 0.7 + 0.3
                } else {
                    item_hover_t
                };
                let item_bg_color = if is_selected {
                    style.item_active_fill
                } else {
                    style.item_hover_fill
                };

                if bg_alpha > 0.005 {
                    ui.draw_overlay_rect(
                        item_pos,
                        item_size,
                        Fill::Solid(item_bg_color.with_alpha(bg_alpha * open_ease)),
                        style.item_corner_radius,
                        0.0,
                        Color::TRANSPARENT,
                        0.0,
                        false,
                        items_clip,
                        0.0,
                    );
                }

                // Draw item text
                let item_text_color = if is_selected {
                    style.item_active_text_color.with_alpha(open_ease)
                } else {
                    style.item_text_color.with_alpha(open_ease)
                };

                let text_y = item_pos[1] + (style.item_height - 20.0) / 2.0;
                ui.draw_overlay_text_styled(
                    &self.options[opt_idx].label,
                    [item_pos[0] + style.item_padding_x, text_y],
                    [
                        item_pos[0] + style.item_padding_x,
                        items_clip[1],
                        item_pos[0] + item_size[0] - style.item_padding_x,
                        items_clip[3],
                    ],
                    item_text_color,
                    14.0,
                    20.0,
                    if is_selected {
                        FontWeight::Medium
                    } else {
                        FontWeight::Regular
                    },
                    false,
                );

                // Selected checkmark
                if is_selected {
                    draw_check(
                        ui,
                        [
                            item_pos[0] + item_size[0] - style.item_padding_x - 14.0,
                            item_pos[1] + (style.item_height - 14.0) / 2.0,
                        ],
                        style.item_active_text_color.with_alpha(open_ease * 0.7),
                        items_clip,
                    );
                }

                // Set pointer cursor when hovering items
                if item_hovered {
                    ui.set_cursor_icon(CursorIcon::Pointer);
                }
            }

            ui.pop_input_block();
        }

        // ------------------------------------------------------------------ //
        // 12. Save state
        // ------------------------------------------------------------------ //
        ui.put_widget_state(&self.id, state);
    }
}

// ---------------------------------------------------------------------------
// Helpers: chevron + checkmark drawn from primitive rects
// ---------------------------------------------------------------------------

fn draw_chevron(
    ui: &mut Ui,
    position: [f32; 2],
    size: [f32; 2],
    padding_x: f32,
    angle_deg: f32,
    color: Color,
) {
    let cx = position[0] + size[0] - padding_x - 7.0;
    let cy = position[1] + size[1] / 2.0;
    let thickness = 1.8;
    let arm_len = 3.8;

    let angle_rad = angle_deg.to_radians();
    let rot = |x: f32, y: f32| -> [f32; 2] {
        let s = angle_rad.sin();
        let c = angle_rad.cos();
        [cx + x * c - y * s, cy + x * s + y * c]
    };

    // When angle_rad = 0 (closed): chevron points downward 'v'
    // Apex is at (0, 1.6), left tip at (-arm_len, -1.6), right tip at (arm_len, -1.6)
    // When angle_rad = PI (open): rotates 180° around (cx, cy) to point upward '^'
    let apex = rot(0.0, 1.6);
    let left = rot(-arm_len, -1.6);
    let right = rot(arm_len, -1.6);

    draw_line(ui, left, apex, thickness, color);
    draw_line(ui, right, apex, thickness, color);
}

fn draw_check(ui: &mut Ui, origin: [f32; 2], color: Color, clip: [f32; 4]) {
    let thickness = 1.8;
    // origin is 14x14 box, center at (+7.0, +7.0)
    let cx = origin[0] + 7.0;
    let cy = origin[1] + 7.0;

    let left = [cx - 3.8, cy + 0.3];
    let valley = [cx - 1.0, cy + 3.2];
    let right = [cx + 4.2, cy - 3.2];

    draw_line_overlay(ui, left, valley, thickness, color, clip);
    draw_line_overlay(ui, valley, right, thickness, color, clip);
}

fn draw_line(ui: &mut Ui, a: [f32; 2], b: [f32; 2], thickness: f32, color: Color) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.1 {
        return;
    }
    // RADIANS: The WGSL shader rotation takes radians
    let angle_rad = dy.atan2(dx);
    let mid_x = (a[0] + b[0]) / 2.0;
    let mid_y = (a[1] + b[1]) / 2.0;

    ui.draw_rect(
        [mid_x - len / 2.0, mid_y - thickness / 2.0],
        [len, thickness],
        Fill::Solid(color),
        thickness / 2.0,
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        angle_rad,
    );
}

fn draw_line_overlay(
    ui: &mut Ui,
    a: [f32; 2],
    b: [f32; 2],
    thickness: f32,
    color: Color,
    clip: [f32; 4],
) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.1 {
        return;
    }
    // RADIANS: The WGSL shader rotation takes radians
    let angle_rad = dy.atan2(dx);
    let mid_x = (a[0] + b[0]) / 2.0;
    let mid_y = (a[1] + b[1]) / 2.0;

    ui.draw_overlay_rect(
        [mid_x - len / 2.0, mid_y - thickness / 2.0],
        [len, thickness],
        Fill::Solid(color),
        thickness / 2.0,
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        clip,
        angle_rad,
    );
}

// ---------------------------------------------------------------------------
// StatefulWidget + Accessible
// ---------------------------------------------------------------------------

impl StatefulWidget for SelectBox {
    type State = SelectBoxState;
    fn state_id(&self) -> &str {
        &self.id
    }
    fn initial_state(&self) -> Self::State {
        SelectBoxState::default()
    }
}

impl Accessible for SelectBox {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::ComboBox
    }
    fn accessibility_label(&self) -> Option<String> {
        Some(self.placeholder.clone())
    }
}
