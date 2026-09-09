use crate::color::Color;
use crate::fill::Fill;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, IntoId, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

pub struct Divider {
    id: Option<String>,
    orientation: DividerOrientation,
    length: f32,
    thickness: f32,
    color: Color,
    width: Option<f32>,
    height: Option<f32>,
}

impl Divider {
    pub fn horizontal(length: f32) -> Self {
        Divider {
            id: None,
            orientation: DividerOrientation::Horizontal,
            length,
            thickness: 1.0,
            color: Theme::BORDER,
            width: None,
            height: None,
        }
    }

    pub fn vertical(length: f32) -> Self {
        Divider {
            id: None,
            orientation: DividerOrientation::Vertical,
            length,
            thickness: 1.0,
            color: Theme::BORDER,
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl IntoId) -> Self {
        self.id = id.into_id();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn faint(mut self) -> Self {
        self.color = Theme::BORDER_FAINT;
        self
    }
}

impl Widget for Divider {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Divider {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        let natural = match self.orientation {
            DividerOrientation::Horizontal => [self.length, self.thickness],
            DividerOrientation::Vertical => [self.thickness, self.length],
        };
        [
            self.width.unwrap_or(natural[0]),
            self.height.unwrap_or(natural[1]),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        // Purely decorative dividers (the common case: no explicit `.id(..)`)
        // have no stable per-instance identity, so registering them would
        // collide with every other un-identified divider in the same frame.
        // Only expose one to accessibility tools once the caller opts in by
        // giving it an id (e.g. an actual resizable pane splitter).
        if self.id.is_some() {
            ui.register_accessible(
                self,
                [
                    position[0],
                    position[1],
                    position[0] + size[0],
                    position[1] + size[1],
                ],
            );
        }

        ui.draw_rect(
            position,
            size,
            Fill::Solid(self.color),
            0.0,
            0.0,
            Color::TRANSPARENT,
            0.0,
            true,
            0.0,
        );
    }
}

impl Accessible for Divider {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(self.id.as_deref().unwrap_or("divider")))
    }
    fn accessibility_role(&self) -> Role {
        Role::Splitter
    }
}
