use kurbo::{BezPath, PathEl, Point};

pub const FLATTEN_TOLERANCE: f64 = 0.2;

#[inline(always)]
fn pt(p: Point) -> [f32; 2] {
    [p.x as f32, p.y as f32]
}

#[inline(always)]
fn push_dedup(points: &mut Vec<[f32; 2]>, p: [f32; 2]) {
    if points.last() != Some(&p) {
        points.push(p);
    }
}

#[inline(always)]
fn finish_contour(contours: &mut Vec<Vec<[f32; 2]>>, current: &mut Vec<[f32; 2]>) {
    if current.len() > 1 && current.first() == current.last() {
        current.pop();
    }
    if current.len() >= 3 {
        let cap = current.capacity();
        contours.push(std::mem::replace(current, Vec::with_capacity(cap)));
    } else {
        current.clear();
    }
}

pub fn flatten_contours(path: &BezPath, tolerance: f64) -> Vec<Vec<[f32; 2]>> {
    let elements = path.elements();
    let mut contours: Vec<Vec<[f32; 2]>> = Vec::new();
    let mut current: Vec<[f32; 2]> = Vec::with_capacity(elements.len() * 4);

    kurbo::flatten(elements.iter().copied(), tolerance, |el| match el {
        PathEl::MoveTo(p) => {
            finish_contour(&mut contours, &mut current);
            current.push(pt(p));
        }
        PathEl::LineTo(p) => push_dedup(&mut current, pt(p)),
        PathEl::ClosePath => finish_contour(&mut contours, &mut current),
        PathEl::QuadTo(..) | PathEl::CurveTo(..) => {}
    });

    finish_contour(&mut contours, &mut current);
    contours
}

pub fn flatten_polyline(path: &BezPath, tolerance: f64) -> Vec<[f32; 2]> {
    let elements = path.elements();
    let mut points: Vec<[f32; 2]> = Vec::with_capacity(elements.len() * 4);
    let mut start: Option<[f32; 2]> = None;

    kurbo::flatten(elements.iter().copied(), tolerance, |el| match el {
        PathEl::MoveTo(p) => {
            let p = pt(p);
            start = Some(p);
            push_dedup(&mut points, p);
        }
        PathEl::LineTo(p) => push_dedup(&mut points, pt(p)),
        PathEl::ClosePath => {
            if let Some(s) = start {
                push_dedup(&mut points, s);
            }
        }
        PathEl::QuadTo(..) | PathEl::CurveTo(..) => {}
    });

    points
}
