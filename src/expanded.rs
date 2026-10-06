use crate::ui::Ui;
use crate::widget::{AnyWidget, Measurable, Widget};

pub struct Expanded<'a> {
    flex: f32,
    child: Box<dyn AnyWidget + 'a>,
}

impl<'a> Expanded<'a> {
    pub fn new(child: &'a mut impl Measurable) -> Self {
        Expanded {
            flex: 1.0,
            child: Box::new(child),
        }
    }

    pub fn flex(mut self, flex: f32) -> Self {
        self.flex = flex.max(0.0);
        self
    }
}

impl<'a> Widget for Expanded<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl<'a> Measurable for Expanded<'a> {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        self.child.measure(ui)
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        self.child.arrange(position, size, ui);
    }

    fn flex(&self) -> f32 {
        self.flex
    }
}
