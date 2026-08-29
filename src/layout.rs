use crate::alignment::{Alignment, to_taffy_align};
use crate::ui::Ui;
use crate::widget::{AnyWidget, Measurable, Widget};
use taffy::prelude::*;

#[macro_export]
macro_rules! column {
    ($($widget:expr),* $(,)?) => {
        $crate::Column::new(vec![$(Box::new($widget)),*])
    };
}

#[macro_export]
macro_rules! row {
    ($($widget:expr),* $(,)?) => {
        $crate::Row::new(vec![$(Box::new($widget)),*])
    };
}

/// A vertical stack of widgets. Holds no position of its own — `arrange_at`
/// is handed where it lives, every frame, by whoever is arranging it.
/// Delegates the actual placement math to taffy: `measure()` measures
/// every child exactly once per frame and caches the results;
/// `arrange()` builds the taffy tree from that cache (no re-measuring)
/// and reads computed positions back out.
pub struct Column<'a> {
    spacing: f32,
    align: Alignment,
    children: Vec<Box<dyn AnyWidget + 'a>>,
    cached_child_sizes: Vec<[f32; 2]>,
}

impl<'a> Column<'a> {
    pub fn new(children: Vec<Box<dyn AnyWidget + 'a>>) -> Self {
        let cached_child_sizes = vec![[0.0; 2]; children.len()];
        Column {
            spacing: 8.0,
            align: Alignment::Center,
            children,
            cached_child_sizes,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.children
            .get_mut(index)?
            .as_any_mut()?
            .downcast_mut::<T>()
    }

    /// Measure then arrange in one call, at the given origin. The only
    /// entry point that should ever be called from outside — measure()
    /// and arrange() individually assume they're called in that order,
    /// within the same frame, which this guarantees.
    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = Measurable::measure(self, ui);
        Measurable::arrange(self, position, size, ui);
    }
}

impl<'a> Widget for Column<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        self.arrange_at([0.0, 0.0], ui);
    }
}

impl<'a> Measurable for Column<'a> {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        self.cached_child_sizes.clear();
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let count = self.children.len();
        for (i, child) in self.children.iter_mut().enumerate() {
            let child_size = child.measure(ui);
            self.cached_child_sizes.push(child_size);
            width = width.max(child_size[0]);
            height += child_size[1];
            if i + 1 < count {
                height += self.spacing;
            }
        }
        [width, height]
    }

    fn arrange(&mut self, position: [f32; 2], _size: [f32; 2], ui: &mut Ui) {
        // _size (the parent-given size) is ignored — this container
        // always sizes itself to its own content via taffy, using the
        // sizes measure() already cached this frame. No child.measure()
        // calls happen here at all.
        let mut tree: TaffyTree<()> = TaffyTree::new();

        let child_node_ids: Vec<NodeId> = self
            .cached_child_sizes
            .iter()
            .map(|size| {
                tree.new_leaf(Style {
                    size: Size {
                        width: length(size[0]),
                        height: length(size[1]),
                    },
                    ..Default::default()
                })
                .unwrap()
            })
            .collect();

        let column_node = tree
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Column,
                    align_items: Some(to_taffy_align(self.align)),
                    gap: Size {
                        width: length(0.0),
                        height: length(self.spacing),
                    },
                    ..Default::default()
                },
                &child_node_ids,
            )
            .unwrap();

        tree.compute_layout(column_node, Size::MAX_CONTENT).unwrap();

        for (child, node_id) in self.children.iter_mut().zip(child_node_ids.iter()) {
            let layout = tree.layout(*node_id).unwrap();
            let child_position = [
                position[0] + layout.location.x,
                position[1] + layout.location.y,
            ];
            let child_size = [layout.size.width, layout.size.height];
            child.arrange(child_position, child_size, ui);
        }
    }
}

/// A horizontal stack, analogous to `Column`.
pub struct Row<'a> {
    spacing: f32,
    align: Alignment,
    children: Vec<Box<dyn AnyWidget + 'a>>,
    cached_child_sizes: Vec<[f32; 2]>,
}

impl<'a> Row<'a> {
    pub fn new(children: Vec<Box<dyn AnyWidget + 'a>>) -> Self {
        let cached_child_sizes = vec![[0.0; 2]; children.len()];
        Row {
            spacing: 8.0,
            align: Alignment::Center,
            children,
            cached_child_sizes,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.children
            .get_mut(index)?
            .as_any_mut()?
            .downcast_mut::<T>()
    }

    pub fn arrange_at(&mut self, position: [f32; 2], ui: &mut Ui) {
        let size = Measurable::measure(self, ui);
        Measurable::arrange(self, position, size, ui);
    }
}

impl<'a> Widget for Row<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        self.arrange_at([0.0, 0.0], ui);
    }
}

impl<'a> Measurable for Row<'a> {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        self.cached_child_sizes.clear();
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let count = self.children.len();
        for (i, child) in self.children.iter_mut().enumerate() {
            let child_size = child.measure(ui);
            self.cached_child_sizes.push(child_size);
            width += child_size[0];
            height = height.max(child_size[1]);
            if i + 1 < count {
                width += self.spacing;
            }
        }
        [width, height]
    }

    fn arrange(&mut self, position: [f32; 2], _size: [f32; 2], ui: &mut Ui) {
        let mut tree: TaffyTree<()> = TaffyTree::new();

        let child_node_ids: Vec<NodeId> = self
            .cached_child_sizes
            .iter()
            .map(|size| {
                tree.new_leaf(Style {
                    size: Size {
                        width: length(size[0]),
                        height: length(size[1]),
                    },
                    ..Default::default()
                })
                .unwrap()
            })
            .collect();

        let row_node = tree
            .new_with_children(
                Style {
                    flex_direction: FlexDirection::Row,
                    align_items: Some(to_taffy_align(self.align)),
                    gap: Size {
                        width: length(self.spacing),
                        height: length(0.0),
                    },
                    ..Default::default()
                },
                &child_node_ids,
            )
            .unwrap();

        tree.compute_layout(row_node, Size::MAX_CONTENT).unwrap();

        for (child, node_id) in self.children.iter_mut().zip(child_node_ids.iter()) {
            let layout = tree.layout(*node_id).unwrap();
            let child_position = [
                position[0] + layout.location.x,
                position[1] + layout.location.y,
            ];
            let child_size = [layout.size.width, layout.size.height];
            child.arrange(child_position, child_size, ui);
        }
    }
}
