use crate::ui::Ui;
use accesskit::Node;
use accesskit::{NodeId, Role};
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Accessible {
    fn accessibility_id(&self) -> NodeId;
    fn accessibility_role(&self) -> Role;
    fn accessibility_label(&self) -> Option<String> {
        None
    }
    fn accessibility_state(&self, _node: &mut Node) {}
}

pub fn hash_id(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusId(u64);

impl FocusId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    pub fn new(id: &str) -> Self {
        FocusId(hash_id(id))
    }
}

pub trait Widget {
    type Output;
    fn ui(&mut self, ui: &mut Ui) -> Self::Output;
    fn on_start(&mut self, _ui: &mut Ui) {}
}

pub trait Measurable: Widget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> Self::Output;
}

pub trait AnyWidget {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2];
    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui);
    fn ui(&mut self, ui: &mut Ui);
    fn as_any_mut(&mut self) -> Option<&mut dyn Any>;
}

impl<T: Measurable> AnyWidget for &mut T {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        Measurable::measure(*self, ui)
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let _ = Measurable::arrange(*self, position, size, ui);
    }

    fn ui(&mut self, ui: &mut Ui) {
        Widget::ui(*self, ui);
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        None
    }
}

pub trait WidgetStyle: Clone {
    fn default_style() -> Self;
}

pub trait StatefulWidget {
    type State: Default + 'static;
    fn state_id(&self) -> &str;
    fn initial_state(&self) -> Self::State {
        Self::State::default()
    }
    fn state<'a>(&self, ui: &'a mut Ui) -> &'a mut Self::State {
        ui.widget_state_or(self.state_id(), self.initial_state())
    }
}
