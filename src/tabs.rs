use crate::animation::{Motion, animate_towards};
use crate::border::Border;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful};
use crate::text::{FontWeight, TextStyle};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id, warn_if_anonymous};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Debug, Clone)]
pub struct TabItem {
    pub id: String,
    pub label: String,
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        TabItem {
            id: generate_uuid(),
            label: label.into(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabsIndicator {
    Pill,
    Underline { thickness: f32 },
    None,
}

#[derive(Clone)]
pub struct TabsStyle {
    pub height: Stateful<f32>,
    pub tab_padding_x: Stateful<f32>,
    pub tab_min_width: Stateful<f32>,
    pub gap: Stateful<f32>,
    pub padding_x: Stateful<f32>,
    pub fill: Stateful<Fill>,
    pub border: Stateful<Border>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub text_style: Stateful<TextStyle>,
    pub tab_fill: Stateful<Fill>,
    pub indicator_fill: Stateful<Fill>,
    pub indicator: TabsIndicator,
    pub indicator_border: Stateful<Border>,
    pub indicator_shadow: Stateful<Option<ShadowStyle>>,
    pub pill_inset_y: Stateful<f32>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub indicator_path: Stateful<Path>,
    pub tab_path: Stateful<Path>,
    pub hover_motion: f32,
    pub indicator_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

fn tab_text_style(theme: &Theme, active_color: Option<Color>) -> Stateful<TextStyle> {
    let base = TextStyle::new()
        .size(14.0, 20.0)
        .weight(FontWeight::Medium)
        .color(theme.text_secondary);
    Stateful::new(base)
        .hovered(base.color(theme.text_primary))
        .selected(TextStyle {
            color: active_color,
            ..base.weight(FontWeight::SemiBold)
        })
}

fn tab_fill(hovered: Color) -> Stateful<Fill> {
    Stateful::new(Fill::Solid(Color::TRANSPARENT))
        .hovered(Fill::Solid(hovered))
        .selected(Fill::Solid(Color::TRANSPARENT))
}

impl TabsStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        TabsStyle {
            height: Stateful::new(40.0),
            tab_padding_x: Stateful::new(16.0),
            tab_min_width: Stateful::new(0.0),
            gap: Stateful::new(4.0),
            padding_x: Stateful::new(0.0),
            fill: Stateful::new(Fill::Solid(Color::TRANSPARENT)),
            border: Stateful::new(Border::NONE),
            shadow: Stateful::new(None),
            text_style: tab_text_style(theme, None),
            tab_fill: tab_fill(Color::TRANSPARENT),
            indicator_fill: Stateful::new(Fill::Solid(theme.active)),
            indicator: TabsIndicator::Pill,
            indicator_border: Stateful::new(Border::NONE),
            indicator_shadow: Stateful::new(None),
            pill_inset_y: Stateful::new(4.0),
            sharp: false,
            path: Stateful::new(Path::rect([theme.metrics.radius_md; 4])),
            indicator_path: Stateful::new(Path::rect([theme.metrics.radius_md; 4])),
            tab_path: Stateful::new(Path::rect([theme.metrics.radius_md; 4])),
            hover_motion: Motion::SNAPPY,
            indicator_motion: Motion::FLUID,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn segmented(theme: &Theme) -> Self {
        TabsStyle {
            gap: Stateful::new(2.0),
            padding_x: Stateful::new(4.0),
            fill: Stateful::new(Fill::Solid(theme.surface_subtle)),
            border: Stateful::new(Border::new(1.0, theme.border_faint)),
            indicator_fill: Stateful::new(Fill::Solid(theme.surface_elevated)),
            text_style: tab_text_style(theme, Some(theme.text_primary)),
            indicator_border: Stateful::new(Border::new(1.0, theme.border)),
            indicator_shadow: Stateful::new(Some(theme.shadow_sm()[0])),
            path: Stateful::new(Path::rect([theme.metrics.radius_lg; 4])),
            ..Self::from_theme(theme)
        }
    }

    pub fn underline(theme: &Theme) -> Self {
        TabsStyle {
            gap: Stateful::new(0.0),
            indicator: TabsIndicator::Underline { thickness: 2.0 },
            text_style: tab_text_style(theme, Some(theme.text_primary)),
            tab_fill: tab_fill(theme.surface_subtle),
            indicator_path: Stateful::new(Path::rect([1.0; 4])),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for TabsStyle {
    fn default() -> Self {
        TabsStyle::from_theme(&Theme::default())
    }
}

fn contrasting_text_color(bg: &Fill) -> Color {
    let sample = match bg {
        Fill::Solid(color) => *color,
        Fill::Gradient(gradient) => gradient
            .stops
            .first()
            .map(|stop| stop.color)
            .unwrap_or(Color::WHITE),
        Fill::Image(_) => Color::WHITE,
    };
    let luma = sample.r * 0.299 + sample.g * 0.587 + sample.b * 0.114;
    if luma > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

#[derive(Clone)]
struct TabsState {
    hover_ts: Vec<f32>,
    strip_hover_t: f32,
    indicator_x: f32,
    indicator_w: f32,
    initialized: bool,
}

impl Default for TabsState {
    fn default() -> Self {
        TabsState {
            hover_ts: Vec::new(),
            strip_hover_t: 0.0,
            indicator_x: 0.0,
            indicator_w: 0.0,
            initialized: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabsResponse {
    pub selected: String,
    pub changed: bool,
    pub hovered_index: Option<usize>,
}

pub struct Tabs {
    id: String,
    items: Vec<TabItem>,
    style: Option<TabsStyle>,
    text_style: Option<TextStyle>,
}

impl Tabs {
    pub fn new(items: Vec<TabItem>) -> Self {
        Tabs {
            id: generate_uuid(),
            items,
            style: None,
            text_style: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style(mut self, style: TabsStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<TabsStyle>) {
        self.style = style;
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn set_text_style(&mut self, text_style: Option<TextStyle>) {
        self.text_style = text_style;
    }

    pub fn selected(&self, ui: &Ui) -> Option<String> {
        ui.selected_option(&self.id).map(|s| s.to_string())
    }

    pub fn set_selected(&mut self, ui: &mut Ui, id: impl Into<String>) {
        ui.select(&self.id, &id.into());
    }

    fn resolved_style(&self, theme: &Theme) -> TabsStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| TabsStyle::from_theme(theme));
        if let Some(custom) = self.text_style {
            style.text_style = style.text_style.map(|t| custom.inherit(*t));
        }
        style
    }

    fn active_index(&self, ui: &Ui) -> usize {
        ui.selected_option(&self.id)
            .and_then(|selected| self.items.iter().position(|it| it.id == selected))
            .unwrap_or(0)
    }

    fn tab_weights(hover_ts: &[f32], active_idx: usize, i: usize) -> StateWeights {
        StateWeights::IDLE
            .hovered(hover_ts.get(i).copied().unwrap_or(0.0))
            .selected(StateWeights::when(i == active_idx))
    }

    fn layout_tabs(&self, ui: &mut Ui, style: &TabsStyle) -> Vec<(f32, f32)> {
        let previous = ui.widget_state::<TabsState>(&self.id).clone();
        let active_idx = self.active_index(ui);
        let strip = StateWeights::IDLE.hovered(previous.strip_hover_t);
        let gap = style.gap.resolve(strip);
        let mut x = style.padding_x.resolve(strip);
        let mut out = Vec::with_capacity(self.items.len());
        for (i, item) in self.items.iter().enumerate() {
            let weights = Self::tab_weights(&previous.hover_ts, active_idx, i);
            let text_w = ui.measure_text(&item.label, style.text_style.resolve(weights));
            let w = (text_w + style.tab_padding_x.resolve(weights) * 2.0)
                .max(style.tab_min_width.resolve(weights));
            out.push((x, w));
            x += w + gap;
        }
        out
    }
}

impl Widget for Tabs {
    type Output = TabsResponse;

    fn ui(&mut self, ui: &mut Ui) -> TabsResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Tabs {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let tabs = self.layout_tabs(ui, &style);
        let strip =
            StateWeights::IDLE.hovered(ui.widget_state::<TabsState>(&self.id).strip_hover_t);
        let width = tabs.last().map(|(x, w)| x + w).unwrap_or(0.0) + style.padding_x.resolve(strip);
        [width, style.height.resolve(strip)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> TabsResponse {
        warn_if_anonymous("Tabs", &self.id);
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed = ui.mouse_pressed_this_frame();
        let blocked = ui.is_input_blocked(mouse_pos);
        let in_clip = ui.point_in_current_clip(mouse_pos);

        let tabs = self.layout_tabs(ui, &style);

        if ui.selected_option(&self.id).is_none()
            && let Some(first) = self.items.first()
        {
            ui.select(&self.id, &first.id);
        }
        let previous_selected = ui.selected_option(&self.id).map(|s| s.to_string());

        let mut state = ui.take_widget_state::<TabsState>(&self.id);
        while state.hover_ts.len() < self.items.len() {
            state.hover_ts.push(0.0);
        }

        let mut hovered_index = None;
        let mut clicked_index = None;
        for (i, &(tab_x, tab_w)) in tabs.iter().enumerate() {
            let tab_pos = [position[0] + tab_x, position[1]];
            let tab_size = [tab_w, size[1]];
            let hit = !blocked
                && in_clip
                && MeasurablePath::rect(tab_pos, tab_size, [0.0; 4]).contains(mouse_pos);
            if hit {
                hovered_index = Some(i);
                if mouse_pressed {
                    clicked_index = Some(i);
                }
            }

            let target = if hit { 1.0 } else { 0.0 };
            state.hover_ts[i] = animate_towards(state.hover_ts[i], target, dt, style.hover_motion);
        }

        if let Some(i) = clicked_index {
            ui.select(&self.id, &self.items[i].id);
        }

        let selected_id = ui
            .selected_option(&self.id)
            .map(|s| s.to_string())
            .unwrap_or_default();
        let changed = previous_selected.as_deref() != Some(selected_id.as_str());

        let active_idx = self
            .items
            .iter()
            .position(|it| it.id == selected_id)
            .unwrap_or(0);
        let (target_x, target_w) = tabs.get(active_idx).copied().unwrap_or((0.0, 0.0));

        if !state.initialized {
            state.indicator_x = target_x;
            state.indicator_w = target_w;
            state.initialized = true;
        } else {
            state.indicator_x =
                animate_towards(state.indicator_x, target_x, dt, style.indicator_motion);
            state.indicator_w =
                animate_towards(state.indicator_w, target_w, dt, style.indicator_motion);
        }

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let strip_path = style
            .path
            .resolve(StateWeights::IDLE.hovered(state.strip_hover_t));
        let strip_hovered = !blocked && in_clip && strip_path(position, size).contains(mouse_pos);
        state.strip_hover_t = animate_towards(
            state.strip_hover_t,
            if strip_hovered { 1.0 } else { 0.0 },
            dt,
            style.hover_motion,
        );
        let strip_weights = StateWeights::IDLE.hovered(state.strip_hover_t);
        let path = style.path.resolve(strip_weights);

        if let Some(shadow) = &style.shadow.resolve(strip_weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.fill.resolve(strip_weights),
            style.border.resolve(strip_weights),
            0.0,
            style.sharp,
            0.0,
        );

        let tab_weights = |i: usize, hover_ts: &[f32]| Self::tab_weights(hover_ts, active_idx, i);

        for (i, &(tab_x, tab_w)) in tabs.iter().enumerate() {
            let weights = tab_weights(i, &state.hover_ts);
            let fill = style.tab_fill.resolve(weights);
            if matches!(fill, Fill::Solid(c) if c.a < 0.003) {
                continue;
            }
            let inset = style.pill_inset_y.resolve(weights);
            ui.draw_shape(
                (style.tab_path.resolve(weights))(
                    [position[0] + tab_x, position[1] + inset],
                    [tab_w, size[1] - inset * 2.0],
                ),
                fill,
                Border::NONE,
                0.0,
                style.sharp,
                0.0,
            );
        }

        let indicator_weights =
            StateWeights::IDLE.hovered(state.hover_ts.get(active_idx).copied().unwrap_or(0.0));
        let indicator_inset = style.pill_inset_y.resolve(indicator_weights);
        let indicator_rect = match style.indicator {
            TabsIndicator::Pill => Some((
                [
                    position[0] + state.indicator_x,
                    position[1] + indicator_inset,
                ],
                [state.indicator_w, size[1] - indicator_inset * 2.0],
            )),
            TabsIndicator::Underline { thickness } => Some((
                [
                    position[0] + state.indicator_x,
                    position[1] + size[1] - thickness,
                ],
                [state.indicator_w, thickness],
            )),
            TabsIndicator::None => None,
        };
        let indicator_fill = style.indicator_fill.resolve(indicator_weights);
        if let Some((indicator_pos, indicator_size)) = indicator_rect {
            let indicator_path = style.indicator_path.resolve(indicator_weights);
            if let Some(shadow) = &style.indicator_shadow.resolve(indicator_weights) {
                draw_shadow(shadow, indicator_pos, indicator_size, &indicator_path, ui);
            }
            ui.draw_shape(
                indicator_path(indicator_pos, indicator_size),
                indicator_fill.clone(),
                style.indicator_border.resolve(indicator_weights),
                0.0,
                style.sharp,
                0.0,
            );
        }

        let auto_active_color = match style.indicator {
            TabsIndicator::Pill => contrasting_text_color(&indicator_fill),
            _ => theme.text_primary,
        };
        for (i, &(tab_x, tab_w)) in tabs.iter().enumerate() {
            let tab_pos = [position[0] + tab_x, position[1]];
            let text_style = style
                .text_style
                .resolve(tab_weights(i, &state.hover_ts))
                .or_color(if i == active_idx {
                    auto_active_color
                } else {
                    theme.text_secondary
                });
            let text_width = ui.measure_text(&self.items[i].label, text_style);
            let text_x = tab_pos[0] + (tab_w - text_width) / 2.0;
            let text_y = tab_pos[1] + (size[1] - text_style.line_height) / 2.0;
            let clip = [
                tab_pos[0],
                tab_pos[1],
                tab_pos[0] + tab_w,
                tab_pos[1] + size[1],
            ];

            ui.draw_text(&self.items[i].label, text_style, [text_x, text_y], clip);
        }

        if let Some(i) = hovered_index {
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .pressed(StateWeights::when(ui.mouse_pressed()))
                        .selected(StateWeights::when(i == active_idx)),
                ),
            );
        }

        ui.put_widget_state(&self.id, state);

        TabsResponse {
            selected: selected_id,
            changed,
            hovered_index,
        }
    }
}

impl Accessible for Tabs {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::TabList
    }
    fn accessibility_label(&self) -> Option<String> {
        None
    }
}
