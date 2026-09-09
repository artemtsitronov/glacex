//! Small composable display primitives used by the 0.2 design system.
//!
//! These deliberately stay content-only and immediate-mode friendly. They can be
//! composed with `row!`, `column!`, `Card`, and the existing interactive widgets.

use crate::color::Color;
use crate::fill::Fill;
use crate::painter::FontWeight;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};
use winit::keyboard::{Key, NamedKey};

pub struct Separator {
    vertical: bool,
    length: f32,
    thickness: f32,
    color: Option<Color>,
}

impl Separator {
    pub fn horizontal(length: f32) -> Self {
        Self {
            vertical: false,
            length,
            thickness: 1.0,
            color: None,
        }
    }
    pub fn vertical(length: f32) -> Self {
        Self {
            vertical: true,
            length,
            thickness: 1.0,
            color: None,
        }
    }
    pub fn thickness(mut self, value: f32) -> Self {
        self.thickness = value;
        self
    }
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }
}

impl Widget for Separator {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}
impl Measurable for Separator {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        if self.vertical {
            [self.thickness, self.length]
        } else {
            [self.length, self.thickness]
        }
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.draw_rect(
            position,
            size,
            Fill::Solid(self.color.unwrap_or(ui.theme().border_faint)),
            0.0,
            0.0,
            Color::TRANSPARENT,
            0.0,
            true,
            0.0,
        );
    }
}

pub struct Skeleton {
    size: [f32; 2],
    radius: f32,
}
impl Skeleton {
    pub fn new(size: [f32; 2]) -> Self {
        Self {
            size,
            radius: Theme::RADIUS_SM,
        }
    }
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
impl Widget for Skeleton {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        self.arrange([0.0, 0.0], self.size, ui);
    }
}
impl Measurable for Skeleton {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        self.size
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let phase = (ui.time() * 1.4).sin() * 0.04 + 0.10;
        ui.draw_rect(
            position,
            size,
            Fill::Solid(ui.theme().text_primary.with_alpha(phase)),
            self.radius,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );
    }
}

pub struct Spinner {
    diameter: f32,
    color: Option<Color>,
}
impl Spinner {
    pub fn new(diameter: f32) -> Self {
        Self {
            diameter,
            color: None,
        }
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}
impl Widget for Spinner {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        self.arrange([0.0, 0.0], [self.diameter, self.diameter], ui);
    }
}
impl Measurable for Spinner {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.diameter, self.diameter]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let angle = ui.time() * 5.0;
        let c = self.color.unwrap_or(ui.theme().text_secondary);
        ui.draw_rect(
            position,
            size,
            Fill::Solid(c.with_alpha(0.16)),
            Theme::RADIUS_FULL,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );
        ui.draw_rect(
            [position[0] + size[0] * 0.18, position[1] + size[1] * 0.18],
            [size[0] * 0.64, size[1] * 0.64],
            Fill::Solid(c.with_alpha(0.72)),
            Theme::RADIUS_FULL,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            angle,
        );
    }
}

pub struct Kbd {
    text: String,
    width: Option<f32>,
}
impl Kbd {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            width: None,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}
impl Widget for Kbd {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}
impl Measurable for Kbd {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        [
            self.width.unwrap_or(
                ui.measure_text_styled(&self.text, 12.0, 16.0, FontWeight::Medium, true) + 16.0,
            ),
            24.0,
        ]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.draw_rect(
            position,
            size,
            Fill::Solid(ui.theme().surface_subtle),
            Theme::RADIUS_SM,
            1.0,
            ui.theme().border,
            0.0,
            false,
            0.0,
        );
        ui.draw_text_styled(
            &self.text,
            [position[0] + 8.0, position[1] + 4.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            ui.theme().text_secondary,
            12.0,
            16.0,
            FontWeight::Medium,
            true,
        );
    }
}

pub struct Avatar {
    label: String,
    size: f32,
    color: Option<Color>,
}
impl Avatar {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            size: 32.0,
            color: None,
        }
    }
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}
impl Widget for Avatar {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        self.arrange([0.0, 0.0], [self.size, self.size], ui);
    }
}
impl Measurable for Avatar {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.size, self.size]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let fill = self.color.unwrap_or(ui.theme().active);
        ui.draw_rect(
            position,
            size,
            Fill::Solid(fill),
            Theme::RADIUS_FULL,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );
        ui.draw_text_styled(
            &self.label,
            position,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            Color::WHITE,
            13.0,
            18.0,
            FontWeight::SemiBold,
            false,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
}
pub struct Alert {
    text: String,
    variant: AlertVariant,
    width: f32,
}

pub struct Empty {
    title: String,
    description: Option<String>,
    width: f32,
}

impl Empty {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            width: 320.0,
        }
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Widget for Empty {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}
impl Measurable for Empty {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [
            self.width,
            if self.description.is_some() {
                84.0
            } else {
                56.0
            },
        ]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let title_width =
            ui.measure_text_styled(&self.title, 14.0, 20.0, FontWeight::SemiBold, false);
        ui.draw_text_styled(
            &self.title,
            [
                position[0] + (size[0] - title_width) * 0.5,
                position[1] + 16.0,
            ],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            ui.theme().text_primary,
            14.0,
            20.0,
            FontWeight::SemiBold,
            false,
        );
        if let Some(description) = &self.description {
            ui.draw_text_colored(
                description,
                [position[0] + 12.0, position[1] + 42.0],
                [
                    position[0],
                    position[1],
                    position[0] + size[0],
                    position[1] + size[1],
                ],
                ui.theme().text_secondary,
            );
        }
    }
}

