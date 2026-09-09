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
