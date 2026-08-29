use crate::ui::Ui;
use crate::widget::{AnyWidget, Measurable, Widget};

pub struct Container<'a> {
    size: [f32; 2],
    child: Box<dyn AnyWidget + 'a>,
}

impl<'a> Container<'a> {
    pub fn new(size: [f32; 2], child: &'a mut impl Measurable) -> Self {
        Container {
            size,
            child: Box::new(child),
        }
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        Measurable::arrange(self, position, self.size, ui);
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
        self.size // explicit, not derived from the child at all
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        // child is drawn at this container's position/size, full stop —
        // no measure-and-center logic like Column/Row do; the child
        // simply gets exactly this box.
        self.child.arrange(position, size, ui);
    }
}
