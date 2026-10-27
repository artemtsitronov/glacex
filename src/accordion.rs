use std::collections::HashMap;

use crate::animation::{Motion, Spring, animate_towards};
use crate::border::Border;
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::{MeasurablePath, Path};
use crate::icons::{ChevronStyle, draw_chevron};
use crate::stateful::{StateWeights, Stateful};
use crate::text::{FontWeight, TextStyle};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{
    Accessible, FocusId, Measurable, StatefulWidget, Widget, hash_id, is_anonymous,
};
use crate::{AnyWidget, generate_uuid};
use accesskit::{Node, NodeId, Role};
use winit::keyboard::{Key, NamedKey};
use winit::window::CursorIcon;

#[derive(Clone)]
pub struct AccordionStyle {
    pub fill: Stateful<Fill>,
    pub border: Stateful<Border>,
    pub path: Stateful<Path>,
    pub header_height: f32,
    pub header_padding_x: f32,
    pub header_fill: Stateful<Fill>,
    pub header_border: Stateful<Border>,
    pub header_path: Stateful<Path>,
    pub header_text_style: Stateful<TextStyle>,
    pub chevron: ChevronStyle,
    pub content_padding: [f32; 2],
    pub show_dividers: bool,
    pub divider_color: Color,
    pub divider_width: f32,
    pub sharp: bool,
    pub hover_motion: f32,
    pub focus_motion: f32,
    pub open_stiffness: f32,
    pub open_damping: f32,
    pub cursor: Stateful<CursorIcon>,
}

