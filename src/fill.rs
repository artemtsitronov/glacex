use crate::Color;

#[macro_export]
macro_rules! linear_gradient {
    ( $( ($r:expr, $g:expr, $b:expr, $a:expr) ),+ ) => {
        let colors = vec![ $( Color::rgba($r, $g, $b, $a) ),+ ];
        let count = colors.len();

        Gradient {
            kind: GradientKind::Linear { angle: 0.0 },
            stops: colors.into_iter().enumerate().map(|(i, color)| {
                let position = if count == 1 { 0.0 } else { i as f32 / (count - 1) as f32 };
                GradientStop {
                    position,
                    color,
                }
            }).collect(),
        }
    };
}

#[macro_export]
macro_rules! radial_gradient {
    ( $( ($r:expr, $g:expr, $b:expr, $a:expr) ),+ ) => {
        let colors = vec![ $( Color::rgba($r, $g, $b, $a) ),+ ];
        let count = colors.len();

        Gradient {
            kind: GradientKind::Radial { angle: 0.0 },
            stops: colors.into_iter().enumerate().map(|(i, color)| {
                let position = if count == 1 { 0.0 } else { i as f32 / (count - 1) as f32 };
                GradientStop {
                    position,
                    color,
                }
            }).collect(),
        }
    };
}

#[macro_export]
macro_rules! conic_gradient {
    ( $( ($r:expr, $g:expr, $b:expr, $a:expr) ),+ ) => {
        let colors = vec![ $( Color::rgba($r, $g, $b, $a) ),+ ];
        let count = colors.len();

        Gradient {
            kind: GradientKind::Conic { angle: 0.0 },
            stops: colors.into_iter().enumerate().map(|(i, color)| {
                let position = if count == 1 { 0.0 } else { i as f32 / (count - 1) as f32 };
                GradientStop {
                    position,
                    color,
                }
            }).collect(),
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear { angle: f32 },
    Radial { center: [f32; 2], radius: f32 },
    Conic { center: [f32; 2] },
    Mesh { corners: [Color; 4] }, // order top right top left bottom left bottom right
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageHandle {
    pub(crate) atlas_uv: [f32; 4],
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl ImageHandle {
    pub fn width(&self) -> f32 {
        self.width as f32
    }
    pub fn height(&self) -> f32 {
        self.height as f32
    }
    pub fn size(&self) -> [f32; 2] {
        [self.width as f32, self.height as f32]
    }
    pub fn scale(&self, factor: f32) -> [f32; 2] {
        [self.width as f32 * factor, self.height as f32 * factor]
    }
}

#[derive(Debug, Clone)]
pub enum Fill {
    Solid(Color),
    Gradient(Gradient),
    Image(ImageHandle),
}

impl Fill {
    pub fn darken(&self, amount: f32) -> Fill {
        match self {
            Fill::Solid(color) => Fill::Solid(color.darken(amount)),
            Fill::Gradient(gradient) => {
                let mut g = gradient.clone();
                for stop in &mut g.stops {
                    stop.color = stop.color.darken(amount);
                }
                Fill::Gradient(g)
            }
            Fill::Image(image) => Fill::Image(*image),
        }
    }

    pub fn lighten(&self, amount: f32) -> Fill {
        match self {
            Fill::Solid(color) => Fill::Solid(color.lighten(amount)),
            Fill::Gradient(gradient) => {
                let mut g = gradient.clone();
                for stop in &mut g.stops {
                    stop.color = stop.color.lighten(amount);
                }
                Fill::Gradient(g)
            }
            Fill::Image(image) => Fill::Image(*image),
        }
    }
}
