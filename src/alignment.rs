use taffy::prelude::AlignItems;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub fn to_taffy_align(align: Alignment) -> AlignItems {
    match align {
        Alignment::Start => AlignItems::START,
        Alignment::Center => AlignItems::CENTER,
        Alignment::End => AlignItems::END,
    }
}
