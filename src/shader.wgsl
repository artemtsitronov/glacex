struct WindowSize {
    width: f32,
    height: f32,
}

@group(0) @binding(0)
var<uniform> window_size: WindowSize;

@group(1) @binding(0)
var gradient_atlas: texture_2d<f32>;
@group(1) @binding(1)
var gradient_sampler: sampler;
@group(2) @binding(0)
var image_atlas: texture_2d<f32>;
@group(2) @binding(1)
var image_sampler: sampler;

struct QuadVertex {
    @location(0) local_position: vec2<f32>,
}

struct ShapeInstance {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) corner_radius: vec4<f32>,
    @location(5) border_width: f32,
    @location(6) border_color: vec4<f32>,
    @location(7) blur_radius: f32,
    @location(8) sharp: f32,
    @location(9) fill_kind: f32,
    @location(10) shape_kind: f32,
    @location(11) gradient_angle: f32,
    @location(12) gradient_row: f32,
    @location(13) gradient_center: vec2<f32>,
    @location(14) rotation: f32,
    @location(15) image_uv: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) half_size: vec2<f32>,
    @location(3) corner_radius: vec4<f32>,
    @location(4) blur_radius: f32,
    @location(5) border_width: f32,
    @location(6) border_color: vec4<f32>,
    @location(7) sharp: f32,
    @location(8) fill_kind: f32,
    @location(9) shape_kind: f32,
    @location(10) gradient_angle: f32,
    @location(11) gradient_row: f32,
    @location(12) gradient_center: vec2<f32>,
    @location(13) rotation: f32,
    @location(14) image_uv: vec4<f32>,
}

// Must be >= AA_PADDING below, or the fade band extends past the padded
// geometry and gets clipped again
const AA_FADE_WIDTH: f32 = 1.5;

// Expands rasterized quad geometry beyond the shape's true bounds so every
// point on the boundary
const AA_PADDING: f32 = 4.0;

fn sd_rounded_box(p: vec2<f32>, half_size: vec2<f32>, radii: vec4<f32>) -> f32 {
    let top_radius = select(radii.y, radii.x, p.x < 0.0);
    let bottom_radius = select(radii.z, radii.w, p.x < 0.0);
    let corner_radius = select(bottom_radius, top_radius, p.y < 0.0);
    let r = min(corner_radius, min(half_size.x, half_size.y));
    let q = abs(p) - half_size + vec2<f32>(r);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

fn sd_ellipse(p: vec2<f32>, half_size: vec2<f32>) -> f32 {
    let k0 = length(p / half_size);
    let k1 = length(p / (half_size * half_size));
    return k0 * (k0 - 1.0) / k1;
}

fn shape_distance(p: vec2<f32>, half_size: vec2<f32>, radii: vec4<f32>, shape_kind: f32) -> f32 {
    if shape_kind < 0.5 {
        return sd_rounded_box(p, half_size, radii);
    }
    return sd_ellipse(p, half_size);
}

fn rotate(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return vec2<f32>(p.x * c - p.y * s, p.x * s + p.y * c);
}

@vertex
fn vs_main(vertex: QuadVertex, instance: ShapeInstance) -> VertexOutput {
    var out: VertexOutput;

    let diagonal = length(instance.size * 0.5);
let rotation_padding = select(0.0, diagonal - max(instance.size.x, instance.size.y) * 0.5, instance.rotation != 0.0);
let padding = max(AA_PADDING, max(instance.blur_radius * 2.0, max(instance.border_width + AA_PADDING, rotation_padding + AA_PADDING)));
    let padded_size = instance.size + vec2<f32>(padding * 2.0);
    let padded_local = vertex.local_position * padded_size - vec2<f32>(padding);
    let pixel_position = instance.position + padded_local;

    let ndc_x = (pixel_position.x / window_size.width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pixel_position.y / window_size.height) * 2.0;
    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);

    out.color = instance.color;
    out.local_pos = padded_local - instance.size * 0.5;
    out.half_size = instance.size * 0.5;
    out.corner_radius = instance.corner_radius;
    out.blur_radius = instance.blur_radius;
    out.border_width = instance.border_width;
    out.border_color = instance.border_color;
    out.sharp = instance.sharp;
    out.shape_kind = instance.shape_kind;
    out.fill_kind = instance.fill_kind;
    out.gradient_angle = instance.gradient_angle;
    out.gradient_row = instance.gradient_row;
    out.gradient_center = instance.gradient_center;
    out.rotation = instance.rotation;
    out.image_uv = instance.image_uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.blur_radius > 0.0 {
        let dist = shape_distance(in.local_pos, in.half_size, in.corner_radius, in.shape_kind);
        let alpha = 1.0 - smoothstep(-in.blur_radius, in.blur_radius, dist);
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    }

    var t: f32 = 0.0;
    if in.fill_kind == 1.0 { // linear
        let angle_rad = radians(in.gradient_angle);
        let direction = vec2<f32>(cos(angle_rad), sin(angle_rad));
        let projected = dot(in.local_pos, direction);
        t = (projected + in.half_size.x) / (in.half_size.x * 2.0);
    } else if in.fill_kind == 2.0 { // radial
        let dist = length(in.local_pos - in.gradient_center);
        t = dist / in.gradient_angle; // gradient_angle holds radius here
    } else if in.fill_kind == 3.0 { // conic
        let angle = atan2(in.local_pos.y - in.gradient_center.y, in.local_pos.x - in.gradient_center.x);
        t = (angle + 3.14159265) / (2.0 * 3.14159265);
    }

    var fill_color = in.color;
    if in.fill_kind >= 1.0 && in.fill_kind <= 3.0 {
        let row_count = 64.0;
        let v = (in.gradient_row + 0.5) / row_count;
        fill_color = textureSample(gradient_atlas, gradient_sampler, vec2<f32>(clamp(t, 0.0, 1.0), v));
    }

    if in.fill_kind == 5.0 { // image
        let local_uv = (in.local_pos + in.half_size) / (in.half_size * 2.0);
        let uv = mix(in.image_uv.xy, in.image_uv.zw, local_uv);
        fill_color = textureSample(image_atlas, image_sampler, uv);
    }

    let rotated_pos = rotate(in.local_pos, -in.rotation);

    let inner_dist = shape_distance(rotated_pos, in.half_size, in.corner_radius, in.shape_kind);
    let outer_dist = shape_distance(
        rotated_pos,
        in.half_size + vec2<f32>(in.border_width),
        in.corner_radius + vec4<f32>(in.border_width),
        in.shape_kind,
    );

    let fill_alpha = 1.0 - smoothstep(0.0, AA_FADE_WIDTH, inner_dist);
    let color = mix(in.border_color, fill_color, fill_alpha);

    let alpha = select(
        1.0 - smoothstep(0.0, AA_FADE_WIDTH, outer_dist),
        select(1.0, 0.0, outer_dist > 0.0),
        in.sharp > 0.5
    );
    return vec4<f32>(color.rgb, color.a * alpha);
}
