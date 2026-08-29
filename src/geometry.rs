pub fn contains(position: [f32; 2], size: [f32; 2], corner_radius: f32, point: [f32; 2]) -> bool {
    let half_size = [size[0] * 0.5, size[1] * 0.5];
    let center = [position[0] + half_size[0], position[1] + half_size[1]];
    let p = [(point[0] - center[0]).abs(), (point[1] - center[1]).abs()];
    let q = [
        p[0] - half_size[0] + corner_radius,
        p[1] - half_size[1] + corner_radius,
    ];
    let dist = q[0].max(q[1]).min(0.0) + (q[0].max(0.0).powi(2) + q[1].max(0.0).powi(2)).sqrt()
        - corner_radius;
    dist <= 0.0
}

pub fn center_text_in(
    position: [f32; 2],
    size: [f32; 2],
    text_width: f32,
    line_height: f32,
) -> [f32; 2] {
    [
        position[0] + (size[0] - text_width) / 2.0,
        position[1] + (size[1] - line_height) / 2.0,
    ]
}
