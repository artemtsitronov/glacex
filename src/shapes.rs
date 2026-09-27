use crate::color::Color;
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode, vertex_attr_array};

/// One corner of the unit quad every rectangle is stamped from. Never
/// changes, instancing reuses this same six-vertex geometry for every shape.
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
pub struct ShapeInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
    pub corner_radius: [f32; 4],
    pub border_color: Color,
    pub render_params: [f32; 4],
    pub shape_params: [f32; 4],
    pub gradient_center: [f32; 2],
    pub image_uv: [f32; 4],
    pub sdf_uv: [f32; 4],
    pub path_params: [f32; 4],
}

impl ShapeInstance {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            1 => Float32x2,
            2 => Float32x2,
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
            6 => Float32x4,
            7 => Float32x4,
            8 => Float32x2,
            9 => Float32x4,
            10 => Float32x4,
            11 => Float32x4,
        ],
    };
}
