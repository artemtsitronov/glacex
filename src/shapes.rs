use crate::color::Color;
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode, vertex_attr_array};

/// One corner of the unit quad every rectangle is stamped from. Never
/// changes — instancing reuses this same six-vertex geometry for every shape.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    local_position: [f32; 2],
}

impl QuadVertex {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![0 => Float32x2],
    };
}

pub const QUAD_VERTICES: [QuadVertex; 6] = [
    QuadVertex {
        local_position: [0.0, 0.0],
    },
    QuadVertex {
        local_position: [1.0, 0.0],
    },
    QuadVertex {
        local_position: [0.0, 1.0],
    },
    QuadVertex {
        local_position: [0.0, 1.0],
    },
    QuadVertex {
        local_position: [1.0, 0.0],
    },
    QuadVertex {
        local_position: [1.0, 1.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
    pub corner_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
    pub blur_radius: f32,
    pub sharp: f32,
    pub fill_kind: f32,
    pub gradient_angle: f32,
    pub gradient_row: f32,
    pub gradient_center: [f32; 2],
}

impl RectInstance {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        // Locations 1..4 — continuing on from QuadVertex's location 0, since
        // both buffers feed the same vertex shader at once.
        attributes: &vertex_attr_array![
            1 => Float32x2,
            2 => Float32x2,
            3 => Float32x4,
            4 => Float32,
            5 => Float32,
            6 => Float32x4,
            7 => Float32,
            8 => Float32,
            9 => Float32,
            10 => Float32,
            11 => Float32,
            12 => Float32x2,
        ],
    };
}
