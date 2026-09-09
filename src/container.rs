use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, IntoId, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

pub struct Container<'a> {
    id: Option<String>,
    width: f32,
    height: f32,
    padding: [f32; 2],
    child: Box<dyn AnyWidget + 'a>,
}

impl<'a> Container<'a> {
    pub const DEFAULT_WIDTH: f32 = 100.0;
    pub const DEFAULT_HEIGHT: f32 = 100.0;

    pub fn new(child: &'a mut impl Measurable) -> Self {
        Container {
            id: None,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            padding: [0.0, 0.0],
            child: Box::new(child),
        }
    }

    pub fn id(mut self, id: impl IntoId) -> Self {
        self.id = id.into_id();
        self
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = size[0];
        self.height = size[1];
        self
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = self.measure(ui);
        Measurable::arrange(self, position, size, ui);
    }
}

impl<'a> Widget for Container<'a> {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        self.arrange_at([0.0, 0.0], ui);
    }
}

impl<'a> Measurable for Container<'a> {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        // Same reasoning as `Card`/`Divider`: an un-identified container has
        // no unique identity, so every anonymous one in the frame would
        // collide under the shared "container" fallback id.
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

        let child_position = [position[0] + self.padding[0], position[1] + self.padding[1]];
        let child_size = [
            (size[0] - self.padding[0] * 2.0).max(0.0),
            (size[1] - self.padding[1] * 2.0).max(0.0),
        ];
        self.child.arrange(child_position, child_size, ui);
    }
}

impl<'a> Accessible for Container<'a> {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(self.id.as_deref().unwrap_or("container")))
    }
    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }
}
