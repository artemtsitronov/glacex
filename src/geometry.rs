use crate::fill::Fill;
use kurbo::{BezPath, Ellipse, Point, RoundedRect, Shape as KurboShape};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurablePath {
    Rect(RoundedRect),
    Ellipse(Ellipse),
    Free(BezPath),
    Open { centerline: BezPath, thickness: f32 },
}

impl MeasurablePath {
    pub fn rect(pos: [f32; 2], size: [f32; 2], radii: [f32; 4]) -> Self {
        Self::Rect(RoundedRect::from_origin_size(
            (pos[0] as f64, pos[1] as f64),
            (size[0] as f64, size[1] as f64),
            (
                radii[0] as f64,
                radii[1] as f64,
                radii[2] as f64,
                radii[3] as f64,
            ),
        ))
    }
    pub fn ellipse(pos: [f32; 2], radii: [f32; 2], rot: f32) -> Self {
        Self::Ellipse(Ellipse::new(
            (pos[0] as f64, pos[1] as f64),
            (radii[0] as f64, radii[1] as f64),
            rot as f64,
        ))
    }
    pub fn free(bez_path: BezPath) -> Self {
        Self::Free(bez_path)
    }
    pub fn open(centerline: BezPath, thickness: f32) -> Self {
        Self::Open {
            centerline,
            thickness,
        }
    }

    pub fn contains(&self, point: [f32; 2]) -> bool {
        let p = Point::new(point[0] as f64, point[1] as f64);
        match self {
            Self::Rect(rect) => rect.contains(p),
            Self::Ellipse(ellipse) => ellipse.contains(p),
            Self::Free(bez_path) => bez_path.contains(p),
            Self::Open { .. } => false,
        }
    }
}

#[derive(Clone)]
pub struct Path(Arc<dyn Fn([f32; 2], [f32; 2]) -> MeasurablePath + Send + Sync>);

impl Path {
    pub fn from_fn(
        f: impl Fn([f32; 2], [f32; 2]) -> MeasurablePath + Send + Sync + 'static,
    ) -> Self {
        Path(Arc::new(f))
    }

    pub fn rect(radii: [f32; 4]) -> Self {
        Path(Arc::new(move |position, size| {
            MeasurablePath::rect(position, size, radii)
        }))
    }

    pub fn ellipse(rotation_degrees: f32) -> Self {
        Path(Arc::new(move |position, size| {
            let radii = [size[0] * 0.5, size[1] * 0.5];
            MeasurablePath::ellipse(
                [position[0] + radii[0], position[1] + radii[1]],
                radii,
                rotation_degrees.to_radians(),
            )
        }))
    }
}

impl std::ops::Deref for Path {
    type Target = dyn Fn([f32; 2], [f32; 2]) -> MeasurablePath + Send + Sync;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub fn straight_path(length: f32, thickness: f32) -> MeasurablePath {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((length as f64, 0.0));
    MeasurablePath::open(path, thickness)
}

#[macro_export]
macro_rules! straight_path {
    ($length:expr, $thickness:expr) => {
        $crate::straight_path($length, $thickness)
    };
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub path: MeasurablePath,
    pub fill: Fill,
}
