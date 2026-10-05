use crate::alignment::Alignment;

// miscellanous stuff, that was abandoned, and didn't fit into anywhere.
// Tragic.

pub fn center_text_in(
    position: [f32; 2],
    size: [f32; 2],
    text_width: f32,
    line_height: f32,
) -> [f32; 2] {
    [
        position[0] + (size[0] - text_width) / 2.0,
        position[1] + (size[1] - line_height) / 2.0,
    ]
}

pub fn align_text_in(
    position: [f32; 2],
    size: [f32; 2],
    text_width: f32,
    line_height: f32,
    padding_x: f32,
    align: Alignment,
) -> [f32; 2] {
    let x = match align {
        Alignment::Start | Alignment::Stretch => position[0] + padding_x,
        Alignment::Center => position[0] + (size[0] - text_width) / 2.0,
        Alignment::End => position[0] + size[0] - padding_x - text_width,
    };
    [x, position[1] + (size[1] - line_height) / 2.0]
}

pub fn scale_rect(position: [f32; 2], size: [f32; 2], scale: f32) -> ([f32; 2], [f32; 2]) {
    let scaled = [size[0] * scale, size[1] * scale];
    (
        [
            position[0] - (scaled[0] - size[0]) / 2.0,
            position[1] - (scaled[1] - size[1]) / 2.0,
        ],
        scaled,
    )
}
