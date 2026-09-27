use kurbo::{BezPath, PathEl};

pub const FLATTEN_TOLERANCE: f64 = 0.2;

pub fn flatten_contours(path: &BezPath, tolerance: f64) -> Vec<Vec<[f32; 2]>> {
    let mut contours: Vec<Vec<[f32; 2]>> = Vec::new();
    let mut current: Vec<[f32; 2]> = Vec::new();

    kurbo::flatten(path.elements().iter().copied(), tolerance, |el| match el {
        PathEl::MoveTo(p) => {
            if current.len() >= 3 {
                contours.push(std::mem::take(&mut current));
            }
            current.clear();
            current.push([p.x as f32, p.y as f32]);
        }
        PathEl::LineTo(p) => current.push([p.x as f32, p.y as f32]),
        PathEl::ClosePath => {
            if current.len() >= 3 {
                contours.push(std::mem::take(&mut current));
            } else {
                current.clear();
            }
        }
        PathEl::QuadTo(..) | PathEl::CurveTo(..) => {}
    });

    if current.len() >= 3 {
        contours.push(current);
    }

    contours
}

pub fn flatten_polyline(path: &BezPath, tolerance: f64) -> Vec<[f32; 2]> {
    let mut points: Vec<[f32; 2]> = Vec::new();

    kurbo::flatten(path.elements().iter().copied(), tolerance, |el| match el {
        PathEl::MoveTo(p) | PathEl::LineTo(p) => points.push([p.x as f32, p.y as f32]),
        PathEl::ClosePath => {
            if let Some(&first) = points.first() {
                points.push(first);
            }
        }
        PathEl::QuadTo(..) | PathEl::CurveTo(..) => {}
    });

    points
}
