use crate::animation::Motion;
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::MeasurablePath;
use crate::stateful::{StateWeights, Stateful};
use crate::ui::Ui;

#[derive(Clone)]
pub struct ChevronStyle {
    pub color: Stateful<Option<Color>>,
    pub size: Stateful<f32>,
    pub thickness: Stateful<f32>,
    pub open_angle: Stateful<f32>,
    pub space: Stateful<f32>,
    pub motion: f32,
}

impl Default for ChevronStyle {
    fn default() -> Self {
        ChevronStyle {
            color: Stateful::new(None),
            size: Stateful::new(4.5),
            thickness: Stateful::new(1.8),
            open_angle: Stateful::new(180.0),
            space: Stateful::new(24.0),
            motion: Motion::SNAPPY,
        }
    }
}

impl ChevronStyle {
    pub fn resolve(&self, weights: StateWeights, fallback_color: Color, padding_x: f32) -> Chevron {
        Chevron {
            color: self.color.resolve(weights).unwrap_or(fallback_color),
            size: self.size.resolve(weights),
            thickness: self.thickness.resolve(weights),
            padding_x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Chevron {
    pub color: Color,
    pub size: f32,
    pub thickness: f32,
    pub padding_x: f32,
}

// chevron stroke endpoints in world space, shared by both chevron drawers
pub fn chevron_points(
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &Chevron,
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

pub fn draw_chevron(
    ui: &mut Ui,
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &Chevron,
) {
    let (tip, left, right) = chevron_points(position, size, angle_deg, chevron);
    draw_line(ui, tip, left, chevron.thickness, chevron.color);
    draw_line(ui, tip, right, chevron.thickness, chevron.color);
}

// overlay version of draw_chevron, keeps it above the open dropdown
pub fn draw_chevron_overlay(
    ui: &mut Ui,
    position: [f32; 2],
    size: [f32; 2],
    angle_deg: f32,
    chevron: &Chevron,
    clip: [f32; 4],
) {
    let (tip, left, right) = chevron_points(position, size, angle_deg, chevron);
    draw_line_overlay(ui, tip, left, chevron.thickness, chevron.color, clip);
    draw_line_overlay(ui, tip, right, chevron.thickness, chevron.color, clip);
}

pub fn draw_check(ui: &mut Ui, origin: [f32; 2], color: Color, thickness: f32, clip: [f32; 4]) {
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

pub fn draw_line(ui: &mut Ui, a: [f32; 2], b: [f32; 2], thickness: f32, color: Color) {
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

pub fn draw_line_overlay(
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
