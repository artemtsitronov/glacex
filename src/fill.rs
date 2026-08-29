use crate::Color;

#[derive(Debug, Clone, Copy)]
pub struct GradientStop {
    pub position: f32, // 0.0 - 1.0
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear { angle: f32 },
    Radial { center: [f32; 2], radius: f32 },
    Conic { center: [f32; 2] },
    Mesh { corners: [Color; 4] }, //top-right, top-left, bottom-left, bottom-right
}

#[derive(Debug, Clone, Copy)]
pub enum GradientHandle {
    Ramp { row: u32 },
    Mesh { tile_index: u32 },
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub kind: GradientKind,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone)]
pub enum Fill {
    Solid(Color),
    Gradient(Gradient),
}
