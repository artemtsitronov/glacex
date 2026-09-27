use glacex::{
    App, Color, Fill, Gradient, GradientKind, GradientStop, MeasurablePath, Theme, Ui, Widget,
};
use kurbo::BezPath;
use std::f64::consts::PI;

struct FreeShapeDemo;

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

        ui.draw_shape(
            MeasurablePath::free(star_path([160.0, 180.0], 90.0, 38.0, 5)),
            Fill::Solid(Color::hex_str("#f59e0b")),
            4.0,
            Color::hex_str("#1e293b"),
            0.0,
            false,
            0.0,
        );

        ui.draw_shape(
            MeasurablePath::free(star_path([420.0, 180.0], 90.0, 38.0, 6)),
            Fill::Gradient(Gradient {
                kind: GradientKind::Linear { angle: 45.0 },
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
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.3,
        );

        let mut track = BezPath::new();
        track.move_to((600.0, 180.0));
        track.line_to((900.0, 180.0));
        ui.draw_open_shape(
            MeasurablePath::open(track.clone(), 16.0),
            Fill::Solid(Color::hex_str("#334155")),
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
            1.0,
        );
        ui.draw_open_shape(
            MeasurablePath::open(track, 16.0),
            Fill::Solid(Color::hex_str("#22c55e")),
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
            0.6,
        );

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
