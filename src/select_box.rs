use crate::animation::{Motion, Spring, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::text::{FontWeight, TextStyle};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, FocusId, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::keyboard::{Key, NamedKey};
use winit::window::CursorIcon;

#[derive(Clone)]
pub struct SelectBoxStyle {
    pub fill: Stateful<Fill>,
    pub text_style: Stateful<TextStyle>,
    pub placeholder_color: Stateful<Color>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub height: Stateful<f32>,
    pub padding_x: Stateful<f32>,

    pub show_chevron: bool,
    pub chevron_color: Stateful<Option<Color>>,
    pub chevron_size: Stateful<f32>,
    pub chevron_thickness: Stateful<f32>,
    pub chevron_open_angle: Stateful<f32>,
    pub chevron_space: Stateful<f32>,

    pub dropdown_fill: Stateful<Fill>,
    pub dropdown_border_width: Stateful<f32>,
    pub dropdown_border_color: Stateful<Color>,
    pub dropdown_padding: Stateful<f32>,
    pub dropdown_max_height: Stateful<f32>,
    pub dropdown_screen_margin: Stateful<f32>,
    pub dropdown_shadow: Stateful<Option<ShadowStyle>>,

    pub item_height: Stateful<f32>,
    pub item_inset_x: Stateful<f32>,
    pub item_padding_x: Stateful<f32>,
    pub item_fill: Stateful<Color>,
    pub item_text_style: Stateful<TextStyle>,
    pub item_corner_radius: Stateful<f32>,
    pub show_check: bool,
    pub check_color: Stateful<Option<Color>>,
    pub check_thickness: Stateful<f32>,

    pub searchable: bool,
    pub search_height: Stateful<f32>,
    pub search_gap: Stateful<f32>,
    pub search_padding_x: Stateful<f32>,
    pub search_fill: Stateful<Color>,
    pub search_border_width: Stateful<f32>,
    pub search_border_color: Stateful<Color>,
    pub search_corner_radius: Stateful<f32>,
    pub search_text_style: Stateful<TextStyle>,
    pub search_placeholder: &'static str,

    pub sharp: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
    pub focus_motion: f32,
    pub chevron_motion: f32,
    pub item_hover_motion: f32,
    pub open_stiffness: f32,
    pub open_damping: f32,
    pub cursor: Stateful<CursorIcon>,
}

impl SelectBoxStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        SelectBoxStyle {
            fill: Stateful::new(Fill::Solid(theme.surface))
                .hovered(Fill::Solid(theme.hovered))
                .focused(Fill::Solid(theme.surface)),
            text_style: Stateful::new(TextStyle::new().color(theme.text_primary)),
            placeholder_color: Stateful::new(theme.text_muted),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(theme.border_strong).focused(theme.focus_border),
            shadow: Stateful::new(None),
            height: Stateful::new(36.0),
            padding_x: Stateful::new(12.0),

            show_chevron: true,
            chevron_color: Stateful::new(None),
            chevron_size: Stateful::new(4.5),
            chevron_thickness: Stateful::new(1.8),
            chevron_open_angle: Stateful::new(180.0),
            chevron_space: Stateful::new(24.0),

            dropdown_fill: Stateful::new(Fill::Solid(theme.surface_elevated)),
            dropdown_border_width: Stateful::new(1.0),
            dropdown_border_color: Stateful::new(theme.border_strong),
            dropdown_padding: Stateful::new(6.0),
            dropdown_max_height: Stateful::new(280.0),
            dropdown_screen_margin: Stateful::new(8.0),
            dropdown_shadow: Stateful::new(Some(
                ShadowStyle::new(
                    Color::BLACK.with_alpha(if theme.is_dark { 0.55 } else { 0.14 }),
                    20.0,
                    [0.0, 6.0],
                )
                .spread(5.0),
            )),

            item_height: Stateful::new(32.0),
            item_inset_x: Stateful::new(4.0),
            item_padding_x: Stateful::new(8.0),
            item_fill: Stateful::new(Color::TRANSPARENT)
                .hovered(theme.surface_subtle)
                .selected(theme.surface_subtle),
            item_text_style: Stateful::new(TextStyle::new().color(theme.text_primary)).selected(
                TextStyle::new()
                    .color(theme.text_primary)
                    .weight(FontWeight::Medium),
            ),
            item_corner_radius: Stateful::new(theme.metrics.radius_sm),
            show_check: true,
            check_color: Stateful::new(None),
            check_thickness: Stateful::new(1.8),

            searchable: false,
            search_height: Stateful::new(36.0),
            search_gap: Stateful::new(4.0),
            search_padding_x: Stateful::new(10.0),
            search_fill: Stateful::new(theme.surface_subtle),
            search_border_width: Stateful::new(1.0),
            search_border_color: Stateful::new(theme.border_strong.with_alpha(0.5)),
            search_corner_radius: Stateful::new(theme.metrics.radius_sm),
            search_text_style: Stateful::new(TextStyle::new().size(13.0, 20.0)),
            search_placeholder: "Search…",

            sharp: false,
            path: Stateful::new(Path::rect([theme.metrics.radius_md; 4])),
            hover_motion: Motion::SNAPPY,
            focus_motion: Motion::GENTLE,
            chevron_motion: Motion::SNAPPY,
            item_hover_motion: Motion::SNAPPY,
            open_stiffness: 380.0,
            open_damping: 30.0,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn filled(theme: &Theme) -> Self {
        SelectBoxStyle {
            fill: Stateful::new(Fill::Solid(theme.surface_subtle))
                .hovered(Fill::Solid(theme.hovered))
                .focused(Fill::Solid(theme.surface)),
            border_color: Stateful::new(Color::TRANSPARENT).focused(theme.focus_border),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for SelectBoxStyle {
    fn default() -> Self {
        SelectBoxStyle::from_theme(&Theme::default())
    }
}

#[derive(Debug, Clone)]
pub struct SelectOption {
    pub value: String,
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

#[derive(Clone)]
pub struct SelectBoxState {
    pub selected: Option<String>,

    pub open_t: f32,
    pub open_spring: Spring,
    pub hover_t: f32,
    pub item_hover_ts: Vec<f32>,
    pub focus_t: f32,
    pub chevron_angle: f32,

    pub open: bool,
    pub keyboard_index: i32,

    pub search_text: String,
    pub search_focus_t: f32,
    pub dropdown_hover_t: f32,

    pub scroll_offset: f32,
}

impl Default for SelectBoxState {
    fn default() -> Self {
        SelectBoxState {
            selected: None,
            open_t: 0.0,
            open_spring: Spring::with_physics(0.0, 380.0, 30.0),
            hover_t: 0.0,
            item_hover_ts: Vec::new(),
            focus_t: 0.0,
            chevron_angle: 0.0,
            open: false,
            keyboard_index: -1,
            search_text: String::new(),
            search_focus_t: 0.0,
            dropdown_hover_t: 0.0,
            scroll_offset: 0.0,
        }
    }
}

pub struct SelectBox {
    id: String,
    options: Vec<SelectOption>,
    placeholder: String,
    style: Option<SelectBoxStyle>,
    text_style: Option<TextStyle>,
    width: f32,
    tooltip: Option<String>,
    searchable: bool,
    overrides: StyleOverrides,
}

impl SelectBox {
    pub fn new(options: Vec<SelectOption>) -> Self {
        SelectBox {
            id: generate_uuid(),
            options,
            placeholder: "Select an option…".into(),
            style: None,
            text_style: None,
            width: 220.0,
            tooltip: None,
            searchable: false,
            overrides: StyleOverrides::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn style(mut self, style: SelectBoxStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn set_text_style(&mut self, text_style: Option<TextStyle>) {
        self.text_style = text_style;
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

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
        if let Some(custom) = self.text_style {
            s.text_style = s.text_style.map(|t| custom.inherit(*t));
            s.item_text_style = s.item_text_style.map(|t| custom.inherit(*t));
        }
        self.overrides.apply(
            &mut s.fill,
            &mut s.border_color,
            &mut s.border_width,
            &mut s.shadow,
        );
        s
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange(position, size, ui);
    }

    pub fn selected(&self, ui: &mut Ui) -> Option<String> {
        self.state(ui).selected.clone()
    }
    pub fn set_selected(&mut self, ui: &mut Ui, selected: Option<String>) {
        let mut state = self.take_state(ui);
        state.selected = selected;
        self.put_state(ui, state);
    }
}

style_overrides!(SelectBox);

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
        let state = ui.widget_state_or::<SelectBoxState>(&self.id, SelectBoxState::default());
        let weights = StateWeights::IDLE
            .hovered(state.hover_t)
            .focused(state.focus_t);
        [self.width, style.height.resolve(weights)]
    }

    #[allow(clippy::too_many_lines)]
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        self.check_id();
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed_frame = ui.mouse_pressed_this_frame();
        let window_size = ui.window_size();

        let mut state =
            ui.take_widget_state_or::<SelectBoxState>(&self.id, SelectBoxState::default());

        let num_options = self.options.len();
        while state.item_hover_ts.len() < num_options {
            state.item_hover_ts.push(0.0);
        }

        let trigger_previous = StateWeights::IDLE
            .hovered(state.hover_t)
            .focused(state.focus_t);
        let dropdown_previous = StateWeights::IDLE.hovered(state.dropdown_hover_t);
        let trigger_path = style.path.resolve(trigger_previous);
        let trigger_blocked = ui.is_input_blocked(mouse_pos) && !state.open;
        let in_clip = ui.point_in_current_clip(mouse_pos);
        let trigger_hovered =
            !trigger_blocked && in_clip && trigger_path(position, size).contains(mouse_pos);

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

        let search_height = style.search_height.resolve(dropdown_previous);
        let search_gap = style.search_gap.resolve(dropdown_previous);
        let search_bar_h = if style.searchable {
            search_height + search_gap * 2.0
        } else {
            0.0
        };

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

        let is_selected_option = |state: &SelectBoxState, opt_idx: usize| {
            state.selected.as_deref() == Some(self.options[opt_idx].value.as_str())
        };
        let item_weights_for = |state: &SelectBoxState, opt_idx: usize| {
            StateWeights::IDLE
                .hovered(state.item_hover_ts.get(opt_idx).copied().unwrap_or(0.0))
                .selected(StateWeights::when(is_selected_option(state, opt_idx)))
        };
        let item_heights: Vec<f32> = filtered_indices
            .iter()
            .map(|&opt_idx| style.item_height.resolve(item_weights_for(&state, opt_idx)))
            .collect();
        let item_tops: Vec<f32> = item_heights
            .iter()
            .scan(0.0, |top, h| {
                let current = *top;
                *top += h;
                Some(current)
            })
            .collect();
        let item_list_h: f32 = item_heights.iter().sum();
        let inner_padding = style.dropdown_padding.resolve(dropdown_previous);
        let content_h = search_bar_h + item_list_h + inner_padding * 2.0;
        let dropdown_h = content_h.min(style.dropdown_max_height.resolve(dropdown_previous));

        // items band stays flush against the trigger, one continuous surface
        let below_y = position[1] + size[1];
        let above_y = position[1] - dropdown_h;
        let margin = style.dropdown_screen_margin.resolve(dropdown_previous);
        let opens_upward = below_y + dropdown_h > window_size[1] - margin && above_y > margin;
        let dropdown_y = if opens_upward { above_y } else { below_y };
        let dropdown_pos = [position[0], dropdown_y];
        let dropdown_size = [size[0], dropdown_h];

        if state.open && mouse_pressed_frame {
            let in_trigger = trigger_path(position, size).contains(mouse_pos);
            let in_dropdown = mouse_pos[0] >= dropdown_pos[0]
                && mouse_pos[0] <= dropdown_pos[0] + dropdown_size[0]
                && mouse_pos[1] >= dropdown_pos[1]
                && mouse_pos[1] <= dropdown_pos[1] + dropdown_size[1];
            if !in_trigger && !in_dropdown {
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
                ui.clear_focus();
            }
        }

        let trigger_clicked = trigger_hovered && mouse_pressed_frame;
        if trigger_clicked {
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
            }
            if ui.key_pressed(Key::Named(NamedKey::Escape)) {
                state.open = false;
                state.keyboard_index = -1;
                state.search_text.clear();
            }

            if style.searchable {
                let typed = ui.typed_text();
                for ch in typed.chars() {
                    if ch == '\x08' {
                        // backspace
                        state.search_text.pop();
                    } else if !ch.is_control() {
                        state.search_text.push(ch);
                    }
                }
            }

            let keyboard_item = usize::try_from(state.keyboard_index)
                .ok()
                .filter(|&ki| count > 0 && ki < item_tops.len());
            if let Some(ki) = keyboard_item {
                let item_top = search_bar_h + inner_padding + item_tops[ki];
                let item_bot = item_top + item_heights[ki];
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

        state.open_spring.stiffness = style.open_stiffness;
        state.open_spring.damping = style.open_damping;
        state
            .open_spring
            .set_target(if state.open { 1.0 } else { 0.0 });
        state.open_spring.update(dt);
        let open_t = state.open_spring.value.clamp(0.0, 1.0);

        let open_ease = 1.0 - (1.0 - open_t).powi(3);

        let chevron_target = if state.open {
            style.chevron_open_angle.resolve(trigger_previous)
        } else {
            0.0
        };
        state.chevron_angle = animate_towards(
            state.chevron_angle,
            chevron_target,
            dt,
            style.chevron_motion,
        );

        let hover_target = if trigger_hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);

        let focus_target = if is_focused || state.open {
            1.0f32
        } else {
            0.0
        };
        state.focus_t = animate_towards(state.focus_t, focus_target, dt, style.focus_motion);

        let weights = StateWeights::IDLE
            .hovered(state.hover_t)
            .focused(state.focus_t);
        let trigger_fill = style.fill.resolve(weights);
        let trigger_border = style.border_color.resolve(weights);
        let trigger_border_width = style.border_width.resolve(weights);
        let trigger_shadow = style.shadow.resolve(weights);
        let base_text_style = style.text_style.resolve(weights);
        let placeholder_color = style.placeholder_color.resolve(weights);
        let trigger_path = style.path.resolve(weights);
        let padding_x = style.padding_x.resolve(weights);

        let dropdown_hovered = state.open
            && mouse_pos[0] >= dropdown_pos[0]
            && mouse_pos[0] <= dropdown_pos[0] + dropdown_size[0]
            && mouse_pos[1] >= dropdown_pos[1]
            && mouse_pos[1] <= dropdown_pos[1] + dropdown_size[1];
        state.dropdown_hover_t = animate_towards(
            state.dropdown_hover_t,
            StateWeights::when(dropdown_hovered),
            dt,
            style.hover_motion,
        );
        let dropdown_weights = StateWeights::IDLE.hovered(state.dropdown_hover_t);

        let label_text = state
            .selected
            .as_deref()
            .and_then(|val| self.options.iter().find(|o| o.value == val))
            .map(|o| o.label.as_str())
            .unwrap_or(self.placeholder.as_str());

        let text_color = if state.selected.is_some() {
            base_text_style.color.unwrap_or(theme.text_primary)
        } else {
            placeholder_color
        };
        let chevron = ChevronStyle {
            color: style.chevron_color.resolve(weights).unwrap_or(text_color),
            size: style.chevron_size.resolve(weights),
            thickness: style.chevron_thickness.resolve(weights),
            padding_x,
        };
        let chevron_space = if style.show_chevron {
            style.chevron_space.resolve(weights)
        } else {
            0.0
        };

        let text_clip = [
            position[0] + padding_x,
            position[1],
            position[0] + size[0] - padding_x - chevron_space,
            position[1] + size[1],
        ];
        let label_style = base_text_style.color(text_color);
        let text_y = position[1] + (size[1] - label_style.line_height) / 2.0;

        // closed trigger draws standalone, open trigger is part of the merged surface below
        if open_ease <= 0.001 {
            if let Some(shadow) = &trigger_shadow {
                draw_shadow(shadow, position, size, &trigger_path, ui);
            }
            ui.draw_shape(
                trigger_path(position, size),
                trigger_fill.clone(),
                trigger_border_width,
                trigger_border,
                0.0,
                style.sharp,
                0.0,
            );

            ui.draw_text(
                label_text,
                label_style,
                [position[0] + padding_x, text_y],
                text_clip,
            );

            if style.show_chevron {
                draw_chevron(ui, position, size, state.chevron_angle, &chevron);
            }
        }

        if trigger_hovered {
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .focused(StateWeights::when(state.open)),
                ),
            );
            if let Some(tip) = &self.tooltip {
                ui.show_tooltip(tip.clone());
            }
        }

        if open_ease > 0.001 {
            let mouse_blocked_by_others = ui.is_input_blocked(mouse_pos);

            // resized as one surface, not clipped, so corners stay rounded mid animation
            let grown_h = open_ease * dropdown_h;
            let container_pos = if opens_upward {
                [position[0], position[1] - grown_h]
            } else {
                position
            };
            let container_size = [size[0], size[1] + grown_h];

            ui.push_input_block([
                container_pos[0],
                container_pos[1],
                container_pos[0] + container_size[0],
                container_pos[1] + container_size[1],
            ]);

            let full_clip = [0.0, 0.0, window_size[0], window_size[1]];

            let visible_h = dropdown_size[1] * open_ease;
            let clip_top = if opens_upward {
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
            let dropdown_clip = [
                dropdown_clip[0].max(0.0),
                dropdown_clip[1].max(0.0),
                dropdown_clip[2].min(window_size[0]),
                dropdown_clip[3].min(window_size[1]),
            ];

            if let Some(shadow) = &style.dropdown_shadow.resolve(dropdown_weights) {
                let shadow_opacity = shadow.color.a * open_ease;
                let shadow_color = shadow.color.with_alpha(shadow_opacity);
                ui.draw_overlay_shape(
                    trigger_path(
                        [
                            container_pos[0] + shadow.offset[0] - shadow.spread,
                            container_pos[1] + shadow.offset[1] - shadow.spread,
                        ],
                        [
                            container_size[0] + shadow.spread * 2.0,
                            container_size[1] + shadow.spread * 2.0,
                        ],
                    ),
                    Fill::Solid(shadow_color),
                    0.0,
                    Color::TRANSPARENT,
                    shadow.blur_radius,
                    false,
                    full_clip,
                    0.0,
                );
            }

            // one background, cross fades trigger look into dropdown look as it grows
            let merged_fill =
                trigger_fill.lerp(&style.dropdown_fill.resolve(dropdown_weights), open_ease);
            let merged_border = trigger_border.lerp(
                style.dropdown_border_color.resolve(dropdown_weights),
                open_ease,
            );
            let dropdown_border_width = style.dropdown_border_width.resolve(dropdown_weights);
            let merged_border_width =
                trigger_border_width + (dropdown_border_width - trigger_border_width) * open_ease;

            ui.draw_overlay_shape(
                trigger_path(container_pos, container_size),
                merged_fill,
                merged_border_width,
                merged_border,
                0.0,
                style.sharp,
                full_clip,
                0.0,
            );

            // overlay layer so label and chevron sit above the merged background
            ui.draw_overlay_text(
                label_text,
                label_style,
                [position[0] + padding_x, text_y],
                text_clip,
            );
            if style.show_chevron {
                draw_chevron_overlay(ui, position, size, state.chevron_angle, &chevron, full_clip);
            }

            let mut content_y = dropdown_pos[1] + inner_padding;

            if style.searchable {
                let search_pos = [dropdown_pos[0] + inner_padding, content_y];
                let search_size = [dropdown_size[0] - inner_padding * 2.0, search_height];
                let search_border_color = style.search_border_color.resolve(dropdown_weights);
                let search_padding_x = style.search_padding_x.resolve(dropdown_weights);

                ui.draw_overlay_shape(
                    MeasurablePath::rect(
                        search_pos,
                        search_size,
                        [style.search_corner_radius.resolve(dropdown_weights); 4],
                    ),
                    Fill::Solid(style.search_fill.resolve(dropdown_weights)),
                    style.search_border_width.resolve(dropdown_weights),
                    search_border_color.with_alpha(search_border_color.a * open_ease),
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
                    placeholder_color.with_alpha(open_ease)
                } else {
                    base_text_style
                        .color
                        .unwrap_or(theme.text_primary)
                        .with_alpha(open_ease)
                };
                let search_style = style
                    .search_text_style
                    .resolve(dropdown_weights)
                    .color(search_color);
                let search_text_y =
                    search_pos[1] + (search_height - search_style.line_height) / 2.0;
                ui.draw_overlay_text(
                    &search_display,
                    search_style,
                    [search_pos[0] + search_padding_x, search_text_y],
                    [
                        search_pos[0] + search_padding_x,
                        search_pos[1],
                        search_pos[0] + search_size[0] - search_padding_x,
                        search_pos[1] + search_size[1],
                    ],
                );

                content_y += search_height + search_gap;
            }

            let items_area_top = content_y;
            let items_clip = [
                dropdown_pos[0],
                items_area_top.max(dropdown_clip[1]),
                dropdown_pos[0] + dropdown_size[0],
                (dropdown_pos[1] + dropdown_size[1]).min(dropdown_clip[3]),
            ];

            let scroll_max =
                (item_list_h + search_bar_h + inner_padding * 2.0 - dropdown_h).max(0.0);
            state.scroll_offset = state.scroll_offset.clamp(0.0, scroll_max);

            let scroll_delta = ui.scroll_delta_y();
            if !ui.scroll_consumed() && scroll_delta != 0.0 {
                let dd_hovered = mouse_pos[0] >= dropdown_pos[0]
                    && mouse_pos[0] <= dropdown_pos[0] + dropdown_size[0]
                    && mouse_pos[1] >= dropdown_pos[1]
                    && mouse_pos[1] <= dropdown_pos[1] + dropdown_size[1];
                if dd_hovered {
                    state.scroll_offset =
                        (state.scroll_offset - scroll_delta).clamp(0.0, scroll_max);
                    ui.consume_scroll();
                }
            }

            for (list_idx, &opt_idx) in filtered_indices.iter().enumerate() {
                let item_height = item_heights[list_idx];
                let item_y = content_y + item_tops[list_idx] - state.scroll_offset;

                if item_y + item_height < items_clip[1] - 1.0 || item_y > items_clip[3] + 1.0 {
                    continue;
                }

                let item_inset_x = style
                    .item_inset_x
                    .resolve(item_weights_for(&state, opt_idx));
                let item_pos = [dropdown_pos[0] + item_inset_x, item_y];
                let item_size = [dropdown_size[0] - item_inset_x * 2.0, item_height];

                // hit test only the visible part, scrolled out rows can sit under the trigger
                let item_hovered = mouse_pos[0] >= item_pos[0]
                    && mouse_pos[0] <= item_pos[0] + item_size[0]
                    && mouse_pos[1] >= item_pos[1].max(items_clip[1])
                    && mouse_pos[1] <= (item_pos[1] + item_size[1]).min(items_clip[3])
                    && !mouse_blocked_by_others;

                let is_keyboard_focused = state.keyboard_index == list_idx as i32;
                let is_selected = is_selected_option(&state, opt_idx);

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
                        style.item_hover_motion,
                    );
                }
                let item_hover_t = state.item_hover_ts.get(opt_idx).copied().unwrap_or(0.0);

                if item_hovered && mouse_pressed_frame && state.open && !trigger_clicked {
                    state.selected = Some(self.options[opt_idx].value.clone());
                    state.open = false;
                    state.keyboard_index = -1;
                    state.search_text.clear();
                }

                let item_weights = StateWeights::IDLE
                    .hovered(item_hover_t)
                    .selected(if is_selected { 1.0 } else { 0.0 });
                let item_bg = style.item_fill.resolve(item_weights);
                let item_bg = item_bg.with_alpha(item_bg.a * open_ease);

                if item_bg.a > 0.005 {
                    ui.draw_overlay_shape(
                        MeasurablePath::rect(
                            item_pos,
                            item_size,
                            [style.item_corner_radius.resolve(item_weights); 4],
                        ),
                        Fill::Solid(item_bg),
                        0.0,
                        Color::TRANSPARENT,
                        0.0,
                        false,
                        items_clip,
                        0.0,
                    );
                }

                let item_text_style = style.item_text_style.resolve(item_weights);
                let item_text_color = item_text_style.color.unwrap_or(theme.text_primary);
                let item_style = item_text_style
                    .color(item_text_color.with_alpha(item_text_color.a * open_ease));
                let item_padding_x = style.item_padding_x.resolve(item_weights);
                let text_y = item_pos[1] + (item_height - item_style.line_height) / 2.0;
                ui.draw_overlay_text(
                    &self.options[opt_idx].label,
                    item_style,
                    [item_pos[0] + item_padding_x, text_y],
                    [
                        item_pos[0] + item_padding_x,
                        items_clip[1],
                        item_pos[0] + item_size[0] - item_padding_x,
                        items_clip[3],
                    ],
                );

                if is_selected && style.show_check {
                    let check_color = style
                        .check_color
                        .resolve(item_weights)
                        .unwrap_or(item_text_color);
                    draw_check(
                        ui,
                        [
                            item_pos[0] + item_size[0] - item_padding_x - 14.0,
                            item_pos[1] + (item_height - 14.0) / 2.0,
                        ],
                        check_color.with_alpha(check_color.a * open_ease),
                        style.check_thickness.resolve(item_weights),
                        items_clip,
                    );
                }

                if item_hovered {
                    ui.set_cursor_icon(
                        style.cursor.resolve(
                            StateWeights::IDLE
                                .hovered(1.0)
                                .selected(StateWeights::when(is_selected)),
                        ),
                    );
                }
            }

            ui.pop_input_block();
        }
        ui.put_widget_state(&self.id, state);
    }
}
struct ChevronStyle {
    color: Color,
    size: f32,
    thickness: f32,
    padding_x: f32,
}

// chevron stroke endpoints in world space, shared by both chevron drawers
fn chevron_points(
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &ChevronStyle,
) -> ([f32; 2], [f32; 2], [f32; 2]) {
    let cx = position[0] + size[0] - chevron.padding_x - 8.0;
    let cy = position[1] + size[1] / 2.0;
    let arm_len = chevron.size;
    let base_angle = std::f32::consts::FRAC_PI_4;

    let arm_dx = arm_len * base_angle.cos();
    let arm_dy = arm_len * base_angle.sin();
    let half_h = arm_dy / 2.0;
    let tip_local = [0.0, half_h];
    let left_local = [-arm_dx, -half_h];
    let right_local = [arm_dx, -half_h];

    let angle_rad = angle_deg.to_radians();
    let (s, c) = angle_rad.sin_cos();
    let rotate = |p: [f32; 2]| [p[0] * c - p[1] * s, p[0] * s + p[1] * c];
    let to_world = |p: [f32; 2]| [cx + p[0], cy + p[1]];

    (
        to_world(rotate(tip_local)),
        to_world(rotate(left_local)),
        to_world(rotate(right_local)),
    )
}

fn draw_chevron(
    ui: &mut Ui,
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &ChevronStyle,
) {
    let (tip, left, right) = chevron_points(position, size, angle_deg, chevron);
    draw_line(ui, tip, left, chevron.thickness, chevron.color);
    draw_line(ui, tip, right, chevron.thickness, chevron.color);
}

// overlay version of draw_chevron, keeps it above the open dropdown
fn draw_chevron_overlay(
    ui: &mut Ui,
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &ChevronStyle,
    clip: [f32; 4],
) {
    let (tip, left, right) = chevron_points(position, size, angle_deg, chevron);
    draw_line_overlay(ui, tip, left, chevron.thickness, chevron.color, clip);
    draw_line_overlay(ui, tip, right, chevron.thickness, chevron.color, clip);
}

fn draw_check(ui: &mut Ui, origin: [f32; 2], color: Color, thickness: f32, clip: [f32; 4]) {
    let x = origin[0];
    let y = origin[1];

    draw_line_overlay(
        ui,
        [x + 2.0, y + 7.0],
        [x + 5.0, y + 10.0],
        thickness,
        color,
        clip,
    );
    draw_line_overlay(
        ui,
        [x + 5.0, y + 10.0],
        [x + 11.0, y + 3.0],
        thickness,
        color,
        clip,
    );
}

fn draw_line(ui: &mut Ui, a: [f32; 2], b: [f32; 2], thickness: f32, color: Color) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.1 {
        return;
    }
    let angle = dy.atan2(dx);
    let mid_x = (a[0] + b[0]) / 2.0;
    let mid_y = (a[1] + b[1]) / 2.0;

    ui.draw_shape(
        MeasurablePath::rect(
            [mid_x - len / 2.0, mid_y - thickness / 2.0],
            [len, thickness],
            [thickness / 2.0; 4],
        ),
        Fill::Solid(color),
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        angle,
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
    let angle = dy.atan2(dx);
    let mid_x = (a[0] + b[0]) / 2.0;
    let mid_y = (a[1] + b[1]) / 2.0;

    ui.draw_overlay_shape(
        MeasurablePath::rect(
            [mid_x - len / 2.0, mid_y - thickness / 2.0],
            [len, thickness],
            [thickness / 2.0; 4],
        ),
        Fill::Solid(color),
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        clip,
        angle,
    );
}

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
