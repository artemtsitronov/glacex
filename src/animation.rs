//! Easing functions and animation utilities for smooth interactive transitions.

/// Standard animation curve presets matching modern design systems (Linear, Vercel, Apple).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ease {
    Linear,
    EaseOutQuad,
    EaseInOutQuad,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutExpo,
    EaseOutBack,
}

impl Ease {
    /// Evaluates the normalized progress `t` (`0.0..=1.0`) under this easing function.
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

/// Computes an animated step from `current` towards `target` using smooth exponential decay.
/// This approach is frame-rate independent: `dt` is elapsed seconds since previous frame,
/// and `half_life` controls responsiveness in seconds (smaller = faster response).
#[inline]
pub fn animate_towards(current: f32, target: f32, dt: f32, half_life: f32) -> f32 {
    if (current - target).abs() < 1e-4 {
        return target;
    }
    let decay = (-dt * (std::f32::consts::LN_2 / half_life.max(1e-4))).exp();
    target + (current - target) * decay
}

/// Linearly interpolates between `a` and `b` by factor `t`.
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Spring simulation state for physics-based spring transitions.
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
            stiffness: 280.0,
            damping: 24.0,
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

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Advances the spring simulation by `dt` seconds using semi-implicit Euler integration.
    pub fn update(&mut self, dt: f32) {
        let dt = dt.min(0.05); // cap step to prevent explosion on stalls
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = spring_force + damping_force;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        if displacement.abs() < 0.001 && self.velocity.abs() < 0.001 {
            self.value = self.target;
            self.velocity = 0.0;
        }
    }
}
