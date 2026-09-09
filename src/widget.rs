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

/// Accepted by every widget's `.id(..)` builder method, so callers can pass
/// a plain `&str`/`String` (the common case), an already-`Option`al id, or
/// `None` to explicitly clear one — without forcing everyone through
/// `Some("...".to_string())`. `Into<Option<String>>` can't cover `&str`
/// directly (no `From<&str> for Option<String>` in std, and orphan rules
/// block adding one), hence this local trait.
pub trait IntoId {
    fn into_id(self) -> Option<String>;
}

impl IntoId for Option<String> {
    fn into_id(self) -> Option<String> {
        self
    }
}

impl IntoId for String {
    fn into_id(self) -> Option<String> {
        Some(self)
    }
}

impl IntoId for &str {
    fn into_id(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl IntoId for Option<&str> {
    fn into_id(self) -> Option<String> {
        self.map(|s| s.to_string())
    }
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
    fn take_state(&self, ui: &mut Ui) -> Self::State {
        ui.take_widget_state_or(self.state_id(), self.initial_state())
    }
    fn put_state(&self, ui: &mut Ui, state: Self::State) {
        ui.put_widget_state(self.state_id(), state)
    }
}