pub struct Toggle {
    id: String,
    label: String,
    width: f32,
}
impl Toggle {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            width: 180.0,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn enabled(&self, ui: &mut Ui) -> bool {
        ui.widget_state::<ToggleState>(&self.id).enabled
    }
}
#[derive(Default)]
pub struct ToggleState {
    pub enabled: bool,
}
impl Widget for Toggle {
    type Output = bool;
    fn ui(&mut self, ui: &mut Ui) -> bool {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
        self.enabled(ui)
    }
}
impl Measurable for Toggle {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, 32.0]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> bool {
        let hovered =
            crate::geometry::contains(position, size, Theme::RADIUS_MD, ui.mouse_position());
        let pressed = ui.mouse_pressed_this_frame();
        let mut enabled = ui.widget_state::<ToggleState>(&self.id).enabled;
        if hovered && pressed {
            enabled = !enabled;
            ui.widget_state::<ToggleState>(&self.id).enabled = enabled;
        }
        let theme = *ui.theme();
        let fill = if enabled {
            theme.active
        } else {
            theme.surface_subtle
        };
        ui.draw_rect(
            position,
            size,
            Fill::Solid(fill),
            Theme::RADIUS_MD,
            1.0,
            theme.border,
            0.0,
            false,
            0.0,
        );
        ui.draw_text_colored(
            &self.label,
            [position[0] + 12.0, position[1] + 7.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            if enabled {
                Color::WHITE
            } else {
                theme.text_primary
            },
        );
        enabled
    }
}
impl Alert {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: AlertVariant::Info,
            width: 320.0,
        }
    }
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
impl Widget for Alert {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}
impl Measurable for Alert {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, 44.0]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let color = match self.variant {
            AlertVariant::Info => ui.theme().active,
            AlertVariant::Success => ui.theme().success,
            AlertVariant::Warning => ui.theme().warning,
            AlertVariant::Error => ui.theme().error,
        };
        ui.draw_rect(
            position,
            size,
            Fill::Solid(color.with_alpha(0.10)),
            Theme::RADIUS_MD,
            1.0,
            color.with_alpha(0.25),
            0.0,
            false,
            0.0,
        );
        ui.draw_text_colored(
            &self.text,
            [position[0] + 12.0, position[1] + 12.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            ui.theme().text_primary,
        );
    }
}

/// A compact, keyboard-friendly tab strip. The selected index is persistent by id.
pub struct Tabs {
    id: String,
    labels: Vec<String>,
    width: f32,
}

#[derive(Default)]
pub struct TabsState {
    pub selected: usize,
}

