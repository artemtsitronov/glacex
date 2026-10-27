use crate::animation::Motion;
use crate::border::Border;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::{animate_hover, stored_hover};
use crate::stateful::{StateWeights, Stateful};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id, is_anonymous};
use accesskit::{NodeId, Role};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct DividerStyle {
    pub fill: Stateful<Fill>,
    pub thickness: Stateful<f32>,
    pub margin: Stateful<f32>,
    pub inset: Stateful<[f32; 2]>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
}

impl DividerStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        DividerStyle {
            fill: Stateful::new(Fill::Solid(theme.border)),
            thickness: Stateful::new(theme.metrics.border_width),
            margin: Stateful::new(0.0),
            inset: Stateful::new([0.0, 0.0]),
            sharp: true,
            path: Stateful::new(Path::rect([0.0; 4])),
            hover_motion: Motion::SNAPPY,
        }
    }

    pub fn faint(theme: &Theme) -> Self {
        DividerStyle {
            fill: Stateful::new(Fill::Solid(theme.border_faint)),
            ..Self::from_theme(theme)
        }
    }

    pub fn strong(theme: &Theme) -> Self {
        DividerStyle {
            fill: Stateful::new(Fill::Solid(theme.border_strong)),
            ..Self::from_theme(theme)
        }
    }

    pub fn rounded(theme: &Theme, thickness: f32) -> Self {
        DividerStyle {
            thickness: Stateful::new(thickness),
            sharp: false,
            path: Stateful::new(Path::from_fn(|position, size| {
                MeasurablePath::rect(position, size, [size[0].min(size[1]) * 0.5; 4])
            })),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for DividerStyle {
    fn default() -> Self {
        DividerStyle::from_theme(&Theme::default())
    }
}

pub struct Divider {
    id: String,
    orientation: DividerOrientation,
    length: f32,
    thickness: Option<Stateful<f32>>,
    color: Option<Stateful<Fill>>,
    faint: bool,
    style: Option<DividerStyle>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Divider {
    pub fn horizontal(length: f32) -> Self {
        Divider {
            id: generate_uuid(),
            orientation: DividerOrientation::Horizontal,
            length,
            thickness: None,
            color: None,
            faint: false,
            style: None,
            width: None,
            height: None,
        }
    }

    pub fn vertical(length: f32) -> Self {
        Divider {
            id: generate_uuid(),
            orientation: DividerOrientation::Vertical,
            length,
            thickness: None,
            color: None,
            faint: false,
            style: None,
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
    }

    pub fn thickness(mut self, thickness: impl Into<Stateful<f32>>) -> Self {
        self.thickness = Some(thickness.into());
        self
    }

    pub fn color(mut self, color: impl Into<Stateful<Fill>>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn faint(mut self) -> Self {
        self.faint = true;
        self
    }

    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<DividerStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> DividerStyle {
        let mut style = self.style.clone().unwrap_or_else(|| {
            if self.faint {
                DividerStyle::faint(theme)
            } else {
                DividerStyle::from_theme(theme)
            }
        });
        if let Some(color) = &self.color {
            style.fill = color.clone();
        }
        if let Some(thickness) = &self.thickness {
            style.thickness = thickness.clone();
        }
        style
    }
}

impl Widget for Divider {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Divider {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let weights = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let across = style.thickness.resolve(weights) + style.margin.resolve(weights) * 2.0;
        let natural = match self.orientation {
            DividerOrientation::Horizontal => [self.length, across],
            DividerOrientation::Vertical => [across, self.length],
        };
        [
            self.width.unwrap_or(natural[0]),
            self.height.unwrap_or(natural[1]),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        // anonymous dividers share a fallback id, only register named ones
        if !is_anonymous(&self.id) {
            ui.register_accessible(
                self,
                [
                    position[0],
                    position[1],
                    position[0] + size[0],
                    position[1] + size[1],
                ],
            );
        }

        let style = self.resolved_style(ui.theme());
        let hovered = {
            let mouse_pos = ui.mouse_position();
            !ui.is_input_blocked(mouse_pos)
                && ui.point_in_current_clip(mouse_pos)
                && MeasurablePath::rect(position, size, [0.0; 4]).contains(mouse_pos)
        };
        let weights =
            StateWeights::IDLE.hovered(animate_hover(ui, &self.id, hovered, style.hover_motion));
        let inset = style.inset.resolve(weights);
        let (line_position, line_size) = match self.orientation {
            DividerOrientation::Horizontal => {
                let thickness = style.thickness.resolve(weights).min(size[1]);
                (
                    [
                        position[0] + inset[0],
                        position[1] + (size[1] - thickness) / 2.0,
                    ],
                    [(size[0] - inset[0] - inset[1]).max(0.0), thickness],
                )
            }
            DividerOrientation::Vertical => {
                let thickness = style.thickness.resolve(weights).min(size[0]);
                (
                    [
                        position[0] + (size[0] - thickness) / 2.0,
                        position[1] + inset[0],
                    ],
                    [thickness, (size[1] - inset[0] - inset[1]).max(0.0)],
                )
            }
        };

        ui.draw_shape(
            (style.path.resolve(weights))(line_position, line_size),
            style.fill.resolve(weights),
            Border::NONE,
            0.0,
            style.sharp,
            0.0,
        );
    }
}

impl Accessible for Divider {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Splitter
    }
}
