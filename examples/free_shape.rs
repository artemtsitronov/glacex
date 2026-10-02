//! free_shape -- custom bezier art with zero widgets and zero layout.
//!
//! sometimes you don't want buttons. you want a star.
//! `ui.draw_shape` / `ui.draw_open_shape` draw any `kurbo` path directly,
//! no `row!`, no `column!`, no answers to give. just pixels.
//!
//! run it: `cargo run --example free_shape`

use glacex::{
    App, Color, Fill, Gradient, GradientKind, GradientStop, MeasurablePath, Theme, Ui, Widget,
};
use kurbo::BezPath;
use std::f64::consts::PI;

// a stateless app: no fields, because there is nothing to remember.
// the art is recomputed from scratch every frame. the gpu doesn't mind.
struct FreeShapeDemo;

// builds a star polygon as a kurbo path: alternating outer/inner radius,
// one point per step, closed at the end. geometry homework, finally useful.
fn star_path(center: [f32; 2], outer: f32, inner: f32, points: usize) -> BezPath {
    let mut path = BezPath::new();
    for i in 0..points * 2 {
        let radius = if i % 2 == 0 { outer } else { inner };
        let angle = -PI / 2.0 + i as f64 * PI / points as f64;
        let x = center[0] as f64 + radius as f64 * angle.cos();
        let y = center[1] as f64 + radius as f64 * angle.sin();
        if i == 0 {
            path.move_to((x, y));
        } else {
            path.line_to((x, y));
        }
    }
    path.close_path();
    path
}

impl Widget for FreeShapeDemo {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        // draw_shape(path, fill, border_width, border_color, blur, sharp, rotation)
        // argument order is fixed -- after the third shape you'll know it by heart.

        // -- star 1: solid fill, dark outline --------------------------------
        ui.draw_shape(
            MeasurablePath::free(star_path([160.0, 180.0], 90.0, 38.0, 5)),
            Fill::Solid(Color::hex_str("#f59e0b")),
            4.0,                       // border width
            Color::hex_str("#1e293b"), // border color
            0.0,                       // blur radius (0 = crisp)
            false,                     // sharp corners?
            0.0,                       // rotation in radians
        );

        // -- star 2: gradient fill, slightly rotated, no border --------------
        ui.draw_shape(
            MeasurablePath::free(star_path([420.0, 180.0], 90.0, 38.0, 6)),
            Fill::Gradient(Gradient {
                kind: GradientKind::Linear { angle: 45.0 }, // degrees
                stops: vec![
                    GradientStop {
                        position: 0.0,
                        color: Color::hex_str("#6366f1"),
                    },
                    GradientStop {
                        position: 1.0,
                        color: Color::hex_str("#ec4899"),
                    },
                ],
            }),
            0.0,                // no border
            Color::TRANSPARENT, // (border color irrelevant at width 0)
            0.0,
            false,
            0.3, // a jaunty tilt
        );

        // -- open paths: strokes with no interior -----------------------------
        // MeasurablePath::open(centerline, thickness) draws a line that can be
        // partially "revealed" (0.0 = invisible, 1.0 = full) -- perfect for
        // progress tracks, sparklines, and squiggles.

        let mut track = BezPath::new();
        track.move_to((600.0, 180.0));
        track.line_to((900.0, 180.0));

        // the full track, dark underneath...
        ui.draw_open_shape(
            MeasurablePath::open(track.clone(), 16.0),
            Fill::Solid(Color::hex_str("#334155")),
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
            1.0, // reveal: the whole thing
        );
        // ...with 60% of green progress on top.
        ui.draw_open_shape(
            MeasurablePath::open(track, 16.0),
            Fill::Solid(Color::hex_str("#22c55e")),
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
            0.6, // reveal: 60%. animate this per frame for a loading bar with style.
        );

        // a quadratic curve, 75% revealed. decorative. no apologies.
        let mut wave = BezPath::new();
        wave.move_to((600.0, 300.0));
        wave.quad_to((750.0, 220.0), (900.0, 300.0));
        ui.draw_open_shape(
            MeasurablePath::open(wave, 10.0),
            Fill::Solid(Color::hex_str("#f59e0b")),
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
            0.75,
        );
    }
}

fn main() {
    App::new(FreeShapeDemo).run();
}
