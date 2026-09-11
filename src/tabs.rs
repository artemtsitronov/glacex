use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::contains;
use crate::painter::FontWeight;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Debug, Clone)]
pub struct TabItem {
    pub id: String,
    pub label: String,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        TabItem {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabsStyle {
    pub height: f32,
    pub tab_padding_x: f32,
    pub gap: f32,
    pub text_color: Color,
    pub hover_text_color: Color,
    pub active_fill: Fill,
    pub pill_corner_radius: f32,
    pub pill_inset_y: f32,
}

impl Default for TabsStyle {
    fn default() -> Self {
        TabsStyle {
            height: 40.0,
            tab_padding_x: 16.0,
            gap: 4.0,
            text_color: Theme::TEXT_SECONDARY,
            hover_text_color: Theme::TEXT_PRIMARY,
            active_fill: Fill::Solid(Theme::ACTIVE),
            pill_corner_radius: Theme::RADIUS_MD,
            pill_inset_y: 4.0,
        }
    }
}

impl TabsStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        TabsStyle {
            text_color: theme.text_secondary,
            hover_text_color: theme.text_primary,
            active_fill: Fill::Solid(theme.active),
            ..TabsStyle::default()
        }
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
    indicator_x: f32,
    indicator_w: f32,
    initialized: bool,
}

impl Default for TabsState {
    fn default() -> Self {
        TabsState {
            hover_ts: Vec::new(),
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
}

impl Tabs {
    pub fn new(id: impl Into<String>, items: Vec<TabItem>) -> Self {
        Tabs {
            id: id.into(),
            items,
            style: None,
        }
    }

    pub fn style(mut self, style: TabsStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn selected(&self, ui: &Ui) -> Option<String> {
        ui.selected_option(&self.id).map(|s| s.to_string())
    }

    pub fn set_selected(&mut self, ui: &mut Ui, id: impl Into<String>) {
        ui.select(&self.id, &id.into());
    }

    fn resolved_style(&self, theme: &Theme) -> TabsStyle {
        self.style
            .clone()
            .unwrap_or_else(|| TabsStyle::from_theme(theme))
    }

    fn layout_tabs(&self, ui: &mut Ui, style: &TabsStyle) -> Vec<(f32, f32)> {
        let mut x = 0.0;
        let mut out = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let text_w = ui.measure_text_styled(&item.label, 14.0, 20.0, FontWeight::Medium, false);
            let w = text_w + style.tab_padding_x * 2.0;
            out.push((x, w));
            x += w + style.gap;
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
        let width = tabs.last().map(|(x, w)| x + w).unwrap_or(0.0);
        [width, style.height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> TabsResponse {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed = ui.mouse_pressed_this_frame();
        let blocked = ui.is_input_blocked(mouse_pos);
        let in_clip = ui.point_in_current_clip(mouse_pos);

        let tabs = self.layout_tabs(ui, &style);

        if ui.selected_option(&self.id).is_none() {
            if let Some(first) = self.items.first() {
                ui.select(&self.id, &first.id);
            }
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
            let hit = !blocked && in_clip && contains(tab_pos, tab_size, 0.0, mouse_pos);
            if hit {
                hovered_index = Some(i);
                if mouse_pressed {
                    clicked_index = Some(i);
                }
            }

            let target = if hit { 1.0 } else { 0.0 };
            state.hover_ts[i] = animate_towards(state.hover_ts[i], target, dt, Motion::SNAPPY);
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
            state.indicator_x = animate_towards(state.indicator_x, target_x, dt, Motion::FLUID);
            state.indicator_w = animate_towards(state.indicator_w, target_w, dt, Motion::FLUID);
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

        ui.draw_rect(
            [
                position[0] + state.indicator_x,
                position[1] + style.pill_inset_y,
            ],
            [state.indicator_w, size[1] - style.pill_inset_y * 2.0],
            style.active_fill.clone(),
            style.pill_corner_radius,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );

        let active_text_color = contrasting_text_color(&style.active_fill);
        for (i, &(tab_x, tab_w)) in tabs.iter().enumerate() {
            let tab_pos = [position[0] + tab_x, position[1]];
            let is_active = i == active_idx;
            let hover_t = state.hover_ts[i];

            let color = if is_active {
                active_text_color
            } else {
                style.text_color.lerp(style.hover_text_color, hover_t)
            };

            let text_width =
                ui.measure_text_styled(&self.items[i].label, 14.0, 20.0, FontWeight::Medium, false);
            let text_x = tab_pos[0] + (tab_w - text_width) / 2.0;
            let text_y = tab_pos[1] + (size[1] - 20.0) / 2.0;
            let clip = [
                tab_pos[0],
                tab_pos[1],
                tab_pos[0] + tab_w,
                tab_pos[1] + size[1],
            ];

            ui.draw_text_styled(
                &self.items[i].label,
                [text_x, text_y],
                clip,
                color,
                14.0,
                20.0,
                if is_active {
                    FontWeight::SemiBold
                } else {
                    FontWeight::Medium
                },
                false,
            );
        }

        if hovered_index.is_some() {
            ui.set_cursor_icon(CursorIcon::Pointer);
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