impl Tabs {
    pub fn new(id: impl Into<String>, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            labels: labels.into_iter().map(Into::into).collect(),
            width: 360.0,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> usize {
        ui.widget_state::<TabsState>(&self.id).selected
    }
}

impl Widget for Tabs {
    type Output = usize;
    fn ui(&mut self, ui: &mut Ui) -> usize {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}
impl Measurable for Tabs {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, 36.0]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> usize {
        let count = self.labels.len().max(1);
        let item_width = size[0] / count as f32;
        let hovered = ui.mouse_position();
        let mut selected = ui
            .widget_state::<TabsState>(&self.id)
            .selected
            .min(count - 1);
        if ui.mouse_pressed_this_frame()
            && hovered[0] >= position[0]
            && hovered[0] <= position[0] + size[0]
            && hovered[1] >= position[1]
            && hovered[1] <= position[1] + size[1]
        {
            selected = ((hovered[0] - position[0]) / item_width).floor() as usize;
            ui.widget_state::<TabsState>(&self.id).selected = selected;
        }
        let theme = *ui.theme();
        ui.draw_rect(
            position,
            size,
            Fill::Solid(theme.surface_subtle),
            Theme::RADIUS_MD,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
        );
        for (index, label) in self.labels.iter().enumerate() {
            let x = position[0] + index as f32 * item_width;
            if index == selected {
                ui.draw_rect(
                    [x + 2.0, position[1] + 2.0],
                    [item_width - 4.0, size[1] - 4.0],
                    Fill::Solid(theme.surface_elevated),
                    Theme::RADIUS_SM,
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    false,
                    0.0,
                );
            }
            ui.draw_text_colored(
                label,
                [x + 12.0, position[1] + 9.0],
                [x, position[1], x + item_width, position[1] + size[1]],
                if index == selected {
                    theme.text_primary
                } else {
                    theme.text_secondary
                },
            );
        }
        selected
    }
}

/// Disclosure row with an animated-height-friendly state model.
pub struct Accordion {
    id: String,
    title: String,
    body: String,
    width: f32,
}
#[derive(Default)]
pub struct AccordionState {
    pub open: bool,
}
impl Accordion {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            width: 360.0,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn open(&self, ui: &mut Ui) -> bool {
        ui.widget_state::<AccordionState>(&self.id).open
    }
}
impl Widget for Accordion {
    type Output = bool;
    fn ui(&mut self, ui: &mut Ui) -> bool {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}
impl Measurable for Accordion {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        [self.width, if self.open(ui) { 76.0 } else { 40.0 }]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> bool {
        let point = ui.mouse_position();
        let hit = point[0] >= position[0]
            && point[0] <= position[0] + self.width
            && point[1] >= position[1]
            && point[1] <= position[1] + 40.0;
        let mut open = ui.widget_state::<AccordionState>(&self.id).open;
        if hit && ui.mouse_pressed_this_frame() {
            open = !open;
            ui.widget_state::<AccordionState>(&self.id).open = open;
        }
        let theme = *ui.theme();
        ui.draw_rect(
            position,
            [size[0], 40.0],
            Fill::Solid(theme.surface),
            Theme::RADIUS_MD,
            1.0,
            theme.border,
            0.0,
            false,
            0.0,
        );
        ui.draw_text_colored(
            &self.title,
            [position[0] + 12.0, position[1] + 10.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + 40.0,
            ],
            theme.text_primary,
        );
        ui.draw_text_colored(
            if open { "−" } else { "+" },
            [position[0] + size[0] - 26.0, position[1] + 10.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + 40.0,
            ],
            theme.text_secondary,
        );
        if open {
            ui.draw_text_colored(
                &self.body,
                [position[0] + 12.0, position[1] + 50.0],
                [
                    position[0],
                    position[1],
                    position[0] + size[0],
                    position[1] + size[1],
                ],
                theme.text_secondary,
            );
        }
        open
    }
}

/// Centered modal surface. The caller controls visibility through the persistent state.
pub struct Dialog {
    id: String,
    title: String,
    body: String,
    width: f32,
}
#[derive(Default)]
pub struct DialogState {
    pub open: bool,
}
impl Dialog {
    pub fn new(id: impl Into<String>, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            width: 420.0,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn open(&self, ui: &mut Ui) -> bool {
        ui.widget_state::<DialogState>(&self.id).open
    }
    pub fn set_open(&self, ui: &mut Ui, open: bool) {
        ui.widget_state::<DialogState>(&self.id).open = open;
    }
}
impl Widget for Dialog {
    type Output = bool;
    fn ui(&mut self, ui: &mut Ui) -> bool {
        let open = self.open(ui);
        if !open {
            return false;
        }
        let size = self.measure(ui);
        let window = ui.window_size();
        self.arrange(
            [(window[0] - size[0]) * 0.5, (window[1] - size[1]) * 0.5],
            size,
            ui,
        )
    }
}
impl Measurable for Dialog {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, 178.0]
    }
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> bool {
        if ui.key_pressed(Key::Named(NamedKey::Escape)) {
            self.set_open(ui, false);
            return false;
        }
        let window = ui.window_size();
        ui.push_input_block([0.0, 0.0, window[0], window[1]]);
        let theme = *ui.theme();
        ui.draw_overlay_rect(
            [0.0, 0.0],
            window,
            Fill::Solid(Color::BLACK.with_alpha(0.35)),
            0.0,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            [0.0, 0.0, window[0], window[1]],
            0.0,
        );
        ui.draw_overlay_rect(
            position,
            size,
            Fill::Solid(theme.surface_elevated),
            Theme::RADIUS_LG,
            1.0,
            theme.border,
            22.0,
            false,
            [0.0, 0.0, window[0], window[1]],
            0.0,
        );
        ui.draw_overlay_text_styled(
            &self.title,
            [position[0] + 24.0, position[1] + 22.0],
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
            theme.text_primary,
            18.0,
            24.0,
            FontWeight::SemiBold,
            false,
        );
        ui.draw_overlay_text_styled(
            &self.body,
            [position[0] + 24.0, position[1] + 58.0],
            [
                position[0] + 20.0,
                position[1] + 50.0,
                position[0] + size[0] - 20.0,
                position[1] + size[1] - 20.0,
            ],
            theme.text_secondary,
            14.0,
            20.0,
            FontWeight::Regular,
            false,
        );
        true
    }
}

pub struct Toast {
    id: String,
    text: String,
    duration: f32,
}
#[derive(Default)]
pub struct ToastState {
    pub visible_since: Option<f32>,
}
impl Toast {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            duration: 3.0,
        }
    }
    pub fn duration(mut self, seconds: f32) -> Self {
        self.duration = seconds.max(0.1);
        self
    }
    pub fn show(&self, ui: &mut Ui) {
        ui.widget_state::<ToastState>(&self.id).visible_since = Some(ui.time());
    }
}
impl Widget for Toast {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let started = ui.widget_state::<ToastState>(&self.id).visible_since;
        let Some(started) = started else {
            return;
        };
        if ui.time() - started > self.duration {
            ui.widget_state::<ToastState>(&self.id).visible_since = None;
            return;
        }
        let window = ui.window_size();
        let width = (ui.measure_text_styled(&self.text, 13.0, 18.0, FontWeight::Medium, false)
            + 28.0)
            .min(window[0] - 24.0);
        let pos = [(window[0] - width) * 0.5, window[1] - 76.0];
        let theme = *ui.theme();
        ui.draw_overlay_rect(
            pos,
            [width, 42.0],
            Fill::Solid(theme.surface_elevated),
            Theme::RADIUS_MD,
            1.0,
            theme.border,
            14.0,
            false,
            [0.0, 0.0, window[0], window[1]],
            0.0,
        );
        ui.draw_overlay_text_styled(
            &self.text,
            [pos[0] + 14.0, pos[1] + 12.0],
            [pos[0], pos[1], pos[0] + width, pos[1] + 42.0],
            theme.text_primary,
            13.0,
            18.0,
            FontWeight::Medium,
            false,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyVariant {
    Body,
    Caption,
    Label,
    Heading,
    Title,
    Code,
}

pub struct Typography {
    text: String,
    variant: TypographyVariant,
    color: Option<Color>,
    width: Option<f32>,
}
impl Typography {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: TypographyVariant::Body,
            color: None,
            width: None,
        }
    }
    pub fn variant(mut self, v: TypographyVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    fn metrics(&self) -> (f32, f32, FontWeight, bool) {
        match self.variant {
            TypographyVariant::Body => (14.0, 20.0, FontWeight::Regular, false),
            TypographyVariant::Caption => (12.0, 16.0, FontWeight::Regular, false),
            TypographyVariant::Label => (13.0, 18.0, FontWeight::Medium, false),
            TypographyVariant::Heading => (18.0, 24.0, FontWeight::SemiBold, false),
            TypographyVariant::Title => (24.0, 30.0, FontWeight::SemiBold, false),
            TypographyVariant::Code => (13.0, 18.0, FontWeight::Regular, true),
        }
    }
}
impl Widget for Typography {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui);
    }
}
impl Measurable for Typography {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let (f, l, w, m) = self.metrics();
        [
            self.width
                .unwrap_or(ui.measure_text_styled(&self.text, f, l, w, m)),
            l,
        ]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) {
        let (f, l, w, m) = self.metrics();
        ui.draw_text_styled(
            &self.text,
            p,
            [p[0], p[1], p[0] + s[0], p[1] + s[1]],
            self.color.unwrap_or(ui.theme().text_primary),
            f,
            l,
            w,
            m,
        );
    }
}