impl AccordionStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        AccordionStyle {
            fill: Stateful::new(Fill::Solid(theme.surface)),
            border: Stateful::new(Border::new(1.0, theme.border)),
            path: Stateful::new(Path::rect([theme.metrics.radius_sm; 4])),
            header_height: 44.0,
            header_padding_x: 12.0,
            header_fill: Stateful::new(Fill::Solid(Color::TRANSPARENT))
                .hovered(Fill::Solid(theme.surface_subtle)),
            header_border: Stateful::new(Border::new(1.0, Color::TRANSPARENT)),
            header_path: Stateful::new(Path::rect([theme.metrics.radius_sm; 4])),
            header_text_style: Stateful::new(
                TextStyle::new()
                    .size(14.0, 20.0)
                    .weight(FontWeight::Medium)
                    .color(theme.text_primary),
            ),
            chevron: ChevronStyle::default(),
            content_padding: [12.0, 8.0],
            show_dividers: true,
            divider_color: theme.border,
            divider_width: 1.0,
            sharp: false,
            hover_motion: Motion::SNAPPY,
            focus_motion: Motion::GENTLE,
            open_stiffness: 380.0,
            open_damping: 30.0,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn filled(theme: &Theme) -> Self {
        AccordionStyle {
            header_fill: Stateful::new(Fill::Solid(theme.surface_subtle))
                .hovered(Fill::Solid(theme.hovered)),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for AccordionStyle {
    fn default() -> Self {
        AccordionStyle::from_theme(&Theme::default())
    }
}

pub struct AccordionItem<'a> {
    pub id: String,
    pub title: String,
    pub content: Box<dyn AnyWidget + 'a>,
    pub default_open: bool,
}

impl<'a> AccordionItem<'a> {
    pub fn new(title: impl Into<String>, content: &'a mut impl Measurable) -> Self {
        AccordionItem {
            id: generate_uuid(),
            title: title.into(),
            content: Box::new(content),
            default_open: false,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

#[derive(Debug, Clone)]
pub struct AccordionItemState {
    pub open: bool,
    pub open_spring: Spring,
    pub hover_t: f32,
    pub focus_t: f32,
    pub content_height: f32,
    pub chevron_angle: f32,
}

impl AccordionItemState {
    pub fn new(open: bool, style: &AccordionStyle) -> Self {
        AccordionItemState {
            open,
            open_spring: Spring::with_physics(
                if open { 1.0 } else { 0.0 },
                style.open_stiffness,
                style.open_damping,
            ),
            hover_t: 0.0,
            focus_t: 0.0,
            content_height: 0.0,
            chevron_angle: if open {
                style
                    .chevron
                    .open_angle
                    .resolve(StateWeights::IDLE.selected(1.0))
            } else {
                0.0
            },
        }
    }

    pub fn open_t(&self) -> f32 {
        self.open_spring.value.clamp(0.0, 1.0)
    }

    pub fn is_visible(&self) -> bool {
        self.open || self.open_t() > 0.001
    }
}

#[derive(Default)]
pub struct AccordionState {
    pub items: HashMap<String, AccordionItemState>,
    pub initialized: bool,
}

struct HeaderNode<'a> {
    id: &'a str,
    label: &'a str,
    expanded: bool,
}

impl Accessible for HeaderNode<'_> {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(self.id))
    }

    fn accessibility_role(&self) -> Role {
        Role::Button
    }

    fn accessibility_label(&self) -> Option<String> {
        Some(self.label.to_string())
    }

    fn accessibility_state(&self, node: &mut Node) {
        node.set_expanded(self.expanded);
    }
}

fn section_height(state: &AccordionItemState, style: &AccordionStyle) -> f32 {
    if state.content_height > 0.0 {
        (state.content_height + style.content_padding[1] * 2.0) * state.open_t()
    } else {
        0.0
    }
}

fn divider_height(style: &AccordionStyle) -> f32 {
    if style.show_dividers { 1.0 } else { 0.0 }
}

fn total_height(state: &AccordionState, keys: &[String], style: &AccordionStyle) -> f32 {
    let sections: f32 = keys
        .iter()
        .map(|key| style.header_height + section_height(&state.items[key], style))
        .sum();
    sections + divider_height(style) * keys.len().saturating_sub(1) as f32
}

fn apply_open(
    multiple: bool,
    state: &mut AccordionState,
    keys: &[String],
    index: usize,
    open: bool,
) {
    if open && !multiple {
        for item_state in state.items.values_mut() {
            item_state.open = false;
        }
    }
    if let Some(item_state) = keys.get(index).and_then(|key| state.items.get_mut(key)) {
        item_state.open = open;
    }
}

pub struct Accordion<'a> {
    id: String,
    items: Vec<AccordionItem<'a>>,
    style: Option<AccordionStyle>,
    text_style: Option<TextStyle>,
    width: f32,
    multiple: bool,
}

impl<'a> Accordion<'a> {
    pub fn new(items: Vec<AccordionItem<'a>>) -> Self {
        Accordion {
            id: generate_uuid(),
            items,
            style: None,
            text_style: None,
            width: 320.0,
            multiple: false,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub fn style(mut self, style: AccordionStyle) -> Self {
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

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange(position, size, ui);
    }

    pub fn is_open(&self, ui: &mut Ui, index: usize) -> bool {
        let Some(key) = self.keys().get(index).cloned() else {
            return false;
        };
        self.state(ui)
            .items
            .get(&key)
            .map(|item_state| item_state.open)
            .unwrap_or(false)
    }

    pub fn set_open(&mut self, ui: &mut Ui, index: usize, open: bool) {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let keys = self.keys();
        let mut state = self.take_state(ui);
        self.sync(&mut state, &keys, &style);
        apply_open(self.multiple, &mut state, &keys, index, open);
        self.put_state(ui, state);
    }

    fn resolved_style(&self, theme: &Theme) -> AccordionStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| AccordionStyle::from_theme(theme));
        if let Some(custom) = self.text_style {
            style.header_text_style = style.header_text_style.map(|t| custom.inherit(*t));
        }
        style
    }

    fn keys(&self) -> Vec<String> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if is_anonymous(&item.id) {
                    format!("#{i}")
                } else {
                    item.id.clone()
                }
            })
            .collect()
    }

    fn header_ids(&self, keys: &[String]) -> Vec<String> {
        keys.iter()
            .map(|key| format!("{}::{key}", self.id))
            .collect()
    }

    fn sync(&self, state: &mut AccordionState, keys: &[String], style: &AccordionStyle) {
        for (item, key) in self.items.iter().zip(keys) {
            if !state.items.contains_key(key) {
                let open = !state.initialized
                    && item.default_open
                    && (self.multiple || !state.items.values().any(|s| s.open));
                state
                    .items
                    .insert(key.clone(), AccordionItemState::new(open, style));
            }
        }
        state.initialized = true;
        state.items.retain(|key, _| keys.contains(key));
    }
}

