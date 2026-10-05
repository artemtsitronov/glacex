use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::text::TextStyle;
use crate::text_edit::TextEditState;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, FocusId, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Clone)]
pub struct TextInputStyle {
    pub fill: Stateful<Fill>,
    pub text_style: Stateful<TextStyle>,
    pub placeholder_color: Stateful<Color>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub padding: Stateful<[f32; 2]>,
    pub selection_color: Stateful<Color>,
    pub selection_radius: Stateful<f32>,
    pub cursor_color: Stateful<Color>,
    pub cursor_width: Stateful<f32>,
    pub cursor_blink_period: f32,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
    pub focus_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

pub(crate) fn focus_ring(shadow: Option<ShadowStyle>, focus_color: Color) -> Option<ShadowStyle> {
    shadow.map(|s| {
        s.color(focus_color.with_alpha(0.25))
            .blur(s.blur_radius + 6.0)
    })
}

pub(crate) fn caret_visible(idle_seconds: f32, period: f32) -> bool {
    period <= 0.0 || idle_seconds < 0.3 || (idle_seconds % period) < period * 0.5
}

impl TextInputStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        let shadow = Some(theme.shadow_sm()[0]);
        TextInputStyle {
            fill: Stateful::new(Fill::Solid(theme.surface)),
            text_style: Stateful::new(TextStyle::new().color(theme.text_primary)),
            placeholder_color: Stateful::new(theme.text_muted),
            border_width: Stateful::new(1.0).focused(1.5),
            border_color: Stateful::new(theme.border_strong).focused(theme.focus_border),
            padding: Stateful::new([10.0, 10.0]),
            selection_color: Stateful::new(theme.selection),
            selection_radius: Stateful::new(0.0),
            cursor_color: Stateful::new(theme.active),
            cursor_width: Stateful::new(2.0),
            cursor_blink_period: 1.0,
            shadow: Stateful::new(shadow).focused(focus_ring(shadow, theme.focus_border)),
            sharp: false,
            path: Stateful::new(Path::rect([6.0; 4])),
            hover_motion: Motion::SNAPPY,
            focus_motion: Motion::GENTLE,
            cursor: Stateful::new(CursorIcon::Text),
        }
    }

    pub fn filled(theme: &Theme) -> Self {
        let base = Self::from_theme(theme);
        TextInputStyle {
            fill: Stateful::new(Fill::Solid(theme.surface_subtle))
                .hovered(Fill::Solid(theme.hovered))
                .focused(Fill::Solid(theme.surface)),
            border_color: Stateful::new(Color::TRANSPARENT).focused(theme.focus_border),
            shadow: Stateful::new(None)
                .focused(focus_ring(Some(theme.shadow_sm()[0]), theme.focus_border)),
            ..base
        }
    }
}

impl Default for TextInputStyle {
    fn default() -> Self {
        TextInputStyle::from_theme(&Theme::default())
    }
}

pub struct TextInput {
    id: String,
    focus_id: FocusId,
    width: f32,
    height: Option<f32>,
    style: Option<TextInputStyle>,
    text_style: Option<TextStyle>,
    default_text: String,
    placeholder: Option<String>,
    custom_padding: Option<Stateful<[f32; 2]>>,
    overrides: StyleOverrides,
}

impl TextInput {
    pub const DEFAULT_WIDTH: f32 = 200.0;