pub struct AspectRatio<W> {
    child: W,
    ratio: f32,
    width: f32,
}
impl<W> AspectRatio<W> {
    pub fn new(child: W, ratio: f32) -> Self {
        Self {
            child,
            ratio: ratio.max(0.01),
            width: 320.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}
impl<W: Measurable> Widget for AspectRatio<W> {
    type Output = W::Output;
    fn ui(&mut self, ui: &mut Ui) -> W::Output {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl<W: Measurable> Measurable for AspectRatio<W> {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, self.width / self.ratio]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> W::Output {
        self.child.arrange(p, s, ui)
    }
}

pub struct Field {
    label: String,
    description: Option<String>,
    error: Option<String>,
    width: f32,
}
impl Field {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            error: None,
            width: 280.0,
        }
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.error = Some(v.into());
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}
impl Widget for Field {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui);
    }
}
impl Measurable for Field {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [
            self.width,
            if self.error.is_some() || self.description.is_some() {
                50.0
            } else {
                22.0
            },
        ]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) {
        let t = *ui.theme();
        ui.draw_text_styled(
            &self.label,
            p,
            [p[0], p[1], p[0] + s[0], p[1] + 20.0],
            t.text_primary,
            13.0,
            18.0,
            FontWeight::Medium,
            false,
        );
        if let Some(x) = self.error.as_ref().or(self.description.as_ref()) {
            ui.draw_text_colored(
                x,
                [p[0], p[1] + 25.0],
                [p[0], p[1], p[0] + s[0], p[1] + s[1]],
                if self.error.is_some() {
                    t.error
                } else {
                    t.text_secondary
                },
            );
        }
    }
}

