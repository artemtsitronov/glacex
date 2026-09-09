#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ease {
    Linear,
    EaseOutQuad,
    EaseInOutQuad,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutQuart,
    EaseOutExpo,
    EaseOutBack,
}

impl Ease {
    #[inline]
    pub fn sample(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Ease::Linear => t,
            Ease::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Ease::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Ease::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Ease::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Ease::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            Ease::EaseOutExpo => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - 2.0f32.powf(-10.0 * t)
                }
            }
            Ease::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
        }
    }
}

pub struct Motion;

impl Motion {
    pub const MICRO: f32 = 0.016;
    pub const INSTANT: f32 = 0.030;
    pub const SNAPPY: f32 = 0.045;
    pub const FLUID: f32 = 0.060;
    pub const GENTLE: f32 = 0.090;

    pub fn standard_spring() -> Spring {
        Spring {
            stiffness: 400.0,
            damping: 25.0,
            ..Default::default()
        }
    }

    pub fn snappy_spring() -> Spring {
        Spring {
            stiffness: 450.0,
            damping: 32.0,
            ..Default::default()
        }
    }

    pub fn fluid_spring() -> Spring {
        Spring {
            stiffness: 300.0,
            damping: 26.0,
            ..Default::default()
        }
    }
}

#[inline]
pub fn animate_towards(current: f32, target: f32, dt: f32, half_life: f32) -> f32 {
    if (current - target).abs() < 1e-4 {
        return target;
    }
    let decay = (-dt * (std::f32::consts::LN_2 / half_life.max(1e-4))).exp();
    target + (current - target) * decay
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Spring {
            value: 0.0,
            velocity: 0.0,
            target: 0.0,
            stiffness: 320.0,
            damping: 26.0,
        }
    }
}

impl Spring {
    pub fn new(initial: f32) -> Self {
        Spring {
            value: initial,
            target: initial,
            ..Default::default()
        }
    }

    pub fn with_physics(initial: f32, stiffness: f32, damping: f32) -> Self {
        Spring {
            value: initial,
            target: initial,
            stiffness,
            damping,
            velocity: 0.0,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.min(0.05);
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = spring_force + damping_force;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        if displacement.abs() < 0.0005 && self.velocity.abs() < 0.0005 {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }

    #[inline]
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < 0.0005 && self.velocity.abs() < 0.0005
    }
}
