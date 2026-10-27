use crate::Color;

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
    Mesh { corners: [Color; 4] }, // order: [top right] [top left] [bottom left] [bottom right]
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
    // yeah, I'm sorry, but how do you expect me to come up with a joke, for the `ImageHandle` name!?
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

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Fill::Solid(color)
    }
}

impl Fill {
    pub fn lerp(&self, other: &Fill, t: f32) -> Fill {
        match (self, other) {
            (Fill::Solid(a), Fill::Solid(b)) => Fill::Solid(a.lerp(*b, t)),
            _ if t >= 0.5 => other.clone(),
            _ => self.clone(),
        }
    }

    pub fn fade(&self, alpha: f32) -> Fill {
        match self {
            Fill::Solid(color) => Fill::Solid(color.with_alpha(color.a * alpha)),
            Fill::Gradient(gradient) => {
                let mut g = gradient.clone();
                for stop in &mut g.stops {
                    stop.color = stop.color.with_alpha(stop.color.a * alpha);
                }
                Fill::Gradient(g)
            }
            Fill::Image(image) => Fill::Image(*image),
        }
    }

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