impl<'a> StatefulWidget for Accordion<'a> {
    type State = AccordionState;

    fn state_id(&self) -> &str {
        &self.id
    }
}

impl<'a> Widget for Accordion<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl<'a> Measurable for Accordion<'a> {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let keys = self.keys();
        let mut state = self.take_state(ui);
        self.sync(&mut state, &keys, &style);

        for (item, key) in self.items.iter_mut().zip(&keys) {
            let item_state = state.items.get_mut(key).unwrap();
            if item_state.is_visible() {
                item_state.content_height = item.content.measure(ui)[1];
            }
        }

        let height = total_height(&state, &keys, &style);
        self.put_state(ui, state);
        [self.width, height]
    }

    #[allow(clippy::too_many_lines)]
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        self.check_id();
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse = ui.mouse_position();
        let clicked = ui.mouse_pressed_this_frame();
        let can_hit = !ui.is_input_blocked(mouse) && ui.point_in_current_clip(mouse);

        let keys = self.keys();
        let header_ids = self.header_ids(&keys);
        let mut state = self.take_state(ui);
        self.sync(&mut state, &keys, &style);

        let focused_index = header_ids
            .iter()
            .position(|id| ui.is_focused(FocusId::new(id)));

        let (x, w) = (position[0], size[0]);
        let pad_x = style.header_padding_x;
        let [content_pad_x, content_pad_y] = style.content_padding;
        let mut y = position[1];
        let mut toggle: Option<usize> = None;
        let mut hovered_index: Option<usize> = None;

        let any_focus = state.items.values().map(|s| s.focus_t).fold(0.0, f32::max);
        let any_open = state.items.values().map(|s| s.open_t()).fold(0.0, f32::max);
        let outer_weights = StateWeights::IDLE.focused(any_focus).selected(any_open);

        let outer_path = style.path.resolve(outer_weights);
        let outer_mpath = outer_path(position, size);

        let outer_fill = style.fill.resolve(outer_weights);
        let outer_border = style.border.resolve(outer_weights);

        if !matches!(outer_fill, Fill::Solid(c) if c.a < 0.003) {
            ui.draw_shape(
                // background, bgbg
                outer_mpath.clone(),
                outer_fill,
                Border::NONE,
                0.0,
                style.sharp,
                0.0,
            );
        }

        for (i, item) in self.items.iter_mut().enumerate() {
            let focus_id = FocusId::new(&header_ids[i]);
            let item_state = state.items.get_mut(&keys[i]).unwrap();
            let weights = StateWeights::IDLE
                .hovered(item_state.hover_t)
                .focused(item_state.focus_t)
                .selected(item_state.open_t());

            let header_pos = [x, y];
            let header_size = [w, style.header_height];
            let path = style.header_path.resolve(weights);

            if can_hit && path(header_pos, header_size).contains(mouse) {
                hovered_index = Some(i);
                if clicked {
                    toggle = Some(i);
                    ui.request_focus(focus_id);
                }
                ui.set_cursor_icon(
                    style.cursor.resolve(
                        StateWeights::IDLE
                            .hovered(1.0)
                            .selected(StateWeights::when(item_state.open)),
                    ),
                );
            }

            ui.register_focusable(focus_id);
            ui.register_accessible(
                &HeaderNode {
                    id: &header_ids[i],
                    label: &item.title,
                    expanded: item_state.open,
                },
                [x, y, x + w, y + style.header_height],
            );

            let fill = style.header_fill.resolve(weights);
            let border = style.header_border.resolve(weights);
            if !matches!(fill, Fill::Solid(c) if c.a < 0.003) || border.color.a >= 0.003 {
                ui.draw_shape(
                    path(header_pos, header_size),
                    fill,
                    border, //temporary
                    0.0,
                    style.sharp,
                    0.0,
                );
            }

            let text_style = style.header_text_style.resolve(weights);
            let text_y = y + (style.header_height - text_style.line_height) / 2.0;
            ui.draw_text(
                &item.title,
                text_style,
                [x + pad_x, text_y],
                [
                    x + pad_x,
                    y,
                    x + w - pad_x - style.chevron.space.resolve(weights),
                    y + style.header_height,
                ],
            );

            let chevron = style.chevron.resolve(
                weights,
                text_style.color.unwrap_or(theme.text_primary),
                pad_x,
            );
            draw_chevron(
                ui,
                header_pos,
                header_size,
                item_state.chevron_angle,
                &chevron,
            );
            y += style.header_height;

            let visible = section_height(item_state, &style);
            if visible > 0.5 && item_state.is_visible() {
                ui.push_clip([x, y + 2.0, x + w, y + visible - 2.0]);
                item.content.arrange(
                    [x + content_pad_x, y + content_pad_y],
                    [
                        (w - content_pad_x * 2.0).max(0.0),
                        item_state.content_height,
                    ],
                    ui,
                );
                ui.pop_clip();
            }
            y += visible;

            if style.show_dividers && i + 1 < keys.len() {
                ui.draw_shape(
                    MeasurablePath::rect([x, y], [w, style.divider_width], [0.0; 4]),
                    Fill::Solid(style.divider_color),
                    Border::NONE,
                    0.0,
                    false,
                    0.0,
                );
                y += 1.0;
            }
        }

        if outer_border.color.a >= 0.003 && outer_border.width > 0.0 {
            ui.draw_shape(
                // border
                outer_mpath.clone(),
                Fill::Solid(Color::TRANSPARENT),
                outer_border,
                0.0,
                style.sharp,
                0.0,
            );
        }

        if let Some(fi) = focused_index {
            let count = keys.len();
            if ui.key_pressed(Key::Named(NamedKey::Enter))
                || ui.key_pressed(Key::Named(NamedKey::Space))
            {
                toggle = Some(fi);
            }
            let next = if ui.key_pressed(Key::Named(NamedKey::ArrowDown)) {
                Some((fi + 1) % count)
            } else if ui.key_pressed(Key::Named(NamedKey::ArrowUp)) {
                Some((fi + count - 1) % count)
            } else if ui.key_pressed(Key::Named(NamedKey::Home)) {
                Some(0)
            } else if ui.key_pressed(Key::Named(NamedKey::End)) {
                Some(count - 1)
            } else {
                None
            };
            if let Some(next) = next {
                ui.request_focus(FocusId::new(&header_ids[next]));
            }
        }

        if let Some(i) = toggle {
            let open = !state.items[&keys[i]].open;
            apply_open(self.multiple, &mut state, &keys, i, open);
        }

        for (i, key) in keys.iter().enumerate() {
            let focused = ui.is_focused(FocusId::new(&header_ids[i]));
            let item_state = state.items.get_mut(key).unwrap();

            item_state.open_spring.stiffness = style.open_stiffness;
            item_state.open_spring.damping = style.open_damping;
            item_state
                .open_spring
                .set_target(if item_state.open { 1.0 } else { 0.0 });
            item_state.open_spring.update(dt);

            let angle_target = if item_state.open {
                style.chevron.open_angle.resolve(
                    StateWeights::IDLE
                        .hovered(item_state.hover_t)
                        .focused(item_state.focus_t)
                        .selected(item_state.open_t()),
                )
            } else {
                0.0
            };
            item_state.chevron_angle = animate_towards(
                item_state.chevron_angle,
                angle_target,
                dt,
                style.chevron.motion,
            );
            item_state.hover_t = animate_towards(
                item_state.hover_t,
                StateWeights::when(hovered_index == Some(i)),
                dt,
                style.hover_motion,
            );
            item_state.focus_t = animate_towards(
                item_state.focus_t,
                StateWeights::when(focused),
                dt,
                style.focus_motion,
            );
        }

        self.put_state(ui, state);
    }
}
