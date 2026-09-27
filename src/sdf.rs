use crate::tessellate::{FLATTEN_TOLERANCE, flatten_contours};
use kurbo::{BezPath, Point, Rect, Shape};

fn segment_distance(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ap = [p[0] - a[0], p[1] - a[1]];
    let t = ((ap[0] * ab[0] + ap[1] * ab[1]) / (ab[0] * ab[0] + ab[1] * ab[1]).max(1e-6))
        .clamp(0.0, 1.0);
    let closest = [a[0] + ab[0] * t, a[1] + ab[1] * t];
    ((p[0] - closest[0]).powi(2) + (p[1] - closest[1]).powi(2)).sqrt()
}

pub fn bake_sdf(path: &BezPath, bounds: Rect, w: usize, h: usize) -> Vec<f32> {
    let edges: Vec<[[f32; 2]; 2]> = flatten_contours(path, FLATTEN_TOLERANCE)
        .iter()
        .flat_map(|c| (0..c.len()).map(|i| [c[i], c[(i + 1) % c.len()]]))
        .collect();

    let mut field = vec![0.0f32; w * h];
    for y in 0..h {
        for x in 0..w {
            let px = bounds.x0 as f32 + (x as f32 + 0.5) / w as f32 * bounds.width() as f32;
            let py = bounds.y0 as f32 + (y as f32 + 0.5) / h as f32 * bounds.height() as f32;
            let dist = edges
                .iter()
                .map(|[a, b]| segment_distance([px, py], *a, *b))
                .fold(f32::MAX, f32::min);
            let inside = path.contains(Point::new(px as f64, py as f64));
            field[y * w + x] = if inside { -dist } else { dist };
        }
    }
    field
}

// segments longer than the stroke half width show round bumps at every joint
fn subdivide_polyline(points: &[[f32; 2]], max_len: f32) -> Vec<[f32; 2]> {
    if points.len() < 2 {
        return points.to_vec();
    }
    let mut out = vec![points[0]];
    for w in points.windows(2) {
        let (a, b) = (w[0], w[1]);
        let len = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
        let steps = (len / max_len).ceil().max(1.0) as usize;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
        }
    }
    out
}

fn segment_distance_t(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> (f32, f32) {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ap = [p[0] - a[0], p[1] - a[1]];
    let len_sq = ab[0] * ab[0] + ab[1] * ab[1];
    let t = if len_sq > 1e-9 {
        ((ap[0] * ab[0] + ap[1] * ab[1]) / len_sq).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let closest = [a[0] + ab[0] * t, a[1] + ab[1] * t];
    let dist = ((p[0] - closest[0]).powi(2) + (p[1] - closest[1]).powi(2)).sqrt();
    (dist, t)
}

pub fn bake_open_sdf(
    centerline: &[[f32; 2]],
    thickness: f32,
    bounds: Rect,
    w: usize,
    h: usize,
) -> (Vec<[f32; 2]>, f32) {
    let centerline = &subdivide_polyline(centerline, (thickness * 0.25).max(1.0));
    let mut cumulative = vec![0.0f32; centerline.len()];
    for i in 1..centerline.len() {
        let d = ((centerline[i][0] - centerline[i - 1][0]).powi(2)
            + (centerline[i][1] - centerline[i - 1][1]).powi(2))
        .sqrt();
        cumulative[i] = cumulative[i - 1] + d;
    }
    let total_len = cumulative.last().copied().unwrap_or(1.0).max(1e-6);
    let half_width = thickness * 0.5;

    let mut field = vec![[0.0f32; 2]; w * h];
    for y in 0..h {
        for x in 0..w {
            let px = bounds.x0 as f32 + (x as f32 + 0.5) / w as f32 * bounds.width() as f32;
            let py = bounds.y0 as f32 + (y as f32 + 0.5) / h as f32 * bounds.height() as f32;

            let mut best = (f32::MAX, 0.0, 0usize);
            for i in 0..centerline.len() - 1 {
                let (d, t) = segment_distance_t([px, py], centerline[i], centerline[i + 1]);
                if d < best.0 {
                    best = (d, t, i);
                }
            }
            let (perp_dist, t, seg_i) = best;
            let seg_len = ((centerline[seg_i + 1][0] - centerline[seg_i][0]).powi(2)
                + (centerline[seg_i + 1][1] - centerline[seg_i][1]).powi(2))
            .sqrt();
            let length_param = (cumulative[seg_i] + t * seg_len) / total_len;

            field[y * w + x] = [perp_dist - half_width, length_param];
        }
    }
    (field, total_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> BezPath {
        let mut p = BezPath::new();
        p.move_to((0.0, 0.0));
        p.line_to((10.0, 0.0));
        p.line_to((10.0, 10.0));
        p.line_to((0.0, 10.0));
        p.close_path();
        p
    }

    #[test]
    fn bake_sdf_test() {
        let path = square();
        let bounds = path.bounding_box();
        let field = bake_sdf(&path, bounds, 32, 32);
        let center = field[16 * 32 + 16];
        assert!((center + 5.0).abs() < 0.5);
    }
}
