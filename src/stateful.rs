use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::Path;
use crate::scrolling::ScrollConfig;
use crate::shadow::ShadowStyle;
use crate::text::TextStyle;
use winit::window::CursorIcon;

pub trait Blend: Clone {
    fn blend(&self, other: &Self, t: f32) -> Self;
}

fn snap<T: Clone>(a: &T, b: &T, t: f32) -> T {
    if t >= 0.5 { b.clone() } else { a.clone() }
}

impl Blend for f32 {
    fn blend(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Blend for [f32; 2] {
    fn blend(&self, other: &Self, t: f32) -> Self {
        [self[0].blend(&other[0], t), self[1].blend(&other[1], t)]
    }
}

impl Blend for Color {
    fn blend(&self, other: &Self, t: f32) -> Self {
        let from = if self.a <= 0.0 {
            other.with_alpha(0.0)
        } else {
            *self
        };
        let to = if other.a <= 0.0 {
            self.with_alpha(0.0)
        } else {
            *other
        };
        from.lerp(to, t)
    }
}

impl Blend for Fill {
    fn blend(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Fill::Solid(a), Fill::Solid(b)) => Fill::Solid(a.blend(b, t)),
            _ => snap(self, other, t),
        }
    }
}

impl Blend for ShadowStyle {
    fn blend(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}

impl Blend for Option<ShadowStyle> {
    fn blend(&self, other: &Self, t: f32) -> Self {
        ShadowStyle::lerp_opt(*self, *other, t)
    }
}

impl Blend for Option<Fill> {
    fn blend(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.blend(b, t)),
            (Some(a), None) if t < 1.0 => Some(a.fade(1.0 - t)),
            (None, Some(b)) if t > 0.0 => Some(b.fade(t)),
            _ => snap(self, other, t),
        }
    }
}

impl Blend for Option<Color> {
    fn blend(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.blend(b, t)),
            _ => snap(self, other, t),
        }
    }
}

impl Blend for Option<[f32; 2]> {
    fn blend(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.blend(b, t)),
            _ => snap(self, other, t),
        }
    }
}

impl Blend for TextStyle {
    fn blend(&self, other: &Self, t: f32) -> Self {
        let discrete = snap(self, other, t);
        TextStyle {
            color: self.color.blend(&other.color, t),
            font_size: self.font_size.blend(&other.font_size, t),
            line_height: self.line_height.blend(&other.line_height, t),
            ..discrete
        }
    }
}

impl Blend for CursorIcon {
    fn blend(&self, other: &Self, t: f32) -> Self {
        snap(self, other, t)
    }
}

impl Blend for Option<CursorIcon> {
    fn blend(&self, other: &Self, t: f32) -> Self {
        snap(self, other, t)
    }
}

impl Blend for ScrollConfig {
    fn blend(&self, other: &Self, t: f32) -> Self {
        ScrollConfig {
            padding: self.padding.blend(&other.padding, t),
            thickness: self.thickness.blend(&other.thickness, t),
            min_thumb: self.min_thumb.blend(&other.min_thumb, t),
            linger_seconds: self.linger_seconds.blend(&other.linger_seconds, t),
        }
    }
}