    pub fn new() -> Self {
        let id = generate_uuid();
        TextInput {
            focus_id: FocusId::new(&id),
            id,
            width: Self::DEFAULT_WIDTH,
            height: None,
            style: None,
            text_style: None,
            default_text: String::new(),
            placeholder: None,
            custom_padding: None,
            overrides: StyleOverrides::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        self.id = id.clone();
        self.focus_id = FocusId::new(&id);
        self
    }

    pub fn padding(mut self, padding: impl Into<Stateful<[f32; 2]>>) -> Self {
        self.custom_padding = Some(padding.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = size[0];
        self.height = Some(size[1]);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn style(mut self, style: TextInputStyle) -> Self {
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

    pub fn set_style(&mut self, style: Option<TextInputStyle>) {
        self.style = style;
    }

    pub fn default_text(mut self, text: impl Into<String>) -> Self {
        self.default_text = text.into();
        self
    }

    pub fn focused(&self, ui: &Ui) -> bool {
        ui.is_focused(self.focus_id)
    }

    pub fn hovered(&self, ui: &mut Ui) -> bool {
        let state = ui.widget_state::<TextEditState>(&self.id);
        state.hovered
    }

    fn resolved_style(&self, theme: &Theme) -> TextInputStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| TextInputStyle::from_theme(theme));
        if let Some(p) = &self.custom_padding {
            style.padding = p.clone();
        }
        if let Some(custom) = self.text_style {
            style.text_style = style.text_style.map(|t| custom.inherit(*t));
        }
        self.overrides.apply(
            &mut style.fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
    }

    pub fn text(&self, ui: &mut Ui) -> String {
        self.state(ui).text().to_string()
    }
    pub fn set_text(&mut self, ui: &mut Ui, text: String) {
        let mut state = self.take_state(ui);
        state.set_text(text);
        self.put_state(ui, state);
    }
}

style_overrides!(TextInput);

impl Widget for TextInput {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for TextInput {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let state = ui.widget_state_or(&self.id, self.initial_state());
        let weights = StateWeights::IDLE
            .hovered(state.hover_t)
            .focused(state.focus_t);
        let line_height = style.text_style.resolve(weights).line_height;
        let natural_height = line_height + style.padding.resolve(weights)[1] * 2.0;
        [self.width, self.height.unwrap_or(natural_height)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        self.check_id();
        ui.register_focusable(self.focus_id);

        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let state = ui.widget_state_or(&self.id, self.initial_state());
        let previous = StateWeights::IDLE
            .hovered(state.hover_t)
            .focused(state.focus_t);
        let padding = style.padding.resolve(previous)[0];
        let text_style = style.text_style.resolve(previous);
        let line_height = text_style.line_height;

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let mouse_pos = ui.mouse_position();
        let hovered = !ui.is_input_blocked(mouse_pos)
            && (style.path.resolve(previous))(position, size).contains(mouse_pos);
        let focused = self.focused(ui);

        let text_position = [
            position[0] + padding,
            position[1] + (size[1] - line_height) / 2.0,
        ];

        let mut state = ui.take_widget_state_or(&self.id, self.initial_state());
        state.hovered = hovered;

        if hovered {
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .focused(StateWeights::when(focused)),
                ),
            );
        }

        if ui.mouse_pressed_this_frame() && hovered {
            ui.request_focus(self.focus_id);
            state.dragging = true;
            state.mark_activity();
            let click_x = ui.mouse_position()[0] - text_position[0] + state.scroll_offset();
            let index = state.cursor_index_for_x(ui, text_style, click_x);
            match ui.click_count() {
                1 => {
                    state.set_cursor(index);
                    state.set_selection_anchor(Some(index));
                }
                2 => {
                    let start = state.word_start(index);
                    let end = state.word_end(index);
                    state.set_selection_anchor(Some(start));
                    state.set_cursor(end);
                }
                _ => {
                    let char_count = state.text().chars().count();
                    state.set_selection_anchor(Some(0));
                    state.set_cursor(char_count);
                }
            }
        }

        if !ui.mouse_pressed() {
            state.dragging = false;
        }

        if ui.mouse_pressed() && state.dragging && ui.click_count() == 1 {
            state.mark_activity();
            let drag_x = ui.mouse_position()[0] - text_position[0] + state.scroll_offset();
            let index = state.cursor_index_for_x(ui, text_style, drag_x);
            state.set_cursor(index);
        }

        if focused {
            state.handle_input(ui);
        }

        let text_width = size[0] - padding * 2.0;
        let cursor_x = ui.measure_text(state.prefix(), text_style);
        state.scroll_into_view(cursor_x, text_width);

        let focus_target = if focused { 1.0f32 } else { 0.0 };
        let hover_target = if hovered { 1.0f32 } else { 0.0 };
        state.focus_t = animate_towards(state.focus_t, focus_target, dt, style.focus_motion);
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);
        let focus_t = state.focus_t;
        let hover_t = state.hover_t;

        let weights = StateWeights::IDLE.hovered(hover_t).focused(focus_t);
        let path = style.path.resolve(weights);

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        // clipped to the padded inner bounds or scrolled selection paints past the edges
        let clip_rect = [
            position[0] + padding,
            position[1],
            position[0] + size[0] - padding,
            position[1] + size[1],
        ];
        ui.push_clip(clip_rect);

        if let Some((start, end)) = state.selection_range() {
            let text = state.text();
            let prefix_start = ui.measure_text(&text[..state.byte_index_for(start)], text_style);
            let prefix_end = ui.measure_text(&text[..state.byte_index_for(end)], text_style);

            let highlight_position = [
                (text_position[0] + prefix_start - state.scroll_offset()).round(),
                text_position[1],
            ];
            let highlight_size = [prefix_end - prefix_start, line_height];

            ui.draw_shape(
                MeasurablePath::rect(
                    highlight_position,
                    highlight_size,
                    [style.selection_radius.resolve(weights); 4],
                ),
                Fill::Solid(style.selection_color.resolve(weights)),
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        if state.text().is_empty() {
            if let Some(placeholder) = &self.placeholder {
                ui.draw_text(
                    placeholder,
                    text_style.color(style.placeholder_color.resolve(weights)),
                    text_position,
                    clip_rect,
                );
            }
        } else {
            ui.draw_text(
                state.text(),
                text_style,
                [text_position[0] - state.scroll_offset(), text_position[1]],
                clip_rect,
            );
        }

        if focused {
            let idle_seconds = state.last_activity().elapsed().as_secs_f32();
            if caret_visible(idle_seconds, style.cursor_blink_period) {
                let cursor_position = [
                    (text_position[0] + cursor_x - state.scroll_offset()).round(),
                    text_position[1],
                ];
                ui.draw_shape(
                    MeasurablePath::rect(
                        cursor_position,
                        [style.cursor_width.resolve(weights), line_height],
                        [0.0; 4],
                    ),
                    Fill::Solid(style.cursor_color.resolve(weights)),
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    true,
                    0.0,
                );
            }
        }

        ui.pop_clip();

        ui.put_widget_state(&self.id, state);
    }
}

impl StatefulWidget for TextInput {
    type State = TextEditState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> TextEditState {
        let mut state = TextEditState::default();
        state.set_text(&self.default_text);
        state
    }
}

impl Accessible for TextInput {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::TextInput
    }
    fn accessibility_label(&self) -> Option<String> {
        self.placeholder.clone()
    }
}
