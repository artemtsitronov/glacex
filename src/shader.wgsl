struct WindowSize {
    width: f32,
    height: f32,
}

@group(0) @binding(0)
var<uniform> window_size: WindowSize;

struct QuadVertex {
    @location(0) local_position: vec2<f32>,
}

struct RectInstance {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) corner_radius: f32,
    @location(5) border_width: f32,
    @location(6) border_color: vec4<f32>,
    @location(7) blur_radius: f32,
    @location(8) sharp: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) half_size: vec2<f32>,
    @location(3) corner_radius: f32,
    @location(4) blur_radius: f32,
    @location(5) border_width: f32,
    @location(6) border_color: vec4<f32>,
    @location(7) sharp: f32,
}

// Must be >= AA_PADDING below, or the fade band extends past the padded
// geometry and gets clipped again — same failure mode padding exists to fix.
const AA_FADE_WIDTH: f32 = 1.5;

// Expands rasterized quad geometry beyond the shape's true bounds so every
// point on the boundary — including cardinal tangent points on a circle —
// has real pixels beyond dist=0 for the AA fade to blend into.
const AA_PADDING: f32 = 4.0;

fn sd_rounded_box(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - radius;
}

@vertex
fn vs_main(vertex: QuadVertex, instance: RectInstance) -> VertexOutput {
    var out: VertexOutput;

    let padding = max(AA_PADDING, max(instance.blur_radius * 2.0, instance.border_width + AA_PADDING));
    let padded_size = instance.size + vec2<f32>(padding * 2.0);
    let padded_local = vertex.local_position * padded_size - vec2<f32>(padding);
    let pixel_position = instance.position + padded_local;

    let ndc_x = (pixel_position.x / window_size.width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pixel_position.y / window_size.height) * 2.0;
    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);

    out.color = instance.color;
    // local_pos/half_size reference the TRUE, unpadded size — the shape's
    // visible boundary doesn't move, only the raster margin around it grows.
    out.local_pos = padded_local - instance.size * 0.5;
    out.half_size = instance.size * 0.5;
    out.corner_radius = instance.corner_radius;
    out.blur_radius = instance.blur_radius;
    out.border_width = instance.border_width;
    out.border_color = instance.border_color;
    out.sharp = instance.sharp;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.blur_radius > 0.0 {
        let dist = sd_rounded_box(in.local_pos, in.half_size, in.corner_radius);
        let alpha = 1.0 - smoothstep(-in.blur_radius, in.blur_radius, dist);
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    }

    let inner_dist = sd_rounded_box(in.local_pos, in.half_size, in.corner_radius);
    let outer_dist = sd_rounded_box(
        in.local_pos,
        in.half_size + vec2<f32>(in.border_width),
        in.corner_radius + in.border_width,
    );

    let fill_alpha = 1.0 - smoothstep(0.0, AA_FADE_WIDTH, inner_dist);
    let color = mix(in.border_color, in.color, fill_alpha);

    let alpha = select(
        1.0 - smoothstep(0.0, AA_FADE_WIDTH, outer_dist), // normal AA
        select(1.0, 0.0, outer_dist > 0.0),                // sharp: hard cutoff, no fade
        in.sharp > 0.5
    );
    return vec4<f32>(color.rgb, color.a * alpha);
}
