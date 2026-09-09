use crate::alignment::{Alignment, to_taffy_align};
use crate::ui::Ui;
use crate::widget::{AnyWidget, Measurable, Widget};
use taffy::prelude::*;

#[allow(clippy::too_many_arguments)]
fn arrange_children(
    children: &mut [Box<dyn AnyWidget + '_>],
    cached_sizes: &[[f32; 2]],
    position: [f32; 2],
    size: [f32; 2],
    padding: [f32; 2],
    spacing: f32,
    align: Alignment,
    direction: FlexDirection,
    ui: &mut Ui,
) {
    let inner_position = [position[0] + padding[0], position[1] + padding[1]];
    let inner_size = [
        (size[0] - padding[0] * 2.0).max(0.0),
        (size[1] - padding[1] * 2.0).max(0.0),
    ];

    let mut tree: TaffyTree<()> = TaffyTree::new();

    let child_node_ids: Vec<NodeId> = cached_sizes
        .iter()
        .map(|s| {
            tree.new_leaf(Style {
                size: Size {
                    width: length(s[0]),
                    height: length(s[1]),
                },
                ..Default::default()
            })
            .unwrap()
        })
        .collect();

    let gap = match direction {
        FlexDirection::Column => Size {
            width: length(0.0),
            height: length(spacing),
        },
        _ => Size {
            width: length(spacing),
            height: length(0.0),
        },
    };

    let container = tree
        .new_with_children(
            Style {
                size: Size {
                    width: length(inner_size[0]),
                    height: length(inner_size[1]),
                },
                flex_direction: direction,
                align_items: Some(to_taffy_align(align)),
                gap,
                ..Default::default()
            },
            &child_node_ids,
        )
        .unwrap();

    tree.compute_layout(container, Size::MAX_CONTENT).unwrap();

    for (child, node_id) in children.iter_mut().zip(child_node_ids.iter()) {
        let layout = tree.layout(*node_id).unwrap();
        let child_position = [
            inner_position[0] + layout.location.x,
            inner_position[1] + layout.location.y,
        ];
        let child_size = [layout.size.width, layout.size.height];
        child.arrange(child_position, child_size, ui);
    }
}

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
    width: Option<f32>,
    height: Option<f32>,
    padding: [f32; 2],
    children: Vec<Box<dyn AnyWidget + 'a>>,
    cached_child_sizes: Vec<[f32; 2]>,
}

impl<'a> Column<'a> {
    pub fn new(children: Vec<Box<dyn AnyWidget + 'a>>) -> Self {
        let cached_child_sizes = vec![[0.0; 2]; children.len()];
        Column {
            spacing: 8.0,
            align: Alignment::Center,
            width: None,
            height: None,
            padding: [0.0, 0.0],
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

    /// Overrides the column's own width — by default it hugs its content.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the column's own height — by default it hugs its content.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Shorthand for `.width(size[0]).height(size[1])`.
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
    }

    /// Insets children from the column's own bounds on every side.
    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
        self
    }

    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.children
            .get_mut(index)?
            .as_any_mut()?
            .downcast_mut::<T>()
    }

    /// Measure then arrange in one call, at the given origin.
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
        [
            self.width.unwrap_or(width + self.padding[0] * 2.0),
            self.height.unwrap_or(height + self.padding[1] * 2.0),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        arrange_children(
            &mut self.children,
            &self.cached_child_sizes,
            position,
            size,
            self.padding,
            self.spacing,
            self.align,
            FlexDirection::Column,
            ui,
        );
    }
}

/// A horizontal stack, analogous to `Column`.
pub struct Row<'a> {
    spacing: f32,
    align: Alignment,
    width: Option<f32>,
    height: Option<f32>,
    padding: [f32; 2],
    children: Vec<Box<dyn AnyWidget + 'a>>,
    cached_child_sizes: Vec<[f32; 2]>,
}

impl<'a> Row<'a> {
    pub fn new(children: Vec<Box<dyn AnyWidget + 'a>>) -> Self {
        let cached_child_sizes = vec![[0.0; 2]; children.len()];
        Row {
            spacing: 8.0,
            align: Alignment::Center,
            width: None,
            height: None,
            padding: [0.0, 0.0],
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

    /// Overrides the row's own width — by default it hugs its content.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the row's own height — by default it hugs its content.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Shorthand for `.width(size[0]).height(size[1])`.
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
    }

    /// Insets children from the row's own bounds on every side.
    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = padding;
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
        [
            self.width.unwrap_or(width + self.padding[0] * 2.0),
            self.height.unwrap_or(height + self.padding[1] * 2.0),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        arrange_children(
            &mut self.children,
            &self.cached_child_sizes,
            position,
            size,
            self.padding,
            self.spacing,
            self.align,
            FlexDirection::Row,
            ui,
        );
    }
}