impl Blend for Path {
    fn blend(&self, other: &Self, t: f32) -> Self {
        snap(self, other, t)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct StateWeights {
    pub hovered: f32,
    pub pressed: f32,
    pub focused: f32,
    pub selected: f32,
}

impl StateWeights {
    pub const IDLE: StateWeights = StateWeights {
        hovered: 0.0,
        pressed: 0.0,
        focused: 0.0,
        selected: 0.0,
    };

    pub fn when(on: bool) -> f32 {
        if on { 1.0 } else { 0.0 }
    }

    pub fn hovered(mut self, t: f32) -> Self {
        self.hovered = t;
        self
    }

    pub fn pressed(mut self, t: f32) -> Self {
        self.pressed = t;
        self
    }

    pub fn focused(mut self, t: f32) -> Self {
        self.focused = t;
        self
    }

    pub fn selected(mut self, t: f32) -> Self {
        self.selected = t;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stateful<T> {
    pub idle: T,
    pub hovered: Option<T>,
    pub pressed: Option<T>,
    pub focused: Option<T>,
    pub selected: Option<T>,
    pub selected_hovered: Option<T>,
}

impl<T> Stateful<T> {
    pub fn new(idle: T) -> Self {
        Stateful {
            idle,
            hovered: None,
            pressed: None,
            focused: None,
            selected: None,
            selected_hovered: None,
        }
    }

    pub fn hovered(mut self, value: T) -> Self {
        self.hovered = Some(value);
        self
    }

    pub fn pressed(mut self, value: T) -> Self {
        self.pressed = Some(value);
        self
    }

    pub fn focused(mut self, value: T) -> Self {
        self.focused = Some(value);
        self
    }

    pub fn selected(mut self, value: T) -> Self {
        self.selected = Some(value);
        self
    }

    pub fn selected_hovered(mut self, value: T) -> Self {
        self.selected_hovered = Some(value);
        self
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Stateful<U> {
        Stateful {
            idle: f(&self.idle),
            hovered: self.hovered.as_ref().map(&f),
            pressed: self.pressed.as_ref().map(&f),
            focused: self.focused.as_ref().map(&f),
            selected: self.selected.as_ref().map(&f),
            selected_hovered: self.selected_hovered.as_ref().map(&f),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.idle).chain(
            [
                &self.hovered,
                &self.pressed,
                &self.focused,
                &self.selected,
                &self.selected_hovered,
            ]
            .into_iter()
            .flatten(),
        )
    }
}

impl<T: Blend> Stateful<T> {
    pub fn resolve(&self, weights: StateWeights) -> T {
        let mut value = layer(&self.idle, self.hovered.as_ref(), weights.hovered);
        if let Some(selected) = &self.selected {
            let on = layer(selected, self.selected_hovered.as_ref(), weights.hovered);
            value = mix(value, &on, weights.selected);
        }
        if let Some(focused) = &self.focused {
            value = mix(value, focused, weights.focused);
        }
        if let Some(pressed) = &self.pressed {
            value = mix(value, pressed, weights.pressed);
        }
        value
    }
}

fn layer<T: Blend>(base: &T, over: Option<&T>, t: f32) -> T {
    match over {
        Some(over) if t > 0.0 => base.blend(over, t),
        _ => base.clone(),
    }
}

fn mix<T: Blend>(base: T, over: &T, t: f32) -> T {
    if t > 0.0 { base.blend(over, t) } else { base }
}

impl<T: Default> Default for Stateful<T> {
    fn default() -> Self {
        Stateful::new(T::default())
    }
}

impl<T> From<T> for Stateful<T> {
    fn from(value: T) -> Self {
        Stateful::new(value)
    }
}

impl From<Color> for Stateful<Fill> {
    fn from(color: Color) -> Self {
        Stateful::new(Fill::Solid(color))
    }
}

impl From<Stateful<Color>> for Stateful<Fill> {
    fn from(colors: Stateful<Color>) -> Self {
        colors.map(|c| Fill::Solid(*c))
    }
}

impl From<Fill> for Stateful<Option<Fill>> {
    fn from(fill: Fill) -> Self {
        Stateful::new(Some(fill))
    }
}

impl From<Color> for Stateful<Option<Fill>> {
    fn from(color: Color) -> Self {
        Stateful::new(Some(Fill::Solid(color)))
    }
}

impl From<Stateful<Color>> for Stateful<Option<Fill>> {
    fn from(colors: Stateful<Color>) -> Self {
        colors.map(|c| Some(Fill::Solid(*c)))
    }
}

impl From<Stateful<Fill>> for Stateful<Option<Fill>> {
    fn from(fills: Stateful<Fill>) -> Self {
        fills.map(|f| Some(f.clone()))
    }
}

impl From<CursorIcon> for Stateful<Option<CursorIcon>> {
    fn from(cursor: CursorIcon) -> Self {
        Stateful::new(Some(cursor))
    }
}

impl From<Color> for Stateful<Option<Color>> {
    fn from(color: Color) -> Self {
        Stateful::new(Some(color))
    }
}

impl From<Stateful<Color>> for Stateful<Option<Color>> {
    fn from(colors: Stateful<Color>) -> Self {
        colors.map(|c| Some(*c))
    }
}

impl From<ShadowStyle> for Stateful<Option<ShadowStyle>> {
    fn from(shadow: ShadowStyle) -> Self {
        Stateful::new(Some(shadow))
    }
}

impl From<Stateful<ShadowStyle>> for Stateful<Option<ShadowStyle>> {
    fn from(shadows: Stateful<ShadowStyle>) -> Self {
        shadows.map(|s| Some(*s))
    }
}

#[derive(Clone, Default)]
pub struct StyleOverrides {
    pub fill: Option<Stateful<Fill>>,
    pub border_color: Option<Stateful<Color>>,
    pub border_width: Option<Stateful<f32>>,
    pub shadow: Option<Stateful<Option<ShadowStyle>>>,
}

impl StyleOverrides {
    pub fn apply(
        &self,
        fill: &mut Stateful<Fill>,
        border_color: &mut Stateful<Color>,
        border_width: &mut Stateful<f32>,
        shadow: &mut Stateful<Option<ShadowStyle>>,
    ) {
        if let Some(v) = &self.fill {
            *fill = v.clone();
        }
        if let Some(v) = &self.border_color {
            *border_color = v.clone();
        }
        if let Some(v) = &self.border_width {
            *border_width = v.clone();
        }
        if let Some(v) = &self.shadow {
            *shadow = v.clone();
        }
    }
}

macro_rules! style_overrides {
    ($widget:ty) => {
        impl $widget {
            pub fn fill(
                mut self,
                fill: impl Into<$crate::stateful::Stateful<$crate::fill::Fill>>,
            ) -> Self {
                self.overrides.fill = Some(fill.into());
                self
            }

            pub fn border_color(
                mut self,
                color: impl Into<$crate::stateful::Stateful<$crate::color::Color>>,
            ) -> Self {
                self.overrides.border_color = Some(color.into());
                self
            }

            pub fn border_width(
                mut self,
                width: impl Into<$crate::stateful::Stateful<f32>>,
            ) -> Self {
                self.overrides.border_width = Some(width.into());
                self
            }

            pub fn shadow(
                mut self,
                shadow: impl Into<$crate::stateful::Stateful<Option<$crate::shadow::ShadowStyle>>>,
            ) -> Self {
                self.overrides.shadow = Some(shadow.into());
                self
            }
        }
    };
}

pub(crate) use style_overrides;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_states_fall_back_to_idle() {
        let value = Stateful::new(1.0f32);
        assert_eq!(
            value.resolve(StateWeights::IDLE.hovered(1.0).pressed(1.0)),
            1.0
        );
    }

    #[test]
    fn states_layer_in_order() {
        let value = Stateful::new(0.0f32).hovered(10.0).pressed(20.0);
        assert_eq!(value.resolve(StateWeights::IDLE.hovered(0.5)), 5.0);
        assert_eq!(
            value.resolve(StateWeights::IDLE.hovered(1.0).pressed(1.0)),
            20.0
        );
    }

    #[test]
    fn selected_hovered_falls_back_to_selected() {
        let value = Stateful::new(0.0f32).hovered(1.0).selected(5.0);
        let weights = StateWeights::IDLE.hovered(1.0).selected(1.0);
        assert_eq!(value.resolve(weights), 5.0);
        assert_eq!(value.selected_hovered(7.0).resolve(weights), 7.0);
    }

    #[test]
    fn color_converts_into_stateful_fill() {
        let fill: Stateful<Fill> = Stateful::new(Color::RED).hovered(Color::BLUE).into();
        assert!(matches!(fill.hovered, Some(Fill::Solid(c)) if c == Color::BLUE));
    }

    #[test]
    fn blending_from_transparent_keeps_target_hue() {
        let mid = Color::TRANSPARENT.blend(&Color::WHITE, 0.5);
        assert_eq!((mid.r, mid.a), (1.0, 0.5));
    }
}