pub struct ButtonGroup {
    labels: Vec<String>,
    id: String,
    width: f32,
}
#[derive(Default)]
pub struct ButtonGroupState {
    pub selected: usize,
}
impl ButtonGroup {
    pub fn new(id: impl Into<String>, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            labels: labels.into_iter().map(Into::into).collect(),
            id: id.into(),
            width: 320.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> usize {
        ui.widget_state::<ButtonGroupState>(&self.id).selected
    }
}
impl Widget for ButtonGroup {
    type Output = usize;
    fn ui(&mut self, ui: &mut Ui) -> usize {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for ButtonGroup {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 34.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> usize {
        let n = self.labels.len().max(1);
        let w = s[0] / n as f32;
        let m = ui.mouse_position();
        let mut selected = ui
            .widget_state::<ButtonGroupState>(&self.id)
            .selected
            .min(n - 1);
        if ui.mouse_pressed_this_frame()
            && m[0] >= p[0]
            && m[0] <= p[0] + s[0]
            && m[1] >= p[1]
            && m[1] <= p[1] + s[1]
        {
            selected = ((m[0] - p[0]) / w) as usize;
            ui.widget_state::<ButtonGroupState>(&self.id).selected = selected;
        }
        let t = *ui.theme();
        for (i, label) in self.labels.iter().enumerate() {
            let x = p[0] + i as f32 * w;
            ui.draw_rect(
                [x, p[1]],
                [w, s[1]],
                Fill::Solid(if i == selected {
                    t.active
                } else {
                    t.surface_subtle
                }),
                Theme::RADIUS_SM,
                1.0,
                t.border,
                0.0,
                false,
                0.0,
            );
            ui.draw_text_colored(
                label,
                [x + 10.0, p[1] + 8.0],
                [x, p[1], x + w, p[1] + s[1]],
                if i == selected {
                    Color::WHITE
                } else {
                    t.text_primary
                },
            );
        }
        selected
    }
}

pub struct Breadcrumb {
    items: Vec<String>,
    width: f32,
}
impl Breadcrumb {
    pub fn new(items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            width: 360.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}
impl Widget for Breadcrumb {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui);
    }
}
impl Measurable for Breadcrumb {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 24.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) {
        let t = *ui.theme();
        let mut x = p[0];
        for (i, item) in self.items.iter().enumerate() {
            ui.draw_text_colored(
                item,
                [x, p[1] + 3.0],
                [p[0], p[1], p[0] + s[0], p[1] + s[1]],
                if i + 1 == self.items.len() {
                    t.text_primary
                } else {
                    t.text_secondary
                },
            );
            x += item.len() as f32 * 7.0 + 12.0;
            if i + 1 < self.items.len() {
                ui.draw_text_colored(
                    "/",
                    [x, p[1] + 3.0],
                    [p[0], p[1], p[0] + s[0], p[1] + s[1]],
                    t.text_muted,
                );
                x += 14.0;
            }
        }
    }
}

pub struct Pagination {
    id: String,
    pages: usize,
    width: f32,
}
#[derive(Default)]
pub struct PaginationState {
    pub page: usize,
}
impl Pagination {
    pub fn new(id: impl Into<String>, pages: usize) -> Self {
        Self {
            id: id.into(),
            pages: pages.max(1),
            width: 280.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn page(&self, ui: &mut Ui) -> usize {
        ui.widget_state::<PaginationState>(&self.id)
            .page
            .min(self.pages - 1)
    }
}
impl Widget for Pagination {
    type Output = usize;
    fn ui(&mut self, ui: &mut Ui) -> usize {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for Pagination {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 34.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> usize {
        let n = self.pages;
        let cell = s[0] / (n as f32 + 2.0);
        let mouse = ui.mouse_position();
        let mut page = self.page(ui);
        if ui.mouse_pressed_this_frame()
            && mouse[1] >= p[1]
            && mouse[1] <= p[1] + s[1]
            && mouse[0] >= p[0]
            && mouse[0] <= p[0] + s[0]
        {
            let index = ((mouse[0] - p[0]) / cell) as usize;
            if index == 0 {
                page = page.saturating_sub(1)
            } else if index > n {
                page = (page + 1).min(n - 1)
            } else {
                page = (index - 1).min(n - 1)
            }
            ui.widget_state::<PaginationState>(&self.id).page = page;
        }
        let t = *ui.theme();
        for i in 0..n + 2 {
            let x = p[0] + i as f32 * cell;
            let label = if i == 0 {
                "‹".to_string()
            } else if i == n + 1 {
                "›".to_string()
            } else {
                (i).to_string()
            };
            ui.draw_rect(
                [x, p[1]],
                [cell - 4.0, s[1]],
                Fill::Solid(if i > 0 && i < n + 1 && i - 1 == page {
                    t.active
                } else {
                    t.surface_subtle
                }),
                Theme::RADIUS_SM,
                1.0,
                t.border,
                0.0,
                false,
                0.0,
            );
            ui.draw_text_colored(
                &label,
                [x + cell * 0.5 - 4.0, p[1] + 8.0],
                [x, p[1], x + cell, p[1] + s[1]],
                if i > 0 && i < n + 1 && i - 1 == page {
                    Color::WHITE
                } else {
                    t.text_primary
                },
            );
        }
        page
    }
}

pub struct Table {
    id: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    width: f32,
}
#[derive(Default)]
pub struct TableState {
    pub selected: Option<usize>,
}
impl Table {
    pub fn new(
        id: impl Into<String>,
        columns: impl IntoIterator<Item = impl Into<String>>,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<String>>>,
    ) -> Self {
        Self {
            id: id.into(),
            columns: columns.into_iter().map(Into::into).collect(),
            rows: rows
                .into_iter()
                .map(|r| r.into_iter().map(Into::into).collect())
                .collect(),
            width: 520.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> Option<usize> {
        ui.widget_state::<TableState>(&self.id).selected
    }
}
impl Widget for Table {
    type Output = Option<usize>;
    fn ui(&mut self, ui: &mut Ui) -> Option<usize> {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for Table {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 36.0 + self.rows.len() as f32 * 32.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> Option<usize> {
        let t = *ui.theme();
        let cols = self.columns.len().max(1);
        let cw = s[0] / cols as f32;
        ui.draw_rect(
            p,
            [s[0], 36.0],
            Fill::Solid(t.surface_subtle),
            Theme::RADIUS_SM,
            1.0,
            t.border,
            0.0,
            false,
            0.0,
        );
        for (i, h) in self.columns.iter().enumerate() {
            ui.draw_text_colored(
                h,
                [p[0] + i as f32 * cw + 10.0, p[1] + 9.0],
                [
                    p[0] + i as f32 * cw,
                    p[1],
                    p[0] + (i + 1) as f32 * cw,
                    p[1] + 36.0,
                ],
                t.text_secondary,
            );
        }
        let mouse = ui.mouse_position();
        let mut selected = ui.widget_state::<TableState>(&self.id).selected;
        for (r, row) in self.rows.iter().enumerate() {
            let y = p[1] + 36.0 + r as f32 * 32.0;
            let hit = mouse[0] >= p[0]
                && mouse[0] <= p[0] + s[0]
                && mouse[1] >= y
                && mouse[1] <= y + 32.0;
            if hit && ui.mouse_pressed_this_frame() {
                selected = Some(r);
                ui.widget_state::<TableState>(&self.id).selected = selected;
            }
            ui.draw_rect(
                [p[0], y],
                [s[0], 32.0],
                Fill::Solid(if selected == Some(r) {
                    t.selection
                } else {
                    t.surface
                }),
                0.0,
                1.0,
                t.border_faint,
                0.0,
                false,
                0.0,
            );
            for (i, value) in row.iter().enumerate() {
                ui.draw_text_colored(
                    value,
                    [p[0] + i as f32 * cw + 10.0, y + 7.0],
                    [
                        p[0] + i as f32 * cw,
                        y,
                        p[0] + (i + 1) as f32 * cw,
                        y + 32.0,
                    ],
                    t.text_primary,
                );
            }
        }
        selected
    }
}

pub struct Carousel {
    id: String,
    slides: Vec<String>,
    width: f32,
    height: f32,
}
#[derive(Default)]
pub struct CarouselState {
    pub index: usize,
}
impl Carousel {
    pub fn new(id: impl Into<String>, slides: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            slides: slides.into_iter().map(Into::into).collect(),
            width: 360.0,
            height: 180.0,
        }
    }
    pub fn size(mut self, s: [f32; 2]) -> Self {
        self.width = s[0];
        self.height = s[1];
        self
    }
    pub fn index(&self, ui: &mut Ui) -> usize {
        ui.widget_state::<CarouselState>(&self.id).index
    }
}
impl Widget for Carousel {
    type Output = usize;
    fn ui(&mut self, ui: &mut Ui) -> usize {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for Carousel {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> usize {
        let n = self.slides.len().max(1);
        let m = ui.mouse_position();
        let mut index = self.index(ui).min(n - 1);
        let button = 28.0;
        if ui.mouse_pressed_this_frame() && m[1] >= p[1] + s[1] - button && m[1] <= p[1] + s[1] {
            if m[0] < p[0] + button {
                index = index.saturating_sub(1)
            } else if m[0] > p[0] + s[0] - button {
                index = (index + 1).min(n - 1)
            }
            ui.widget_state::<CarouselState>(&self.id).index = index;
        }
        let t = *ui.theme();
        ui.draw_rect(
            p,
            s,
            Fill::Solid(t.surface_subtle),
            Theme::RADIUS_MD,
            1.0,
            t.border,
            0.0,
            false,
            0.0,
        );
        if let Some(text) = self.slides.get(index) {
            ui.draw_text_styled(
                text,
                [p[0] + 20.0, p[1] + (s[1] - 24.0) * 0.5],
                [
                    p[0] + 16.0,
                    p[1] + 16.0,
                    p[0] + s[0] - 16.0,
                    p[1] + s[1] - 32.0,
                ],
                t.text_primary,
                18.0,
                24.0,
                FontWeight::SemiBold,
                false,
            );
        }
        ui.draw_text_colored(
            "‹",
            [p[0] + 10.0, p[1] + s[1] - button + 4.0],
            [p[0], p[1], p[0] + button, p[1] + s[1]],
            t.text_secondary,
        );
        ui.draw_text_colored(
            "›",
            [p[0] + s[0] - button + 10.0, p[1] + s[1] - button + 4.0],
            [p[0], p[1], p[0] + s[0], p[1] + s[1]],
            t.text_secondary,
        );
        index
    }
}

pub struct Chart {
    values: Vec<f32>,
    width: f32,
    height: f32,
    color: Option<Color>,
}
impl Chart {
    pub fn new(values: impl IntoIterator<Item = f32>) -> Self {
        Self {
            values: values.into_iter().collect(),
            width: 360.0,
            height: 160.0,
            color: None,
        }
    }
    pub fn size(mut self, s: [f32; 2]) -> Self {
        self.width = s[0];
        self.height = s[1];
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }
}
impl Widget for Chart {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui);
    }
}
impl Measurable for Chart {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) {
        let t = *ui.theme();
        ui.draw_rect(
            p,
            s,
            Fill::Solid(t.surface),
            Theme::RADIUS_MD,
            1.0,
            t.border,
            0.0,
            false,
            0.0,
        );
        let max = self.values.iter().copied().fold(0.0_f32, f32::max).max(1.0);
        let count = self.values.len().max(1);
        let gap = 6.0;
        let bw = ((s[0] - gap * (count as f32 + 1.0)) / count as f32).max(1.0);
        let c = self.color.unwrap_or(t.active);
        for (i, v) in self.values.iter().enumerate() {
            let h = (v / max * (s[1] - 24.0)).max(1.0);
            let x = p[0] + gap + i as f32 * (bw + gap);
            ui.draw_rect(
                [x, p[1] + s[1] - h - 12.0],
                [bw, h],
                Fill::Solid(c),
                Theme::RADIUS_SM,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub id: String,
    pub label: String,
    pub hint: Option<String>,
}
impl Command {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            hint: None,
        }
    }
    pub fn hint(mut self, h: impl Into<String>) -> Self {
        self.hint = Some(h.into());
        self
    }
}

pub struct CommandPalette {
    id: String,
    commands: Vec<Command>,
    width: f32,
}
#[derive(Default)]
pub struct CommandPaletteState {
    pub open: bool,
    pub query: String,
    pub active: usize,
}
impl CommandPalette {
    pub fn new(id: impl Into<String>, commands: impl IntoIterator<Item = Command>) -> Self {
        Self {
            id: id.into(),
            commands: commands.into_iter().collect(),
            width: 460.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn open(&self, ui: &mut Ui) -> bool {
        ui.widget_state::<CommandPaletteState>(&self.id).open
    }
    pub fn set_open(&self, ui: &mut Ui, v: bool) {
        ui.widget_state::<CommandPaletteState>(&self.id).open = v
    }
}
impl Widget for CommandPalette {
    type Output = Option<String>;
    fn ui(&mut self, ui: &mut Ui) -> Option<String> {
        let mut state = ui.take_widget_state::<CommandPaletteState>(&self.id);
        if ui.key_pressed(Key::Named(NamedKey::Escape)) {
            state.open = false;
        }
        if !state.open {
            ui.put_widget_state(&self.id, state);
            return None;
        }
        for ch in ui.typed_text().chars() {
            if !ch.is_control() {
                state.query.push(ch);
            }
        }
        if ui.key_pressed(Key::Named(NamedKey::Backspace)) {
            state.query.pop();
        }
        let needle = state.query.to_lowercase();
        let filtered: Vec<usize> = self
            .commands
            .iter()
            .enumerate()
            .filter(|(_, c)| c.label.to_lowercase().contains(&needle))
            .map(|(i, _)| i)
            .collect();
        if !filtered.is_empty() {
            state.active = state.active.min(filtered.len() - 1);
            if ui.key_pressed(Key::Named(NamedKey::ArrowDown)) {
                state.active = (state.active + 1).min(filtered.len() - 1);
            }
            if ui.key_pressed(Key::Named(NamedKey::ArrowUp)) {
                state.active = state.active.saturating_sub(1);
            }
            if ui.key_pressed(Key::Named(NamedKey::Enter)) {
                let result = self.commands[filtered[state.active]].id.clone();
                state.open = false;
                ui.put_widget_state(&self.id, state);
                return Some(result);
            }
        }
        let window = ui.window_size();
        let h = (filtered.len().min(7) as f32 * 34.0 + 82.0).max(116.0);
        let p = [(window[0] - self.width) * 0.5, 72.0];
        let theme = *ui.theme();
        ui.push_input_block([0.0, 0.0, window[0], window[1]]);
        ui.draw_overlay_rect(
            [0.0, 0.0],
            window,
            Fill::Solid(Color::BLACK.with_alpha(0.25)),
            0.0,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            [0.0, 0.0, window[0], window[1]],
            0.0,
        );
        ui.draw_overlay_rect(
            p,
            [self.width, h],
            Fill::Solid(theme.surface_elevated),
            Theme::RADIUS_LG,
            1.0,
            theme.border,
            20.0,
            false,
            [0.0, 0.0, window[0], window[1]],
            0.0,
        );
        let query = if state.query.is_empty() {
            "Search commands…"
        } else {
            &state.query
        };
        ui.draw_overlay_text_styled(
            query,
            [p[0] + 18.0, p[1] + 18.0],
            [p[0], p[1], p[0] + self.width, p[1] + 48.0],
            theme.text_secondary,
            14.0,
            20.0,
            FontWeight::Regular,
            false,
        );
        for (row, &index) in filtered.iter().take(7).enumerate() {
            let y = p[1] + 58.0 + row as f32 * 34.0;
            let active = row == state.active;
            ui.draw_overlay_rect(
                [p[0] + 8.0, y],
                [self.width - 16.0, 30.0],
                Fill::Solid(if active {
                    theme.surface_subtle
                } else {
                    Color::TRANSPARENT
                }),
                Theme::RADIUS_SM,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                [0.0, 0.0, window[0], window[1]],
                0.0,
            );
            ui.draw_overlay_text_styled(
                &self.commands[index].label,
                [p[0] + 18.0, y + 6.0],
                [p[0], p[1], p[0] + self.width, p[1] + h],
                theme.text_primary,
                13.0,
                18.0,
                FontWeight::Medium,
                false,
            );
        }
        ui.put_widget_state(&self.id, state);
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub struct Calendar {
    id: String,
    date: CalendarDate,
    width: f32,
}
#[derive(Default)]
pub struct CalendarState {
    pub selected: Option<CalendarDate>,
}
impl Calendar {
    pub fn new(id: impl Into<String>, date: CalendarDate) -> Self {
        Self {
            id: id.into(),
            date,
            width: 320.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> Option<CalendarDate> {
        ui.widget_state::<CalendarState>(&self.id).selected
    }
}
impl Widget for Calendar {
    type Output = Option<CalendarDate>;
    fn ui(&mut self, ui: &mut Ui) -> Option<CalendarDate> {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for Calendar {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 250.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> Option<CalendarDate> {
        let t = *ui.theme();
        ui.draw_rect(
            p,
            s,
            Fill::Solid(t.surface),
            Theme::RADIUS_MD,
            1.0,
            t.border,
            0.0,
            false,
            0.0,
        );
        let title = format!("{} / {}", self.date.year, self.date.month);
        ui.draw_text_styled(
            &title,
            [p[0] + 16.0, p[1] + 14.0],
            [p[0], p[1], p[0] + s[0], p[1] + 35.0],
            t.text_primary,
            15.0,
            20.0,
            FontWeight::SemiBold,
            false,
        );
        let cell = s[0] / 7.0;
        let start_y = p[1] + 50.0;
        let days = days_in_month(self.date.year, self.date.month);
        let first = weekday(self.date.year, self.date.month, 1);
        let mouse = ui.mouse_position();
        let mut selected = ui.widget_state::<CalendarState>(&self.id).selected;
        for day in 1..=days {
            let index = first + day as usize - 1;
            let row = index / 7;
            let col = index % 7;
            let x = p[0] + col as f32 * cell;
            let y = start_y + row as f32 * 30.0;
            let hit =
                mouse[0] >= x && mouse[0] <= x + cell && mouse[1] >= y && mouse[1] <= y + 30.0;
            if hit && ui.mouse_pressed_this_frame() {
                selected = Some(CalendarDate {
                    year: self.date.year,
                    month: self.date.month,
                    day,
                });
                ui.widget_state::<CalendarState>(&self.id).selected = selected;
            }
            let active = selected
                == Some(CalendarDate {
                    year: self.date.year,
                    month: self.date.month,
                    day,
                });
            if active {
                ui.draw_rect(
                    [x + 3.0, y + 2.0],
                    [cell - 6.0, 26.0],
                    Fill::Solid(t.active),
                    Theme::RADIUS_SM,
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    false,
                    0.0,
                );
            }
            ui.draw_text_colored(
                &day.to_string(),
                [x + cell * 0.5 - 4.0, y + 6.0],
                [x, y, x + cell, y + 30.0],
                if active { Color::WHITE } else { t.text_primary },
            );
        }
        selected
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
fn weekday(year: i32, month: u32, day: u32) -> usize {
    let mut y = year;
    let mut m = month as i32;
    if m < 3 {
        y -= 1;
        m += 12;
    }
    (((day as i32 + ((13 * (m + 1)) / 5) + y + (y / 4) - (y / 100) + (y / 400)) % 7 + 6) % 7)
        as usize
}

pub struct DatePicker {
    calendar: Calendar,
}
impl DatePicker {
    pub fn new(id: impl Into<String>, date: CalendarDate) -> Self {
        Self {
            calendar: Calendar::new(id, date),
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.calendar = self.calendar.width(w);
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> Option<CalendarDate> {
        self.calendar.selected(ui)
    }
}
impl Widget for DatePicker {
    type Output = Option<CalendarDate>;
    fn ui(&mut self, ui: &mut Ui) -> Option<CalendarDate> {
        self.calendar.ui(ui)
    }
}
impl Measurable for DatePicker {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        self.calendar.measure(ui)
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> Option<CalendarDate> {
        self.calendar.arrange(p, s, ui)
    }
}

pub struct InputOtp {
    id: String,
    length: usize,
    width: f32,
}
#[derive(Default)]
pub struct InputOtpState {
    pub value: String,
}
impl InputOtp {
    pub fn new(id: impl Into<String>, length: usize) -> Self {
        Self {
            id: id.into(),
            length: length.clamp(1, 12),
            width: length as f32 * 38.0,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn value(&self, ui: &mut Ui) -> String {
        ui.widget_state::<InputOtpState>(&self.id).value.clone()
    }
}
impl Widget for InputOtp {
    type Output = String;
    fn ui(&mut self, ui: &mut Ui) -> String {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for InputOtp {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 42.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> String {
        let mut state = ui.take_widget_state::<InputOtpState>(&self.id);
        for ch in ui.typed_text().chars().filter(|c| c.is_ascii_digit()) {
            if state.value.len() < self.length {
                state.value.push(ch);
            }
        }
        if ui.key_pressed(Key::Named(NamedKey::Backspace)) {
            state.value.pop();
        }
        let t = *ui.theme();
        let gap = 6.0;
        let w = (s[0] - gap * (self.length.saturating_sub(1) as f32)) / self.length as f32;
        for i in 0..self.length {
            let x = p[0] + i as f32 * (w + gap);
            let filled = state.value.chars().nth(i);
            ui.draw_rect(
                [x, p[1]],
                [w, s[1]],
                Fill::Solid(t.surface),
                Theme::RADIUS_MD,
                1.0,
                if filled.is_some() { t.active } else { t.border },
                0.0,
                false,
                0.0,
            );
            if let Some(ch) = filled {
                ui.draw_text_styled(
                    &ch.to_string(),
                    [x + w * 0.5 - 4.0, p[1] + 10.0],
                    [x, p[1], x + w, p[1] + s[1]],
                    t.text_primary,
                    16.0,
                    20.0,
                    FontWeight::SemiBold,
                    false,
                );
            }
        }
        let value = state.value.clone();
        ui.put_widget_state(&self.id, state);
        value
    }
}

pub struct ToggleGroup {
    id: String,
    labels: Vec<String>,
    multiple: bool,
    width: f32,
}
#[derive(Default)]
pub struct ToggleGroupState {
    pub selected: Vec<usize>,
}
impl ToggleGroup {
    pub fn new(id: impl Into<String>, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            labels: labels.into_iter().map(Into::into).collect(),
            multiple: false,
            width: 320.0,
        }
    }
    pub fn multiple(mut self, v: bool) -> Self {
        self.multiple = v;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn selected(&self, ui: &mut Ui) -> Vec<usize> {
        ui.widget_state::<ToggleGroupState>(&self.id)
            .selected
            .clone()
    }
}
impl Widget for ToggleGroup {
    type Output = Vec<usize>;
    fn ui(&mut self, ui: &mut Ui) -> Vec<usize> {
        let s = self.measure(ui);
        self.arrange([0.0, 0.0], s, ui)
    }
}
impl Measurable for ToggleGroup {
    fn measure(&mut self, _: &mut Ui) -> [f32; 2] {
        [self.width, 34.0]
    }
    fn arrange(&mut self, p: [f32; 2], s: [f32; 2], ui: &mut Ui) -> Vec<usize> {
        let n = self.labels.len().max(1);
        let w = s[0] / n as f32;
        let mouse = ui.mouse_position();
        let mut selected = ui
            .widget_state::<ToggleGroupState>(&self.id)
            .selected
            .clone();
        if ui.mouse_pressed_this_frame()
            && mouse[0] >= p[0]
            && mouse[0] <= p[0] + s[0]
            && mouse[1] >= p[1]
            && mouse[1] <= p[1] + s[1]
        {
            let i = ((mouse[0] - p[0]) / w) as usize;
            if self.multiple {
                if let Some(pos) = selected.iter().position(|x| *x == i) {
                    selected.remove(pos);
                } else {
                    selected.push(i)
                }
            } else {
                selected = vec![i]
            }
            ui.widget_state::<ToggleGroupState>(&self.id).selected = selected.clone();
        }
        let t = *ui.theme();
        for (i, label) in self.labels.iter().enumerate() {
            let x = p[0] + i as f32 * w;
            let active = selected.contains(&i);
            ui.draw_rect(
                [x, p[1]],
                [w, s[1]],
                Fill::Solid(if active { t.active } else { t.surface_subtle }),
                Theme::RADIUS_SM,
                1.0,
                t.border,
                0.0,
                false,
                0.0,
            );
            ui.draw_text_colored(
                label,
                [x + 8.0, p[1] + 8.0],
                [x, p[1], x + w, p[1] + s[1]],
                if active { Color::WHITE } else { t.text_primary },
            );
        }
        selected
    }
}
